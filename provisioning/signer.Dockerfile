# syntax=docker/dockerfile:1.7-labs
FROM debian:bookworm-slim
ARG BINARIES_PATH
ARG PLATFORM
ENV RUST_LOG=info
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ${BINARIES_PATH}/${PLATFORM}/local-signer-module /usr/local/bin/app
RUN useradd -u 10001 -m appuser && \
    mkdir -p /app/data/proxy /app/data/keystores/keys /app/data/keystores/secrets && \
    chown -R appuser:appuser /app
USER appuser
ENTRYPOINT ["/usr/local/bin/app"]