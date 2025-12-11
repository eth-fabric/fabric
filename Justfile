# ===============================
# Local binary execution (without Docker)
# ===============================

# Generate config and .env files in config/simulation
setup-simulation:
	cargo run --bin simulation-setup

# Run local signer module
run-local-signer:
	#!/usr/bin/env bash
	set -a
	source config/simulation/signer.env
	set +a
	cargo run --bin local-signer-module

# Run local gateway module
run-local-gateway:
	#!/usr/bin/env bash
	set -a
	source config/simulation/gateway.env
	set +a
	cargo run --bin gateway

# Run local proposer module
run-local-proposer:
	#!/usr/bin/env bash
	set -a
	source config/simulation/proposer.env
	set +a
	cargo run --bin proposer

# Run local relay
run-local-relay:
	#!/usr/bin/env bash
	set -a
	source config/simulation/relay.env
	set +a
	cargo run --bin relay

# Run local spammer
run-local-spammer:
	#!/usr/bin/env bash
	set -a
	source config/simulation/spammer.env
	set +a
	cargo run --bin spammer

# Run local mock beacon node
run-local-beacon-mock:
	#!/usr/bin/env bash
	set -a
	source config/simulation/beacon-mock.env
	set +a
	cargo run --bin beacon-mock

# ===============================
# Docker building and execution
# ===============================

# Generate config and .env files in config/docker
setup-docker-simulation:
	DOCKER=true cargo run --bin simulation-setup
	
# Run all dockerized services
up VERSION="dev":
	#!/usr/bin/env bash
	export VERSION={{VERSION}}
	echo "Starting all dockerized services (version: {{VERSION}})"
	docker compose up -d

down:
	docker compose down

logs SERVICE:
	docker compose logs -f {{SERVICE}}

# ===============================
# Commit-boost style builders
# ===============================

_create-docker-builder:
	docker buildx create --name multiarch-builder --driver docker-container --use > /dev/null 2>&1 || true

# Detect the first supported platform shorthand like linux_amd64 or linux_arm64
_platform:
	@docker buildx inspect --bootstrap | sed -n 's/^ *Platforms: *//p' | cut -d',' -f1 | tr -d ' ' | tr '/' '_'

# Build binary artifact for a workspace bin into ./build/<version>/<platform>

_docker-build-binary version bin: _create-docker-builder
	docker buildx build --rm --platform=local \
	  -f provisioning/build.Dockerfile \
	  --output "build/{{version}}" \
	  --target output \
	  --build-arg TARGET_CRATE=fabric-binaries \
	  --build-arg BINARY_NAME={{bin}} .

# Build runtime image for a bin using prebuilt artifacts
_docker-build-image version bin: _create-docker-builder
	PLATFORM=`just _platform`; \
	docker buildx build --rm --load \
	  -f provisioning/{{bin}}.Dockerfile \
	  --build-arg BINARIES_PATH=build/{{version}} \
	  --build-arg PLATFORM=$PLATFORM \
	  -t fabric/{{bin}}:{{version}} .

build-gateway version: (_docker-build-binary version "gateway") (_docker-build-image version "gateway")
build-relay version:   (_docker-build-binary version "relay")   (_docker-build-image version "relay")
build-proposer version:(_docker-build-binary version "proposer")(_docker-build-image version "proposer")
build-spammer version: (_docker-build-binary version "spammer") (_docker-build-image version "spammer")
build-signer version:  (_docker-build-binary version "local-signer-module") (_docker-build-image version "signer")
build-beacon-mock version: (_docker-build-binary version "beacon-mock") (_docker-build-image version "beacon-mock")

build-all version:
	just build-gateway {{version}} && \
	just build-relay {{version}} && \
	just build-proposer {{version}} && \
	just build-spammer {{version}} && \
	just build-signer {{version}} && \
	just build-beacon-mock {{version}}
