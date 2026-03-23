# syntax=docker/dockerfile:1
FROM lukemathwalker/cargo-chef:latest-rust-alpine AS chef
RUN apk add --no-cache musl-dev
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json .
# build deps only (cached as long as Cargo.toml/lock don't change)
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM scratch
COPY --from=builder /app/target/release/tokount /tokount
ENTRYPOINT ["/tokount"]
