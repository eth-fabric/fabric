#!/bin/bash

set -e

export ENCLAVE_NAME="${ENCLAVE_NAME:-preconf-testnet}"
export SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
export REPO_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"
export OUTPUT_DIR=${OUTPUT_DIR:-/tmp/fabric}
export FABRIC_CONFIG=${REPO_ROOT}/config/docker.config.toml
export CONSTRAINTS_BUILDER_CONFIG=../../constraints_builder/docker/config/constraints-builder-docker-config.toml
export CONSTRAINTS_BUILDER_BLOCKLIST=../../constraints_builder/docker/config/blocklist.json

docker compose down 

kurtosis clean -a