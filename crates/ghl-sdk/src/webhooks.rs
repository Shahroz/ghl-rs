//! Webhook signature verification and typed events.
//!
//! GoHighLevel signs every webhook with **RSA-SHA256** (PKCS#1 v1.5) over the
//! raw request body and sends the base64 signature in the `x-wh-signature`
//! header. [`verify`] checks it against HighLevel's published public key.
//!
//! Requires the `webhooks` cargo feature.
//!
//! # Verify, then parse
//!
//! Always verify the **raw bytes** you received. Re-serializing parsed JSON can
//! reorder keys or change spacing, which invalidates the signature.
//!
//! ```no_run
//! use ghl_sdk::webhooks::{self, WebhookEvent};
//!
//! # fn handler(raw_body: &[u8], signature: &str) -> Result<(), Box<dyn std::error::Error>> {
//! webhooks::verify(raw_body, signature)?;
//! let event: WebhookEvent = serde_json::from_slice(raw_body)?;
//!
//! // Guard against replays: reject stale timestamps and ids you've already seen.
//! if event.is_stale(std::time::Duration::from_secs(300)) {
//!     return Ok(()); // too old — drop it
//! }
//!
//! match event.event_type() {
//!     "ContactCreate" => println!("new contact in {:?}", event.location_id),
//!     "InvoicePaid" => println!("paid!"),
//!     other => println!("unhandled event {other}"),
//! }
//! # Ok(()) }
//! ```
//!
//! # Axum example
//!
//! ```ignore
//! async fn ghl_webhook(headers: HeaderMap, body: axum::body::Bytes) -> StatusCode {
//!     let Some(sig) = headers.get("x-wh-signature").and_then(|v| v.to_str().ok()) else {
//!         return StatusCode::BAD_REQUEST;
//!     };
//!     if ghl_sdk::webhooks::verify(&body, sig).is_err() {
//!         return StatusCode::UNAUTHORIZED;
//!     }
//!     // …parse and handle…
//!     StatusCode::OK
//! }
//! ```

use std::time::{Duration, SystemTime};

use base64::Engine as _;
use rsa::pkcs8::DecodePublicKey;
use rsa::signature::Verifier;
use serde::{Deserialize, Serialize};

/// The header carrying the base64 RSA signature.
pub const SIGNATURE_HEADER: &str = "x-wh-signature";

/// HighLevel's webhook-signing public key, as published in their
/// [webhook authentication guide][guide].
///
/// HighLevel rotates this key occasionally and announces it by email and in the
/// developer Slack. If verification starts failing across the board, check for a
/// rotation notice and use [`verify_with_key`] with the new key until this crate
/// ships an update.
///
/// [guide]: https://marketplace.gohighlevel.com/docs/oauth/WebhookAuthentication
pub const PUBLIC_KEY_PEM: &str = "-----BEGIN PUBLIC KEY-----
MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAokvo/r9tVgcfZ5DysOSC
Frm602qYV0MaAiNnX9O8KxMbiyRKWeL9JpCpVpt4XHIcBOK4u3cLSqJGOLaPuXw6
dO0t6Q/ZVdAV5Phz+ZtzPL16iCGeK9po6D6JHBpbi989mmzMryUnQJezlYJ3DVfB
csedpinheNnyYeFXolrJvcsjDtfAeRx5ByHQmTnSdFUzuAnC9/GepgLT9SM4nCpv
uxmZMxrJt5Rw+VUaQ9B8JSvbMPpez4peKaJPZHBbU3OdeCVx5klVXXZQGNHOs8gF
3kvoV5rTnXV0IknLBXlcKKAQLZcY/Q9rG6Ifi9c+5vqlvHPCUJFT5XUGG5RKgOKU
J062fRtN+rLYZUV+BjafxQauvC8wSWeYja63VSUruvmNj8xkx2zE/Juc+yjLjTXp
IocmaiFeAO6fUtNjDeFVkhf5LNb59vECyrHD2SQIrhgXpO4Q3dVNA5rw576PwTzN
h/AMfHKIjE4xQA1SZuYJmNnmVZLIZBlQAF9Ntd03rfadZ+yDiOXCCs9FkHibELhC
HULgCsnuDJHcrGNd5/Ddm5hxGQ0ASitgHeMZ0kcIOwKDOzOU53lDza6/Y09T7sYJ
PQe7z0cvj7aE4B+Ax1ZoZGPzpJlZtGXCsu9aTEGEnKzmsFqwcSsnw3JB31IGKAyk
T1hhTiaCeIY/OwwwNUY2yvcCAwEAAQ==
-----END PUBLIC KEY-----";

/// Why a webhook failed verification.
#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum WebhookError {
    /// The `x-wh-signature` header wasn't valid base64.
    #[error("signature is not valid base64: {0}")]
    Signature(#[from] base64::DecodeError),

    /// The configured public key couldn't be parsed.
    #[error("public key is not a valid PEM SPKI key: {0}")]
    PublicKey(String),

    /// The signature did not match the body. Treat the request as hostile.
    #[error("signature does not match the payload")]
    Mismatch,
}

