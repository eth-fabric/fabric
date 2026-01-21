#!/bin/bash
# get-kurtosis-data.sh
# Downloads genesis files, JWT secret, and bootnode info from Kurtosis enclave
# Uses kurtosis CLI + assumes the apache additional service is enabled in kurtosis-network-params.yaml

set -e
ENCLAVE="${1:-preconf-testnet}"
DATA_DIR="${2:?usage: get_kurtosis_data.sh [enclave] <path-to-data-directory>}"

echo "=== Fetching Kurtosis data from enclave: ${ENCLAVE} ==="

rm -rf "${DATA_DIR}"
mkdir -p "${DATA_DIR}/genesis" "${DATA_DIR}/jwt"

# Get Apache's port
APACHE_PORT=$(kurtosis port print "${ENCLAVE}" apache http)

# [1/2] Download everything from Apache in one shot (genesis + bootnodes)
echo "[1/2] Downloading genesis data and bootnodes from Apache..."
curl -sL "${APACHE_PORT}/network-config.tar" | tar -xz -C "${DATA_DIR}/genesis"

# [2/2] JWT is not served by Apache, so we still need kurtosis CLI for this
echo "[2/2] Downloading JWT secret..."
kurtosis files download "${ENCLAVE}" jwt_file "${DATA_DIR}/jwt"

echo ""
echo "=== Data fetched successfully ==="
echo "Genesis files: ${DATA_DIR}/genesis/"
echo "JWT secret:    ${DATA_DIR}/jwt/jwtsecret"