# ===============================
# Local binary execution (without Docker)
# ===============================

# Generate .simulation.env with JWTs and configuration
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