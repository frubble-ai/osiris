# Osiris's image: one binary that runs every process. OSIRIS_PROCESS (serve, worker or migrate)
# chooses which, so each Cloud Run service or job sets only environment variables, never a command
# or arguments. Nothing environment-specific is baked in: no settings files, no secrets. Only the
# build's commit, from the OSIRIS_COMMIT build argument, which /health reports.
#
# cargo-chef builds the dependencies in their own layer, so a change to our code alone rebuilds in
# seconds.

FROM rust:1.99-slim-bookworm AS chef
RUN apt-get update && apt-get install -y --no-install-recommends cmake g++ make \
    && rm -rf /var/lib/apt/lists/* \
    && cargo install cargo-chef --locked
WORKDIR /osiris

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /osiris/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json -p osiris_server
COPY . .
# Queries are checked against the committed .sqlx/: no database at build.
ENV SQLX_OFFLINE=true
RUN cargo build --release --locked -p osiris_server

# Distroless: libc and CA roots, no shell or package manager, and runs as the non-root user.
FROM gcr.io/distroless/cc-debian12:nonroot AS runtime
ARG OSIRIS_COMMIT=local
ENV OSIRIS_COMMIT=$OSIRIS_COMMIT
COPY --from=builder /osiris/target/release/osiris-server /usr/local/bin/osiris-server
USER nonroot
ENTRYPOINT ["/usr/local/bin/osiris-server"]
