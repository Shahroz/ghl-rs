# GoHighLevel Rust SDK + MCP Server — Research & Design Proposal

**Status:** Draft v1 · **Author:** Shahroz Allauddin · **Date:** 2026-07-29
**Working names:** `ghl-sdk` (library) + `ghl-mcp` (server) — both names verified available on crates.io as of today

---

## 1. Executive Summary

GoHighLevel (GHL) is a fast-growing, PE-backed all-in-one CRM platform claiming **~60–70k paying agencies and 2.2M downstream businesses**. Its official developer stack is young: a TypeScript SDK (v3.0.0, ~52k npm downloads/month), near-dead official Python/PHP libraries, and **no official SDK for Rust, Go, Java, or Ruby**. Its official MCP server is a limited beta: **~36 tools, single sub-account (location) scope, no agency-wide access**.

The proposal: build an open-source **Rust workspace** containing:

1. **`ghl-sdk`** — a production-grade, typed, async Rust client for the GHL API 2.0, generated hybrid-style from GHL's official OpenAPI specs, with first-class OAuth + Private Integration Token auth, rate-limit-aware retries, and per-module streams for pagination.
2. **`ghl-mcp`** — an MCP server binary built on the official `rmcp` SDK (v3.x), backed by `ghl-sdk`, differentiated from GHL's official MCP server by **agency-level (multi-location) access, broader API coverage, and single-binary deployment** (no Node runtime).

**Honest correction to the premise:** the Rust niche is *nearly* empty, not literally empty. One community crate exists (`highlevel-api` v0.2.1, published July 2026, 57 total downloads, 0 stars, self-described "heavy development"). The opportunity is not "first Rust SDK" — it is **"first serious, maintained, agency-grade Rust SDK, and the first Rust-native GHL MCP server."** That is still a real gap, and the MCP angle is the stronger half of the pitch.

**Recommendation:** proceed with a 2-week validation spike (Phase 0), then a focused MVP covering the 5 highest-traffic API modules plus the MCP server. Do not attempt full 41-module coverage up front.

---

## 2. Market Research & Product Case

*(Following the product-brainstorming discipline: problem exploration → assumption testing → convergence. Facts below are tagged [V] verified from primary sources, [E] estimate/secondary source.)*

### 2.1 The platform

| Fact | Value | Confidence |
|---|---|---|
| Businesses served (sub-accounts) | ~2.2M claimed | [V] official About page |
| Paying agency customers | ~60–70k | [E] official milestone 60k+ (2023) + secondary 2026 sources |
| Funding | General Atlantic minority growth investment (Apr 2024); ~$1–1.8B valuation estimates | [V] investment / [E] valuation |
| ARR | $82.7M (2024, Latka est.) — likely understated | [E] |
| Business model | $97–$497/mo tiers; SaaS-mode resellers keep 100% of client billing | [V] |
| Marketplace | ~1,500+ apps; **0% commission** — developers keep the entire payment | [E] app count / [V] commission policy |

The distinctive dynamic: GHL's **SaaS-mode reseller layer** means every agency is itself a mini-SaaS vendor with integration and automation needs — a long tail of buyers for developer tooling that most CRMs don't have.

### 2.2 The developer ecosystem and its pain

