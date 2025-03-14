FROM rust:1.83-alpine AS builder

WORKDIR /app

RUN apk add --no-cache musl-dev

RUN cargo init
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release

COPY src ./src
# Update timestamp so Cargo knows to rebuild
RUN touch src/main.rs
ENV PROJECT_NAME=cicd-tp
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/app/target \
    cargo build --release && cp target/release/$PROJECT_NAME /app.bin

FROM scratch
USER 1000
COPY --from=builder /app.bin /app.bin

EXPOSE 3000
CMD ["/app.bin"]
