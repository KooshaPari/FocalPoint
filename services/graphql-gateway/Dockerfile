# Builder stage
FROM rust:1.82-alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /app

# Copy workspace manifests and source
COPY Cargo.toml Cargo.lock ./
COPY crates ./crates
COPY services/graphql-gateway ./services/graphql-gateway

# Build the GraphQL gateway binary
RUN cargo build --release -p focus-graphql-gateway

# Runtime stage
FROM alpine:3.21

RUN apk add --no-cache ca-certificates

WORKDIR /app

# Copy binary from builder
COPY --from=builder /app/target/release/focus-graphql-gateway .

# Labels
LABEL org.opencontainers.image.title="FocalPoint GraphQL Gateway"
LABEL org.opencontainers.image.description="GraphQL API gateway for FocalPoint — queries, mutations, subscriptions"
LABEL org.opencontainers.image.vendor="FocalPoint"

# Environment: production mode, no auth required by default
ENV FOCALPOINT_GRAPHQL_PROD=false
ENV FOCALPOINT_GRAPHQL_TOKEN=""
ENV RUST_LOG=info

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD wget -q --spider http://localhost:8473/health || exit 1

# Expose GraphQL port
EXPOSE 8473

# Run the service
CMD ["./focus-graphql-gateway", "--bind", "0.0.0.0:8473"]