/// Verify a webhook body against HighLevel's published public key.
///
/// Pass the **raw** bytes exactly as received — see the module docs.
pub fn verify(raw_body: &[u8], signature_b64: &str) -> Result<(), WebhookError> {
    verify_with_key(raw_body, signature_b64, PUBLIC_KEY_PEM)
}

/// Verify against a specific PEM public key.
///
/// Use this if HighLevel rotates its key before this crate is updated, or to
/// test with a key of your own.
pub fn verify_with_key(
    raw_body: &[u8],
    signature_b64: &str,
    public_key_pem: &str,
) -> Result<(), WebhookError> {
    // The header may arrive with incidental whitespace or newlines.
    let cleaned: String = signature_b64.split_whitespace().collect();
    let signature_bytes = base64::engine::general_purpose::STANDARD.decode(cleaned)?;

    let key = rsa::RsaPublicKey::from_public_key_pem(public_key_pem)
        .map_err(|e| WebhookError::PublicKey(e.to_string()))?;
    let verifying_key = rsa::pkcs1v15::VerifyingKey::<sha2::Sha256>::new(key);
    let signature = rsa::pkcs1v15::Signature::try_from(signature_bytes.as_slice())
        .map_err(|_| WebhookError::Mismatch)?;

    verifying_key
        .verify(raw_body, &signature)
        .map_err(|_| WebhookError::Mismatch)
}

/// A GoHighLevel webhook payload.
///
/// HighLevel sends 58 event types that share an envelope (`type`, `timestamp`,
/// `webhookId`, and usually `locationId`) but differ in their remaining fields.
/// The common envelope is typed here; everything else stays in [`Self::data`],
/// so a new upstream event never breaks your handler.
///
/// Deserialize the event, then reach into `data` for the specifics — or
/// re-deserialize the whole body into a DTO from
/// [`ghl-models`](https://docs.rs/ghl-models) once you know the type.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WebhookEvent {
    /// Event name, e.g. `ContactCreate`, `InvoicePaid`, `AppointmentUpdate`.
    #[serde(default, rename = "type")]
    pub event_type: Option<String>,
    /// When HighLevel emitted the event (ISO-8601).
    #[serde(default)]
    pub timestamp: Option<String>,
    /// Unique id for this delivery — persist it to reject replays.
    #[serde(default)]
    pub webhook_id: Option<String>,
    /// Sub-account the event belongs to, when it is location-scoped.
    #[serde(default)]
    pub location_id: Option<String>,
    /// Agency the event belongs to, on agency-level events.
    #[serde(default)]
    pub company_id: Option<String>,
    /// Every other field in the payload, event-specific.
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

impl WebhookEvent {
    /// The event name, or `""` when absent.
    pub fn event_type(&self) -> &str {
        self.event_type.as_deref().unwrap_or_default()
    }

    /// Parse [`Self::timestamp`] into a [`SystemTime`].
    ///
    /// Returns `None` when it is missing or not RFC-3339.
    pub fn timestamp_at(&self) -> Option<SystemTime> {
        let raw = self.timestamp.as_deref()?;
        parse_rfc3339(raw)
    }

    /// Whether the event is older than `max_age` — the replay-window check
    /// HighLevel recommends (they suggest 5 minutes).
    ///
    /// An unparseable or missing timestamp counts as stale: a payload you can't
    /// date is one you can't bound.
    pub fn is_stale(&self, max_age: Duration) -> bool {
        match self.timestamp_at() {
            Some(t) => SystemTime::now()
                .duration_since(t)
                .map(|age| age > max_age)
                .unwrap_or(false), // clock skew into the future — not stale
            None => true,
        }
    }

    /// Re-deserialize the whole payload into a concrete type, e.g. one of the
    /// `ghl-models` DTOs.
    pub fn parse_as<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_value(serde_json::to_value(self)?)
    }
}

