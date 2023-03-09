FROM rust:alpine AS builder

WORKDIR /build

RUN apk add --no-cache musl-dev

COPY . .

RUN --mount=type=cache,target=/build/target \
    --mount=type=cache,target=/cargo \
    CARGO_HOME=/cargo cargo build --locked --release \
    && mkdir dist \
    && cp $(find target/release/ -maxdepth 1 -executable -type f) dist/ \
    && strip dist/*

FROM scratch

LABEL org.opencontainers.image.source="https://github.com/Bootstrap-Academy/backend"

ENV RUST_LOG=info

COPY --from=builder /build/dist /