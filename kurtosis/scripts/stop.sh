#!/bin/bash

set -e
# The name of the Kurtosis enclave, default: preconf-testnet
export ENCLAVE_NAME="${ENCLAVE_NAME:-preconf-testnet}"

# The directory containing this script
export SCRIPTS_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# The directory containing the root Kurtosis directory
export KURTOSIS_DIR="$(cd "${SCRIPTS_DIR}/.." && pwd)"

# The root directory of the repository
export REPO_ROOT="$(cd "${KURTOSIS_DIR}/.." && pwd)"

# The directory to write output to, default: /tmp/fabric
export OUTPUT_DIR=${OUTPUT_DIR:-/tmp/fabric}

# The directory containing the simulation configuration files
export CONFIG_DIR=${KURTOSIS_DIR}/config

# The config file for the fabric services
export FABRIC_CONFIG=${REPO_ROOT}/config/docker.config.toml

# The config file for the constraints builder
export CONSTRAINTS_BUILDER_CONFIG=${CONFIG_DIR}/constraints-builder-config.toml

# The blocklist file for the constraints builder
export CONSTRAINTS_BUILDER_BLOCKLIST=${CONFIG_DIR}/blocklist.json

cd ${CONFIG_DIR}

docker compose down 

kurtosis clean -a