/// Minimal RFC-3339 parser for the `timestamp` field.
///
/// Avoids pulling a date-time crate into the dependency tree for one field;
/// only the shape HighLevel actually sends is supported
/// (`2026-01-28T14:35:00Z`, optionally fractional, optionally `±HH:MM`).
fn parse_rfc3339(s: &str) -> Option<SystemTime> {
    let bytes = s.as_bytes();
    if bytes.len() < 19 {
        return None;
    }
    let num = |a: usize, b: usize| s.get(a..b)?.parse::<i64>().ok();
    let (y, mo, d) = (num(0, 4)?, num(5, 7)?, num(8, 10)?);
    let (h, mi, sec) = (num(11, 13)?, num(14, 16)?, num(17, 19)?);
    if !(1..=12).contains(&mo) || !(1..=31).contains(&d) {
        return None;
    }

    // Offset suffix, if any.
    let rest = &s[19..];
    let mut offset_secs: i64 = 0;
    let tz = rest.trim_start_matches(|c: char| c == '.' || c.is_ascii_digit());
    if !(tz.is_empty() || tz == "Z" || tz == "z") {
        let sign = match tz.as_bytes()[0] {
            b'+' => 1,
            b'-' => -1,
            _ => return None,
        };
        let oh = tz.get(1..3)?.parse::<i64>().ok()?;
        let om = tz
            .get(4..6)
            .and_then(|v| v.parse::<i64>().ok())
            .unwrap_or(0);
        offset_secs = sign * (oh * 3600 + om * 60);
    }

    // Days since the Unix epoch (proleptic Gregorian, Howard Hinnant's algorithm).
    let y_adj = if mo <= 2 { y - 1 } else { y };
    let era = if y_adj >= 0 { y_adj } else { y_adj - 399 } / 400;
    let yoe = y_adj - era * 400;
    let mp = (mo + 9) % 12;
    let doy = (153 * mp + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;

    let secs = days * 86_400 + h * 3600 + mi * 60 + sec - offset_secs;
    if secs < 0 {
        return None;
    }
    Some(SystemTime::UNIX_EPOCH + Duration::from_secs(secs as u64))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rejects_a_tampered_body() {
        // Not a real signature, so verification must fail rather than pass.
        let err = verify(b"{\"type\":\"ContactCreate\"}", "aGVsbG8=").unwrap_err();
        assert!(matches!(err, WebhookError::Mismatch), "got {err:?}");
    }

    #[test]
    fn rejects_non_base64_signature() {
        let err = verify(b"{}", "not base64!!!").unwrap_err();
        assert!(matches!(err, WebhookError::Signature(_)), "got {err:?}");
    }

    #[test]
    fn published_public_key_parses() {
        assert!(rsa::RsaPublicKey::from_public_key_pem(PUBLIC_KEY_PEM).is_ok());
    }

    #[test]
    fn round_trips_a_generated_signature() {
        use rsa::pkcs1v15::SigningKey;
        use rsa::pkcs8::EncodePublicKey;
        use rsa::signature::{SignatureEncoding, Signer};

        // A throwaway key stands in for HighLevel's, proving the verify path.
        let mut rng = rand::thread_rng();
        let private = rsa::RsaPrivateKey::new(&mut rng, 2048).expect("keygen");
        let pem = private
            .to_public_key()
            .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
            .expect("pem");

        let body = br#"{"type":"ContactCreate","webhookId":"abc"}"#;
        let sig = SigningKey::<sha2::Sha256>::new(private).sign(body);
        let sig_b64 = base64::engine::general_purpose::STANDARD.encode(sig.to_bytes());

        verify_with_key(body, &sig_b64, &pem).expect("valid signature verifies");
        verify_with_key(b"tampered", &sig_b64, &pem).expect_err("tampered body rejected");
    }

    #[test]
    fn parses_envelope_and_keeps_unknown_fields() {
        let raw = r#"{
            "type": "ContactCreate",
            "timestamp": "2026-01-28T14:35:00Z",
            "webhookId": "abc123",
            "locationId": "loc-1",
            "id": "contact-9",
            "email": "ada@example.com"
        }"#;
        let ev: WebhookEvent = serde_json::from_str(raw).unwrap();
        assert_eq!(ev.event_type(), "ContactCreate");
        assert_eq!(ev.webhook_id.as_deref(), Some("abc123"));
        assert_eq!(ev.location_id.as_deref(), Some("loc-1"));
        // Event-specific fields survive in `data`.
        assert_eq!(ev.data["email"], "ada@example.com");
        assert_eq!(ev.data["id"], "contact-9");
    }

    #[test]
    fn timestamps_drive_the_replay_window() {
        let old: WebhookEvent =
            serde_json::from_str(r#"{"timestamp":"2020-01-01T00:00:00Z"}"#).unwrap();
        assert!(old.is_stale(Duration::from_secs(300)));
        assert!(old.timestamp_at().is_some());

        // A missing timestamp can't be bounded, so it counts as stale.
        let none: WebhookEvent = serde_json::from_str("{}").unwrap();
        assert!(none.is_stale(Duration::from_secs(300)));

        // Offsets are honored.
        let with_offset = parse_rfc3339("2026-01-28T14:35:00+05:00").expect("offset form parses");
        let as_utc = parse_rfc3339("2026-01-28T09:35:00Z").expect("utc form parses");
        assert_eq!(with_offset, as_utc);
    }

    #[test]
    fn fractional_seconds_parse() {
        assert!(parse_rfc3339("2026-01-28T14:35:00.123Z").is_some());
        assert!(parse_rfc3339("garbage").is_none());
        assert!(parse_rfc3339("2026-13-28T14:35:00Z").is_none()); // month 13
    }
}
