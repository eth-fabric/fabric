#!/bin/bash
# get-kurtosis-data.sh
# Downloads genesis files, JWT secret, and bootnode info from Kurtosis enclave
# Uses kurtosis CLI + assumes the apache additional service is enabled in kurtosis-network-params.yaml

set -e

ENCLAVE_NAME="${ENCLAVE_NAME:-preconf-testnet}"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
DATA_DIR="${SCRIPT_DIR}/data"

echo "=== Fetching Kurtosis data from enclave: ${ENCLAVE_NAME} ==="

rm -rf "${DATA_DIR}"
mkdir -p "${DATA_DIR}/genesis" "${DATA_DIR}/jwt"

# Get Apache's port
APACHE_PORT=$(kurtosis port print "${ENCLAVE_NAME}" apache http)

# [1/2] Download everything from Apache in one shot (genesis + bootnodes)
echo "[1/2] Downloading genesis data and bootnodes from Apache..."
curl -sL "${APACHE_PORT}/network-config.tar" | tar -xz -C "${DATA_DIR}/genesis"

# [2/2] JWT is not served by Apache, so we still need kurtosis CLI for this
echo "[2/2] Downloading JWT secret..."
kurtosis files download "${ENCLAVE_NAME}" jwt_file "${DATA_DIR}/jwt"

echo ""
echo "=== Data fetched successfully ==="
echo "Genesis files: ${DATA_DIR}/genesis/"
echo "JWT secret:    ${DATA_DIR}/jwt/jwtsecret"