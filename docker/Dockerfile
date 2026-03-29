# syntax=docker/dockerfile:1
# ── Builder ──────────────────────────────────────────────────────────
FROM rust:1.91-bookworm AS builder

WORKDIR /build

COPY Cargo.toml Cargo.lock ./
COPY crates/ crates/
COPY packages/ packages/
COPY tools/ tools/

RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/usr/local/cargo/git \
    --mount=type=cache,target=/build/target \
    cargo build --release --package liter-llm-cli && \
    cp target/release/liter-llm /usr/local/bin/liter-llm

# ── Runtime (Chainguard) ─────────────────────────────────────────────
FROM cgr.dev/chainguard/glibc-dynamic:latest

COPY --from=builder /usr/local/bin/liter-llm /usr/bin/liter-llm

ENV RUST_LOG=info

EXPOSE 4000

LABEL org.opencontainers.image.source="https://github.com/kreuzberg-dev/liter-llm"
LABEL org.opencontainers.image.description="LiterLLM — OpenAI-compatible LLM proxy server with 142+ providers"
LABEL org.opencontainers.image.licenses="MIT"

ENTRYPOINT ["liter-llm"]
CMD ["api", "--host", "0.0.0.0", "--port", "4000"]
