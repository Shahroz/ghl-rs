//! Calendars API: calendars, free slots, and appointments.

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Ghl;
use crate::error::Result;

/// A bookable calendar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Calendar {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_type: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_active: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot_duration: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// An appointment on a calendar.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct Appointment {
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub calendar_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contact_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_time: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    /// `confirmed`, `cancelled`, `showed`, `noshow`, `invalid`, …
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub appointment_status: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    /// Any fields this SDK doesn't model yet.
    #[serde(flatten)]
    pub extra: serde_json::Map<String, serde_json::Value>,
}

/// Payload for [`CalendarsService::create_appointment`].
///
/// `calendar_id`, `location_id`, `contact_id`, and `start_time` are required by
/// the API; `start_time` is ISO-8601 with offset, e.g. `2026-08-01T14:00:00+05:00`.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
#[allow(missing_docs)] // fields mirror the API wire format 1:1
pub struct CreateAppointment {
    pub calendar_id: String,
    pub location_id: String,
    pub contact_id: String,
    pub start_time: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appointment_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assigned_user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Notify the contact and assigned user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_notify: Option<bool>,
    /// Book even if the slot isn't in the calendar's availability.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore_free_slot_validation: Option<bool>,
}

#[derive(Deserialize)]
struct CalendarList {
    #[serde(default)]
    calendars: Vec<Calendar>,
}

/// Free slots grouped by date, as returned by the API.
///
/// The API keys slots by date (`"2026-08-01": { "slots": [...] }`), so this is
/// kept as a flexible map rather than a fixed struct.
#[derive(Debug, Clone, Default)]
pub struct FreeSlots {
    /// Date (`YYYY-MM-DD`) → ISO-8601 slot start times.
    pub by_date: Vec<(String, Vec<String>)>,
}

impl FreeSlots {
    /// Every slot across all dates, flattened.
    pub fn all(&self) -> Vec<&str> {
        self.by_date
            .iter()
            .flat_map(|(_, slots)| slots.iter().map(String::as_str))
            .collect()
    }
}

/// Access to the Calendars API. Obtained via [`Ghl::calendars`].
pub struct CalendarsService {
    client: Ghl,
}

impl CalendarsService {
    pub(crate) fn new(client: Ghl) -> Self {
        Self { client }
    }

    /// `GET /calendars/` — calendars in a location.
    pub async fn list(&self, location_id: &str) -> Result<Vec<Calendar>> {
        let list: CalendarList = self
            .client
            .send(
                Method::GET,
                "/calendars/",
                &[("locationId".into(), location_id.to_owned())],
                None::<&()>,
            )
            .await?;
        Ok(list.calendars)
    }

    /// `GET /calendars/{id}/free-slots` — bookable slots in a date range.
    ///
    /// `start_date` and `end_date` are epoch milliseconds, matching the API.
    pub async fn free_slots(
        &self,
        calendar_id: &str,
        start_date: i64,
        end_date: i64,
        timezone: Option<&str>,
    ) -> Result<FreeSlots> {
        let mut query: Vec<(String, String)> = vec![
            ("startDate".into(), start_date.to_string()),
            ("endDate".into(), end_date.to_string()),
        ];
        if let Some(tz) = timezone {
            query.push(("timezone".into(), tz.to_owned()));
        }
        let raw: serde_json::Value = self
            .client
            .send(
                Method::GET,
                &format!("/calendars/{calendar_id}/free-slots"),
                &query,
                None::<&()>,
            )
            .await?;

        // Response shape: { "2026-08-01": { "slots": [...] }, "traceId": "..." }
        let mut by_date = Vec::new();
        if let Some(map) = raw.as_object() {
            for (key, value) in map {
                let Some(slots) = value.get("slots").and_then(|s| s.as_array()) else {
                    continue; // skips traceId and other non-date keys
                };
                by_date.push((
                    key.clone(),
                    slots
                        .iter()
                        .filter_map(|s| s.as_str().map(str::to_owned))
                        .collect(),
                ));
            }
        }
        by_date.sort_by(|a, b| a.0.cmp(&b.0));
        Ok(FreeSlots { by_date })
    }

    /// `POST /calendars/events/appointments` — book an appointment.
    pub async fn create_appointment(&self, appointment: CreateAppointment) -> Result<Appointment> {
        self.client
            .send(
                Method::POST,
                "/calendars/events/appointments",
                &[],
                Some(&appointment),
            )
            .await
    }

    /// `GET /calendars/events/appointments/{id}` — fetch one appointment.
    pub async fn get_appointment(&self, event_id: &str) -> Result<Appointment> {
        #[derive(Deserialize)]
        struct Envelope {
            event: Appointment,
        }
        let envelope: Envelope = self
            .client
            .send(
                Method::GET,
                &format!("/calendars/events/appointments/{event_id}"),
                &[],
                None::<&()>,
            )
            .await?;
        Ok(envelope.event)
    }
}
