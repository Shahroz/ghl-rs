//! Tests for the generated typed services (see `xtask/generate_services.py`).

use ghl_sdk::services::invoices::ListInvoicesParams;
use ghl_sdk::Ghl;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn client(server: &MockServer) -> Ghl {
    Ghl::builder()
        .base_url(server.uri())
        .private_integration_token("pit-test")
        .build()
        .expect("client builds")
}

#[tokio::test]
async fn params_builder_sends_required_and_optional_query_args() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/invoices/"))
        .and(query_param("altId", "loc-1"))
        .and(query_param("altType", "location"))
        .and(query_param("limit", "10"))
        .and(query_param("offset", "0"))
        .and(query_param("status", "draft"))
        .and(header("Version", "2021-07-28"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "invoices": [{ "_id": "inv-1", "name": "August" }],
            "total": 1
        })))
        .expect(1)
        .mount(&server)
        .await;

    let params = ListInvoicesParams::new("loc-1", "location", "10", "0").status("draft");
    let out = client(&server)
        .invoices()
        .list_invoices(&params)
        .await
        .expect("typed call succeeds");

    assert_eq!(out.total, Some(1.0));
    assert_eq!(out.invoices.len(), 1);
}

#[tokio::test]
async fn omitted_optional_params_are_not_sent() {
    let server = MockServer::start().await;
    // `status` is absent from the builder, so it must not appear on the wire.
    Mock::given(method("GET"))
        .and(path("/invoices/"))
        .and(query_param("altId", "loc-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "invoices": [], "total": 0
        })))
        .expect(1)
        .mount(&server)
        .await;

    let params = ListInvoicesParams::new("loc-1", "location", "5", "0");
    let q = client(&server).invoices().list_invoices(&params).await;
    assert!(q.is_ok());
    // Only the four required params were serialized.
    let received = &server.received_requests().await.unwrap()[0];
    let url = received.url.to_string();
    assert!(!url.contains("status="), "unexpected status param in {url}");
    assert!(url.contains("altType=location"), "missing altType in {url}");
}

#[tokio::test]
async fn path_params_are_percent_encoded() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/invoices/inv%2F42"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "_id": "inv/42" })))
        .expect(1)
        .mount(&server)
        .await;

    // A slash in an id must not escape its path segment.
    let out = client(&server)
        .invoices()
        .get_invoice(
            "inv/42",
            &ghl_sdk::services::invoices::GetInvoiceParams::new("loc-1", "location"),
        )
        .await;
    assert!(out.is_ok(), "encoded path should resolve: {out:?}");
}

#[cfg(feature = "contacts")]
#[tokio::test]
async fn generated_methods_coexist_with_handwritten_ones() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/contacts/c-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "contact": { "id": "c-1", "email": "a@b.co" }
        })))
        .mount(&server)
        .await;

    let ghl = client(&server);
    // Hand-written ergonomic helper: unwraps the envelope, returns Contact.
    let typed = ghl.contacts().get("c-1").await.expect("handwritten get");
    assert_eq!(typed.email.as_deref(), Some("a@b.co"));

    // Generated method for the same endpoint, returning the spec's response DTO.
    let generated = ghl.contacts().get_contact("c-1").await;
    assert!(generated.is_ok(), "generated get_contact works too");
}
