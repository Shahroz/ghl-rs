//! Integration tests against a wiremock server.

use futures_util::TryStreamExt;
use ghl_sdk::contacts::CreateContact;
use ghl_sdk::Ghl;
use serde_json::json;
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, Request, Respond, ResponseTemplate};

fn client(server: &MockServer) -> Ghl {
    Ghl::builder()
        .base_url(server.uri())
        .private_integration_token("pit-test-token")
        .max_retries(2)
        .build()
        .expect("client builds")
}

#[tokio::test]
async fn create_contact_sends_auth_and_version_headers() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/contacts/"))
        .and(header("Authorization", "Bearer pit-test-token"))
        .and(header("Version", "2021-07-28"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "contact": { "id": "c-1", "locationId": "loc-1", "email": "ada@example.com" }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let contact = client(&server)
        .contacts()
        .create(CreateContact {
            location_id: "loc-1".into(),
            email: Some("ada@example.com".into()),
            ..Default::default()
        })
        .await
        .expect("create succeeds");

    assert_eq!(contact.id, "c-1");
    assert_eq!(contact.email.as_deref(), Some("ada@example.com"));
}

#[tokio::test]
async fn retries_on_429_then_succeeds() {
    struct FailOnceThenOk;
    impl Respond for FailOnceThenOk {
        fn respond(&self, _req: &Request) -> ResponseTemplate {
            use std::sync::atomic::{AtomicU32, Ordering};
            static CALLS: AtomicU32 = AtomicU32::new(0);
            if CALLS.fetch_add(1, Ordering::SeqCst) == 0 {
                ResponseTemplate::new(429).insert_header("Retry-After", "0")
            } else {
                ResponseTemplate::new(200).set_body_json(json!({
                    "contact": { "id": "c-2" }
                }))
            }
        }
    }

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/contacts/c-2"))
        .respond_with(FailOnceThenOk)
        .expect(2)
        .mount(&server)
        .await;

    let contact = client(&server)
        .contacts()
        .get("c-2")
        .await
        .expect("retry succeeds");
    assert_eq!(contact.id, "c-2");
}

#[tokio::test]
async fn surfaces_api_error_message() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/contacts/missing"))
        .respond_with(ResponseTemplate::new(404).set_body_json(json!({
            "message": "Contact not found"
        })))
        .mount(&server)
        .await;

    let err = client(&server).contacts().get("missing").await.unwrap_err();
    match err {
        ghl_sdk::Error::Api {
            status, message, ..
        } => {
            assert_eq!(status.as_u16(), 404);
            assert_eq!(message, "Contact not found");
        }
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn stream_follows_cursor_pagination() {
    let server = MockServer::start().await;

    // Page 1: full page (limit 2) with a cursor.
    Mock::given(method("GET"))
        .and(path("/contacts/"))
        .and(query_param("locationId", "loc-1"))
        .and(query_param("limit", "2"))
        .and(query_param("startAfterId", "c-b"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "contacts": [{ "id": "c-c" }],
            "meta": {}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/contacts/"))
        .and(query_param("locationId", "loc-1"))
        .and(query_param("limit", "2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "contacts": [{ "id": "c-a" }, { "id": "c-b" }],
            "meta": { "startAfterId": "c-b", "startAfter": 1700000000000i64 }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let ids: Vec<String> = client(&server)
        .contacts()
        .list("loc-1")
        .limit(2)
        .stream()
        .map_ok(|c| c.id)
        .try_collect()
        .await
        .expect("stream completes");

    assert_eq!(ids, vec!["c-a", "c-b", "c-c"]);
}

#[tokio::test]
async fn rate_status_reflects_headers() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/contacts/c-9"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("X-RateLimit-Remaining", "42")
                .insert_header("X-RateLimit-Daily-Remaining", "199000")
                .set_body_json(json!({ "contact": { "id": "c-9" } })),
        )
        .mount(&server)
        .await;

    let ghl = client(&server);
    ghl.contacts().get("c-9").await.expect("request ok");
    let status = ghl.rate_status();
    assert_eq!(status.burst_remaining, Some(42));
    assert_eq!(status.daily_remaining, Some(199000));
}

#[tokio::test]
async fn opportunity_search_uses_snake_case_params_and_paginates() {
    use ghl_sdk::opportunities::Opportunity;

    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/opportunities/search"))
        .and(query_param("location_id", "loc-1"))
        .and(query_param("limit", "2"))
        .and(query_param("status", "open"))
        .and(query_param("startAfterId", "o-b"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "opportunities": [{ "id": "o-c", "name": "Deal C" }],
            "meta": {}
        })))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(method("GET"))
        .and(path("/opportunities/search"))
        .and(query_param("location_id", "loc-1"))
        .and(query_param("limit", "2"))
        .and(query_param("status", "open"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "opportunities": [
                { "id": "o-a", "monetaryValue": 1500.5 },
                { "id": "o-b" }
            ],
            "meta": { "startAfterId": "o-b" }
        })))
        .expect(1)
        .mount(&server)
        .await;

    let all: Vec<Opportunity> = client(&server)
        .opportunities()
        .search("loc-1")
        .limit(2)
        .status("open")
        .stream()
        .try_collect()
        .await
        .expect("stream completes");

    assert_eq!(
        all.iter().map(|o| o.id.as_str()).collect::<Vec<_>>(),
        vec!["o-a", "o-b", "o-c"]
    );
    assert_eq!(all[0].monetary_value, Some(1500.5));
}

#[tokio::test]
async fn pipelines_and_opportunity_create_roundtrip() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/opportunities/pipelines"))
        .and(query_param("locationId", "loc-1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "pipelines": [{
                "id": "pipe-1",
                "name": "Sales",
                "stages": [
                    { "id": "st-1", "name": "New", "position": 0 },
                    { "id": "st-2", "name": "Won", "position": 1 }
                ]
            }]
        })))
        .mount(&server)
        .await;
    Mock::given(method("POST"))
        .and(path("/opportunities/"))
        .respond_with(ResponseTemplate::new(201).set_body_json(json!({
            "opportunity": {
                "id": "o-1", "name": "Acme deal",
                "pipelineId": "pipe-1", "pipelineStageId": "st-1", "status": "open"
            }
        })))
        .mount(&server)
        .await;
    Mock::given(method("PUT"))
        .and(path("/opportunities/o-1/status"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({ "succeded": true })))
        .mount(&server)
        .await;

    let ghl = client(&server);
    let pipelines = ghl
        .opportunities()
        .pipelines("loc-1")
        .await
        .expect("pipelines");
    assert_eq!(pipelines[0].stages.len(), 2);

    let created = ghl
        .opportunities()
        .create(ghl_sdk::opportunities::CreateOpportunity {
            location_id: "loc-1".into(),
            pipeline_id: pipelines[0].id.clone(),
            name: "Acme deal".into(),
            ..Default::default()
        })
        .await
        .expect("create");
    assert_eq!(created.id, "o-1");
    assert_eq!(created.status.as_deref(), Some("open"));

    ghl.opportunities()
        .update_status("o-1", "won")
        .await
        .expect("status update");
}
