#!/bin/bash
# get-kurtosis-keys.sh
# Downloads all validator keys and secrets from Kurtosis enclave
# and consolidates them into single directories

set -e

ENCLAVE_NAME="${ENCLAVE_NAME:-preconf-testnet}"
OUTPUT_DIR="${OUTPUT_DIR:-/tmp/fabric}"
TEMP_DIR=$(mktemp -d)

echo "=== Fetching Validator Keys from enclave: ${ENCLAVE_NAME} ==="
echo "Output directory: ${OUTPUT_DIR}"
echo ""

# Clean up temp dir on exit
cleanup() {
    rm -rf "$TEMP_DIR"
}
trap cleanup EXIT

# Create output directories
rm -rf "${OUTPUT_DIR}/validator_keys" "${OUTPUT_DIR}/validator_secrets"
mkdir -p "${OUTPUT_DIR}/validator_keys" "${OUTPUT_DIR}/validator_secrets"

# Get list of validator key artifacts
# Pattern matches: {number}-{cl_client}-{el_client}-{start}-{end}
# e.g., 1-lighthouse-geth-0-3, 2-lighthouse-geth-4-7
echo "Discovering validator key artifacts..."
ARTIFACTS=$(kurtosis enclave inspect "$ENCLAVE_NAME" 2>/dev/null | \
    grep -E '^[a-f0-9]+\s+[0-9]+-[a-zA-Z0-9]+-[a-zA-Z0-9]+-[0-9]+-[0-9]+$' | \
    awk '{print $2}' || true)

if [ -z "$ARTIFACTS" ]; then
    echo "ERROR: No validator key artifacts found in enclave '${ENCLAVE_NAME}'"
    echo "Expected artifacts matching pattern: {index}-{cl}-{el}-{start}-{end}"
    exit 1
fi

# Count artifacts
ARTIFACT_COUNT=$(echo "$ARTIFACTS" | wc -l | tr -d ' ')
echo "Found ${ARTIFACT_COUNT} validator key artifact(s)"
echo ""

# Download and merge each artifact
CURRENT=0
for ARTIFACT in $ARTIFACTS; do
    CURRENT=$((CURRENT + 1))
    echo "[${CURRENT}/${ARTIFACT_COUNT}] Processing: ${ARTIFACT}"
    
    # Download to temp directory
    DOWNLOAD_DIR="${TEMP_DIR}/${ARTIFACT}"
    mkdir -p "$DOWNLOAD_DIR"
    kurtosis files download "$ENCLAVE_NAME" "$ARTIFACT" "$DOWNLOAD_DIR" >/dev/null 2>&1
    
    # Copy keys if they exist
    if [ -d "${DOWNLOAD_DIR}/keys" ]; then
        KEY_COUNT=$(ls -1 "${DOWNLOAD_DIR}/keys" 2>/dev/null | wc -l | tr -d ' ')
        echo "  - Copying ${KEY_COUNT} keys..."
        cp -r "${DOWNLOAD_DIR}/keys/"* "${OUTPUT_DIR}/validator_keys/" 2>/dev/null || true
    fi
    
    # Copy secrets if they exist
    if [ -d "${DOWNLOAD_DIR}/secrets" ]; then
        SECRET_COUNT=$(ls -1 "${DOWNLOAD_DIR}/secrets" 2>/dev/null | wc -l | tr -d ' ')
        echo "  - Copying ${SECRET_COUNT} secrets..."
        cp -r "${DOWNLOAD_DIR}/secrets/"* "${OUTPUT_DIR}/validator_secrets/" 2>/dev/null || true
    fi
done

echo ""
echo "=== Validator Keys Fetched Successfully ==="

# Show summary
TOTAL_KEYS=$(ls -1 "${OUTPUT_DIR}/validator_keys" 2>/dev/null | wc -l | tr -d ' ')
TOTAL_SECRETS=$(ls -1 "${OUTPUT_DIR}/validator_secrets" 2>/dev/null | wc -l | tr -d ' ')

echo "Total keys:    ${TOTAL_KEYS} -> ${OUTPUT_DIR}/validator_keys/"
echo "Total secrets: ${TOTAL_SECRETS} -> ${OUTPUT_DIR}/validator_secrets/"
echo ""

# List the keys
if [ "$TOTAL_KEYS" -gt 0 ]; then
    echo "Validator public keys:"
    ls -1 "${OUTPUT_DIR}/validator_keys" | head -10
    if [ "$TOTAL_KEYS" -gt 10 ]; then
        echo "  ... and $((TOTAL_KEYS - 10)) more"
    fi
fi

#!/bin/bash