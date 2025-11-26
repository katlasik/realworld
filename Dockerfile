FROM rust:1.91-bookworm AS builder

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./

COPY ./src ./src

COPY ./migrations ./migrations

RUN cargo build --release

FROM debian:12-slim

WORKDIR /

RUN groupadd --system realworld

RUN useradd --system --gid realworld --create-home realworld

RUN apt-get update && apt-get install -y --no-install-recommends libssl3 && rm -rf /var/lib/apt/lists/*

USER realworld

COPY --from=builder /usr/src/app/target/release/realworld /usr/local/bin/realworld

CMD ["/usr/local/bin/realworld"]