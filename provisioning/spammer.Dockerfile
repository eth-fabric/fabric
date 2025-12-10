# syntax=docker/dockerfile:1.7-labs
FROM debian:bookworm-slim
ARG BINARIES_PATH
ARG PLATFORM
ENV RUST_LOG=info
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates curl && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY ${BINARIES_PATH}/${PLATFORM}/spammer /usr/local/bin/app
USER 10001:10001
ENTRYPOINT ["/usr/local/bin/app"]
