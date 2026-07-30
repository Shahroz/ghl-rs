# Build the MCP server, then ship it in a distroless image.
FROM rust:1.88-slim AS build
WORKDIR /src
# Copy manifests first so dependency compilation caches across source edits.
COPY Cargo.toml Cargo.lock ./
COPY crates/ghl-models/Cargo.toml crates/ghl-models/
COPY crates/ghl-sdk/Cargo.toml crates/ghl-sdk/
COPY crates/ghl-mcp/Cargo.toml crates/ghl-mcp/
COPY . .
RUN cargo build --release -p ghl-mcp

FROM gcr.io/distroless/cc-debian12
COPY --from=build /src/target/release/ghl-mcp /usr/local/bin/ghl-mcp
# Streamable HTTP; override with your own args for stdio.
EXPOSE 8000
ENV GHL_HTTP_ADDR=0.0.0.0:8000
ENTRYPOINT ["/usr/local/bin/ghl-mcp"]