- **API 2.0** spans **41 modules** (contacts, conversations, opportunities, calendars, payments, invoices, workflows, custom objects, SaaS API, voice-ai, …) with an official OpenAPI 3.0 spec repo: [GoHighLevel/highlevel-api-docs](https://github.com/GoHighLevel/highlevel-api-docs). A third-party count puts it at ~413 operations / 792 schemas (unverified but plausible). [V]
- **Official SDKs:** TypeScript (`@gohighlevel/api-client`, v3.0.0, ~52k dl/mo, 27 GitHub stars), Python and PHP (both v3.0.0, 3–4 stars, minimal adoption). No Rust/Go/Java/Ruby. [V]
- **Verified developer pain** (from GHL's own public issue tracker, 73 open issues): missing Funnels/reporting/pipeline-creation APIs, non-standardized pagination, webhook payload gaps, 100 req/10s + 200k req/day rate limits, **official policy of no developer support** ("Support does NOT provide code auditing or developer consultative services"). [V]
- **Freelance market:** GHL specialists bill **$25–150/hr** on Upwork; $300–$10k project sizes; a dedicated GHL jobs market exists. [E]
- **1,045 GitHub repos** match "gohighlevel" — a real community long-tail. [V]

### 2.3 The MCP landscape (the sharper half of the opportunity)

- GHL's **official MCP server**: `https://services.leadconnectorhq.com/mcp/` — ~36 tools across 9 categories, PIT auth, **single-location scope only**, included on $297+ plans, roadmap promising 250+ tools. A newer per-client v2 orchestrator endpoint (`/mcp/anthropic/v2`) exposes 5 meta-tools that proxy "hundreds of operations." [V]
- **Documented unmet needs** on GHL's ideas board: agency-wide connections, multi-location workflows, fuller CRUD coverage, workflow-builder access. [V]
- Community MCP servers fill the gap messily: `mastanley13/GoHighLevel-MCP` is the **most-starred GHL repo on GitHub** (184 stars); `@elitedcs/ghl-mcp` claims 233 tools at 3,372 npm dl/mo. All are Node-based, self-hosted, self-secured, and none is dominant. [V]
- Macro momentum: OpenAI and Google adopted MCP (2025); registries list 15–20k servers; Anthropic reports 400M+ monthly MCP SDK downloads. HubSpot, Salesforce, Linear, Notion all ship official CRM/SaaS MCP servers. [V]/[E]

### 2.4 Jobs to be done

1. **Backend/integration engineer at an agency or SaaS-mode reseller:** *"When my product needs to sync contacts/opportunities into GHL for hundreds of client locations, I want a typed, retrying, rate-limit-aware client so I can build reliable multi-tenant sync without hand-rolling HTTP."*
2. **AI-agent builder:** *"When I wire an agent to a client's CRM, I want an MCP server that covers the whole API across all their locations, deployable as one binary, so I don't run a Node process per client or hit the official server's single-location wall."*
3. **Rust-shop platform team:** *"When our stack is already Rust (axum services, agents via rmcp), I want a GHL client that fits our ecosystem instead of shelling out to a JS SDK."*

### 2.5 Assumption testing — what would kill this

| Assumption | Risk | Evidence today | Cheapest test |
|---|---|---|---|
| Enough developers want **Rust specifically** for GHL | **HIGH — the riskiest assumption.** GHL's dev base skews JS/PHP agency developers | 1,045 GHL repos are overwhelmingly JS/Python; the lone Rust crate has 57 downloads | Phase 0 spike + post in GHL Developer Council Slack & r/gohighlevel; measure stars/downloads/replies in 4 weeks |
| The MCP gap stays open | **MEDIUM.** GHL promises 250+ tools; their v2 meta-tool endpoint already widens coverage | Agency-wide scope still absent and unroadmapped; official server tied to $297+ plans | Track official MCP changelog monthly; differentiate on multi-location + self-host, not raw tool count |
| OpenAPI specs are codegen-quality | MEDIUM | Specs exist and are maintained, but pagination is non-uniform and per-module quirks are documented | Phase 0: run typify/progenitor over 3 modules, count manual patches needed |
| API stability | MEDIUM | `apps/v3/` specs are already staged in the official repo — **an API v3 is coming** | Build the codegen pipeline so regenerating against v3 is cheap; pin `Version` headers per module |
| MCP users care about implementation language | LOW-MEDIUM | They don't, directly — but they care about what Rust buys: single static binary, no npx/node supply-chain surface, low memory for always-on gateways | Lead marketing with deployment/ops benefits, not "it's Rust" |

**Deliberate counter-position (steelman against building):** if the goal were maximum adoption, a better-maintained *TypeScript* MCP server would reach more GHL developers. The reasons to do it in Rust anyway: (a) the TS niche is crowded (official SDK + 6 community MCP servers), Rust is open; (b) the single-binary/no-runtime story is a genuine differentiator for agencies deploying per-client gateways; (c) it compounds with the author's Rust positioning and the rmcp ecosystem's growth. This is a "own an empty niche and the ops story" bet, not a "biggest market" bet.

### 2.6 Positioning

> **The agency-grade GoHighLevel toolkit for Rust and AI agents.** One typed SDK, one static-binary MCP server, every location in your agency — no Node runtime, no per-location reconnecting, no waiting on official coverage.

Differentiators vs. official MCP server: agency (company) token support with per-location exchange, multi-location tools, self-hostable, broader coverage, open source. Differentiators vs. community Node servers: typed end-to-end, one binary, rate-limit pooling, maintained test suite against recorded fixtures.

### 2.7 Monetization (sequenced, not simultaneous)

1. **Now:** open source (Apache-2.0/MIT dual), consulting funnel — GHL officially provides no developer support, and experts bill $75–150/hr.
2. **Next:** hosted **multi-tenant MCP gateway** (OAuth, token vault, rate-limit pooling across locations, audit logs) — open-core; the single-location limit of the official server is the wedge.
3. **Later:** GHL Marketplace app (0% commission, 60k+ agency buyers) wrapping the gateway; and the Octokit/Supabase pattern — community SDKs demonstrably convert into vendor sponsorship, adoption, or hiring.

### 2.8 Success metrics

- Phase 0 gate: ≥ 30 combined stars or ≥ 10 substantive community replies within 4 weeks of the spike announcement.
- 6 months: 500+ crates.io downloads/month, 3 external contributors, MCP server listed on the official MCP Registry + mcp.so + Smithery.
- 12 months: 1 paying gateway customer or 1 sponsored engagement; parity with official MCP tool coverage.

---

## 3. System Design

*(Following the system-design framework: requirements → high-level design → deep dives → scale/reliability → trade-offs.)*

### 3.1 Requirements

**Functional**
- F1: Typed async client for GHL API 2.0 (`services.leadconnectorhq.com`), covering priority modules first: contacts, conversations, opportunities, calendars, locations, payments, invoices, custom objects, workflows(read), users.
- F2: Auth: OAuth 2.0 authorization-code + refresh flow (marketplace apps), **agency (company) token → location token exchange**, and Private Integration Tokens (PIT).
- F3: Automatic token refresh + pluggable token storage (memory, file, `TokenStore` trait for Redis/Postgres).
- F4: Rate-limit awareness: parse `X-RateLimit-Remaining` / `X-RateLimit-Daily-Remaining`, backoff on 429 honoring `Retry-After`.
- F5: Per-module pagination adapters exposed as `futures::Stream`.
- F6: Webhook helpers: signature validation + typed event payloads.
- F7: MCP server exposing SDK operations as tools over **stdio and Streamable HTTP**, with location scoping per call and agency-wide operation.
- F8: Raw escape hatch (`client.get_raw/post_raw`) for endpoints not yet typed — the octocrab pattern.

**Non-functional**
- N1: MSRV = stable − 6 months; CI-enforced; rustls by default (clean cross-compile, no OpenSSL).
- N2: Compile-time discipline: per-module cargo features (the async-stripe lesson — monolithic generated crates destroy compile times).
- N3: No secrets in logs; `Debug` impls redact tokens; MCP tool outputs never echo credentials.
- N4: Regeneration cost from updated/changed OpenAPI specs must be < 1 day (v3 is staged in GHL's repo — this is a hard requirement, not nice-to-have).
- N5: MCP server memory < 50 MB resident for always-on gateway deployments.

**Constraints:** solo maintainer initially; GHL rate limits (100 req/10s, 200k/day per app per location); unofficial status (no support channel, trademark care in naming/branding).

### 3.2 High-level architecture

```
┌─────────────────────────────────────────────────────────────────┐
│  cargo workspace: ghl-rs                                        │
│                                                                 │
│  ┌──────────────┐   ┌───────────────────────────────────────┐  │
│  │  ghl-types   │   │  ghl-sdk (lib)                        │  │
│  │  (generated  │◄──│  ClientBuilder │ AuthProvider trait   │  │
│  │  from GHL    │   │  reqwest + middleware stack:          │  │
│  │  OpenAPI     │   │   auth-refresh → rate-limit → retry   │  │
│  │  specs)      │   │  per-module services (feature-gated)  │  │
│  └──────────────┘   │  pagination Streams │ webhook verify  │  │
│         ▲           └───────────────┬───────────────────────┘  │
│         │                           │                           │
│  ┌──────┴──────┐    ┌───────────────▼───────────────────────┐  │
│  │ xtask:      │    │  ghl-mcp (bin)                        │  │
│  │ regen from  │    │  rmcp v3 · #[tool] handlers           │  │
│  │ OpenAPI     │    │  stdio + Streamable HTTP transports   │  │
│  └─────────────┘    │  LocationRouter (agency→location      │  │
│                     │  token exchange, per-location cache)  │  │
│                     └───────────────┬───────────────────────┘  │
└─────────────────────────────────────┼──────────────────────────┘
                                      │ HTTPS
                     ┌────────────────▼─────────────────┐
                     │ services.leadconnectorhq.com     │
                     │ (GHL API 2.0, Version headers,   │
                     │  OAuth /oauth/token,             │
                     │  /oauth/locationToken)           │
                     └──────────────────────────────────┘

Consumers:  Rust apps ──► ghl-sdk directly
            Claude / ChatGPT / Gemini / any MCP host ──► ghl-mcp
            (stdio for local, Streamable HTTP for hosted gateway)
```

### 3.3 SDK deep dive

**Codegen strategy — hybrid (the async-stripe/progenitor lesson):**
- Generate **types only** (`ghl-types`) from the official OpenAPI 3.0 specs using `typify`/progenitor-types, one Rust module per GHL spec file, regenerated by an `xtask`.
- **Hand-write the client surface** — builders, auth, retries, pagination, errors — so codegen churn never breaks the public API. Full-client generators (openapi-generator's Rust output) are non-idiomatic and would be hostage to GHL's spec quirks (documented non-uniform pagination, loose schemas).

**Client shape (octocrab-style semantic API over a raw core):**

```rust
let ghl = Ghl::builder()
    .auth(Auth::private_integration(pit_token))      // or Auth::oauth(config, token_store)
    .build()?;

// Typed, feature-gated module services
let contact = ghl.contacts()
    .create(&location_id, CreateContact { email: Some("a@b.co".into()), ..Default::default() })
    .await?;

// Pagination as a Stream (per-module adapter: contacts = startAfter/startAfterId cursor)
let mut stream = ghl.contacts().list(&location_id).limit(100).stream();
while let Some(c) = stream.try_next().await? { /* ... */ }

// Agency → location token exchange (the multi-tenant primitive)
let loc = ghl.as_location(&company_id, &location_id).await?;

// Escape hatch for not-yet-typed endpoints
let v: serde_json::Value = ghl.get_raw("/funnels/lookup", &[("locationId", id)]).await?;
```

**Middleware stack** (reqwest + `reqwest-middleware`, the 70M-download de-facto standard):
`AuthRefresh` (async `AuthProvider` trait, AWS-credentials-provider style, single-flight refresh) → `RateLimitGovernor` (per-location token bucket seeded from response headers; queues rather than errors at the 100/10s burst) → `Retry` (exponential backoff + jitter on 429/5xx/transport, honors `Retry-After`, idempotent-only by default) → `Trace` (tracing spans, tokens redacted).

**Version headers:** each module pins its required `Version:` value (2021-07-28 / 2021-04-15) as module metadata from the specs — never a user concern.

**Errors** (thiserror, the SDK norm):

```rust
pub enum Error {
    Api { status: StatusCode, code: Option<String>, message: String, request_id: Option<String> },
    RateLimited { retry_after: Option<Duration> },
    Auth(AuthError),          // refresh failed, token expired & non-recoverable, scope missing
    Transport(reqwest_middleware::Error),
    Decode { path: &'static str, source: serde_json::Error },  // includes endpoint context
}
```

**Feature flags:** `default = ["contacts", "conversations", "opportunities", "calendars", "locations", "rustls"]`; every other module opt-in; `native-tls` alternative; `webhooks`; `blocking` explicitly out of scope for v0.

### 3.4 MCP server deep dive

**Stack:** `rmcp` v3.x (official SDK, post-1.0, tracks MCP spec 2026-07-28) with `#[tool]`/`#[tool_router]` macros; transports: stdio (local hosts) and Streamable HTTP stateless (hosted gateway).

**Tool design — curated tools, not 1:1 endpoint dumping** (mcp-builder guidance: comprehensive coverage, but context-efficient):
- **Tier 1 — curated, high-frequency tools (~30–40):** `ghl_search_contacts`, `ghl_create_contact`, `ghl_send_message`, `ghl_list_opportunities`, `ghl_move_opportunity_stage`, `ghl_get_calendar_slots`, `ghl_book_appointment`, `ghl_list_invoices`, … Each: strict schemars input schema, `outputSchema` + structured content, annotations (`readOnlyHint`, `destructiveHint`, `idempotentHint`), paginated with explicit `cursor`/`limit`, responses trimmed to agent-relevant fields (full payloads behind `verbose: true`).
- **Tier 2 — meta-tools for the long tail (3):** `ghl_search_operations`, `ghl_describe_operation`, `ghl_execute_operation` — driven by the same OpenAPI metadata that generates `ghl-types`. This matches where GHL's own v2 MCP endpoint went, and gives ~413-operation coverage without 413 tool definitions polluting agent context.
- **Multi-location (the differentiator):** every tool takes optional `location_id`; server config accepts an agency token; a `LocationRouter` exchanges and caches per-location tokens (`/oauth/locationToken`) and enforces per-location rate budgets. `ghl_list_locations` lets agents discover scope. The official server cannot do any of this.
- **Safety:** destructive tools (delete contact, cancel invoice) ship behind a `--allow-destructive` flag and are annotated so hosts can require confirmation; errors return actionable text ("missing scope `contacts.write` — re-run install with …") per MCP best practices.

**Distribution** (all channels observed working in the wild for Rust MCP servers): `cargo install ghl-mcp`; **cargo-dist** → GitHub Releases binaries + shell installer + Homebrew tap; **npm wrapper** with platform binaries as optionalDependencies so `npx ghl-mcp` works in every MCP host config; Docker image for Streamable HTTP; listing on the official MCP Registry, mcp.so, and Smithery.

### 3.5 Scale & reliability

- Load profile is bounded by GHL's own limits (100 req/10s/location) — the engineering problem is **politeness and fairness across locations**, not throughput. The per-location token bucket + queue handles burst smoothing; daily-limit headroom is surfaced via a `ghl_rate_status` tool and `tracing` metrics.
- Hosted gateway (later phase): stateless Streamable HTTP behind any LB; token vault (encrypted at rest) is the only stateful piece; horizontal scaling trivial. Monitoring: `tracing` + OTLP exporter, alert on 429 ratio and refresh failures.
- Failure honesty: GHL has no SLA for the API; retries + idempotency-key support (where the API allows) is the ceiling of what a client can guarantee.

### 3.6 Key trade-offs (explicit)

| Decision | Chosen | Rejected | Why |
|---|---|---|---|
| Codegen | Types-only from OpenAPI + hand-written client | Full-client generation; fully hand-written | Spec quirks stay out of the public API; regeneration stays cheap for v3; hand-writing 413 ops doesn't scale to a solo maintainer |
| MCP tool surface | ~35 curated + 3 meta-tools | 1:1 tool per endpoint (413 tools) | Context bloat kills agent performance; meta-tools proven by GHL's own v2 design |
| Language | Rust | TypeScript (bigger GHL audience) | Empty niche + single-binary ops story + rmcp maturity; TS space already has 6+ competitors and the official SDK |
| HTTP stack | reqwest + reqwest-middleware + backon | hyper direct; isahc | Ecosystem-standard, middleware composability, 70M downloads of prior art |
| Versioning | 0.x, semver-checked (cargo-semver-checks), MSRV bumps in minors | 1.0 early | API v3 is staged upstream; committing to 1.0 before v3 lands would force a fast 2.0 |
| Scope of v0 | 5 modules + MCP | Full 41-module parity | Ship in weeks, validate demand before paying the long-tail cost |

**Revisit as it grows:** split `ghl-types` per-module into crates if compile times degrade (async-stripe's exact evolution); stateful Streamable HTTP sessions if long-running agent tasks demand it; workflow/funnel modules when GHL ships those APIs (top-voted gaps).

---

## 4. Delivery Roadmap

| Phase | Duration | Deliverables | Exit gate |
|---|---|---|---|
| **0 — Validation spike** | 2 wks | Codegen pipeline over 3 modules; PIT auth + contacts CRUD working; `ghl-mcp` PoC with 5 tools on stdio; announce in GHL Developer Council Slack, r/gohighlevel, MCP communities | Codegen patch-count acceptable AND community signal (≥30 stars or ≥10 substantive replies in 4 wks) |
| **1 — SDK MVP** | 4–6 wks | `ghl-sdk` 0.1: OAuth + PIT + token exchange, 5 priority modules, retries/rate-limiting, pagination streams, webhook verification, docs.rs complete, recorded-fixture test suite | Published to crates.io; example app (multi-location contact sync) runs clean |
| **2 — MCP server MVP** | 3–4 wks | `ghl-mcp` 0.1: ~35 curated tools + 3 meta-tools, stdio + Streamable HTTP, agency multi-location routing, npm/npx wrapper + cargo-dist binaries + Docker; registry listings; **MCP evaluation suite** (10 read-only, multi-step, verifiable Q&A per mcp-builder methodology) run against Claude | Eval pass-rate ≥ 8/10; installable via npx one-liner |
| **3 — Coverage & community** | ongoing | Long-tail modules by demand; contributor docs; monthly regen against upstream specs; track official MCP roadmap | 3 external contributors; parity on official MCP tool coverage |
| **4 — Monetization** | after traction | Hosted gateway (open-core), GHL Marketplace app | First paying customer |

---

## 5. Engineering Quality Standards

*(What the code-review discipline demands of this codebase — these are the standing review gates, applied from the first PR.)*

**Security**
- Tokens never logged, never in `Debug` output (newtype `Secret<T>` with redacting impls); never in URL query params where a header works.
- Webhook signature validation is constant-time; sample payloads in tests contain no real customer data.
- MCP server: destructive tools gated + annotated; meta-tool `ghl_execute_operation` validates against the OpenAPI allowlist (no SSRF via arbitrary paths); Streamable HTTP mode requires auth — never an open proxy to a customer's CRM. Supply-chain: `cargo-deny` (advisories + licenses) in CI, lockfile committed, release binaries built only from CI.

**Correctness**
- Every retry path idempotent-only by default; 429 handling honors `Retry-After` and never tight-loops.
- Deserialization is forward-compatible: unknown fields tolerated (`deny_unknown_fields` off), enums with `#[serde(other)]` fallbacks — GHL ships fast and payloads drift (documented webhook payload gaps).
- Pagination adapters tested per module against recorded fixtures, since GHL pagination is per-module inconsistent (cursor vs skip vs page).

**Testing strategy**
- Unit: middleware stack against `wiremock` (429s, refresh races, header parsing).
- Integration: recorded fixtures (VCR-style) from a real dev sub-account; a small live smoke suite behind an env-gated feature, run pre-release only (rate-limit-respectful).
- MCP: MCP Inspector in CI smoke; the 10-question eval suite as the release regression bar.

**CI (GitHub Actions):** fmt, clippy `-D warnings`, test matrix {stable, MSRV} × {`--all-features`, `--no-default-features`}, cargo-deny, cargo-semver-checks pre-release, docs build with `missing_docs` denied, release automation via release-plz + cargo-dist.

---

## 6. Documentation Plan

*(Write for the reader; fastest path to first success; show don't tell.)*

| Artifact | Audience | Content commitment |
|---|---|---|
| Workspace README | Evaluating dev, 30 seconds | What/why, badge row, **< 5-minute quickstart for both consumption modes**: `cargo add ghl-sdk` + 10-line contact-create; `npx ghl-mcp` + Claude Desktop/Code config JSON snippet |
| docs.rs | SDK user | Every public item documented (CI-enforced); module-level docs state each GHL module's Version header, pagination style, and required scopes |
| `examples/` | Copy-paste dev | `contact_sync.rs` (multi-location), `oauth_flow.rs` (axum callback + token store), `webhook_server.rs`, `agent_demo/` (Claude + ghl-mcp walkthrough) |
| MCP tool reference | Agent builder | Generated from tool schemas — one page per tool: description, input/output schema, annotations, required GHL scopes, example transcript |
| AUTH.md | Everyone (biggest confusion source in GHL land) | PIT vs OAuth vs agency-token decision tree; scope catalog for the covered modules; token-lifetime table |
| Runbook (gateway phase) | Operator | Deploy, token-vault rotation, rate-limit alarms, rollback |
| CONTRIBUTING.md | Contributor | Codegen regen workflow (`cargo xtask regen`), fixture recording, module-addition checklist |
| Non-affiliation notice | Everyone | Clear "unofficial, not affiliated with HighLevel Inc." in README + crate metadata — trademark hygiene |

Docs debt rule: a module ships only with its docs.rs coverage, example snippet, and AUTH.md scope entries in the same PR.

---

## 7. Risks & Open Questions

**Risks**
1. **GHL ships agency-scope MCP + 250 tools before Phase 2 lands** — mitigations: self-host story and Rust SDK remain; speed matters, hence the small MVP.
2. **API v3 migration** (specs already staged upstream) — mitigation: codegen pipeline as a first-class deliverable (N4); pin v2 Version headers; plan a `v3` feature flag.
3. **Solo-maintainer burnout across 41 modules** — mitigation: demand-driven coverage, meta-tools carry the long tail, contributor pipeline from day one.
4. **Trademark** — "GoHighLevel"/"HighLevel" are trademarks; use `ghl-*` naming, explicit non-affiliation, no logo use. (Their marketplace tolerates a large unofficial ecosystem today.)
5. **Weak Rust demand in GHL's community** — the Phase 0 gate exists precisely to kill the project cheaply if signal doesn't appear.

**Open questions**
- Exact rate-limit scoping (per app per location vs per resource) — verify against primary docs / empirically in Phase 0 before hardcoding the governor.
- Does GHL's OAuth support enough of the flow headlessly for a pure-CLI MCP onboarding, or does the gateway need a hosted callback? (Phase 0 spike item.)
- Should `ghl-types` be one crate or per-module crates from day one? (Measure compile times in Phase 0; async-stripe precedent says split eventually.)
- Marketplace app review implications for an MCP gateway app (~10-business-day review claim is third-party — confirm officially).

---

## 8. Sources

**Primary:** [GHL API docs](https://marketplace.gohighlevel.com/docs/) · [OAuth docs](https://marketplace.gohighlevel.com/docs/Authorization/OAuth2.0/) · [Official OpenAPI specs](https://github.com/GoHighLevel/highlevel-api-docs) (+ its issue tracker for pain points) · [Official MCP server docs](https://marketplace.gohighlevel.com/docs/other/mcp) · [MCP launch post](https://www.gohighlevel.com/post/introducing-the-mcp-server) · [v1 deprecation](https://www.gohighlevel.com/post/deprecating-the-highlevel-api-v1-and-migrating-to-v2) · [About/scale claims](https://www.gohighlevel.com/about-us) · [Marketplace 0% pricing](https://help.gohighlevel.com/support/solutions/articles/155000001217) · [General Atlantic investment](https://www.generalatlantic.com/media-article/highlevel-announces-minority-growth-investment-from-general-atlantic/) · [@gohighlevel/api-client (npm)](https://www.npmjs.com/package/@gohighlevel/api-client) · [rmcp / official Rust MCP SDK](https://github.com/modelcontextprotocol/rust-sdk) · [SO Survey 2025](https://survey.stackoverflow.co/2025/technology) · crates.io API (name availability, download counts, `highlevel-api` crate)

**Secondary (flagged where used):** Latka ARR estimate · ghllogic rate-limit guide · lobehub operation counts · appsforghl review-timeline claim · TaskVirtual/ghlbuilds 2026 agency counts · truefoundry/tooldirectory MCP registry sizes

**Ecosystem references:** [octocrab](https://github.com/XAMPPRocky/octocrab) · [async-stripe](https://github.com/arlyon/async-stripe) · [progenitor](https://github.com/oxidecomputer/progenitor) · [reqwest-middleware](https://github.com/TrueLayer/reqwest-middleware) · [cargo-dist](https://opensource.axo.dev/cargo-dist/) · [mastanley13/GoHighLevel-MCP](https://github.com/mastanley13/GoHighLevel-MCP) · [smithy-rs design](https://smithy-lang.github.io/smithy-rs/design/)
