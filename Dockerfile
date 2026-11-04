# Osiris's image: the service (`osiris serve`) and its migration job (`osiris migrate`).
# cargo-chef builds the dependencies in their own layer, so a change to our code alone
# rebuilds in seconds. Queries are checked against .sqlx/ (SQLX_OFFLINE): no database at build.

FROM rust:1.98-slim-bookworm AS chef
RUN apt-get update && apt-get install -y --no-install-recommends cmake g++ make \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-chef --locked
WORKDIR /osiris

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /osiris/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
ENV SQLX_OFFLINE=true
RUN cargo build --release --locked

# Distroless: libc and CA roots, no shell, runs as nonroot.
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
COPY --from=builder /osiris/target/release/osiris /usr/local/bin/osiris
ENV LOG_FORMAT=json PORT=8080
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/osiris"]
CMD ["serve"]
