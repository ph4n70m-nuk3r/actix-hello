FROM rust:alpine AS chef
WORKDIR /app
RUN apk add --no-cache alpine-sdk musl-dev
RUN cargo install cargo-chef

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
## Dependency caching occurs on this layer:
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin actix-hello

## Final runtime image:
FROM alpine:latest AS runtime
LABEL MAINTAINER=ph4n70m-nuk3r
WORKDIR /app
COPY --from=builder /app/target/release/actix-hello /usr/local/bin
COPY config.toml config.toml
EXPOSE 8080
ENTRYPOINT ["/usr/local/bin/actix-hello"]
