#!/bin/bash
# run.sh
# Full orchestration script:
# 1. Docker compose down
# 2. Tear down existing Kurtosis enclave
# 3. Run Kurtosis to create network
# 4. Extract ports and update configuration files
# 5. Update chain spec in fabric config
# 6. Download proposer keystores
# 7. Fetch genesis/jwt/bootnodes
# 8. Fetch Kurtosis lighthouse peer info
# 9. Clean previous fabric databases
# 10. Auto generate configs
# 11. Docker compose up

set -e

# Parse command line arguments
DOCKER_ONLY=false
for arg in "$@"; do
    case $arg in
        --docker-only)
            DOCKER_ONLY=true
            shift
            ;;
    esac
done

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

# The directory containing the simulation data files
export DATA_DIR=${CONFIG_DIR}/data

# The config file for the fabric services
export FABRIC_CONFIG=${REPO_ROOT}/config/docker.config.toml

# The config file for the constraints builder
export CONSTRAINTS_BUILDER_CONFIG=${CONFIG_DIR}/constraints-builder-config.toml

# The blocklist file for the constraints builder
export CONSTRAINTS_BUILDER_BLOCKLIST=${CONFIG_DIR}/blocklist.json

echo "========================================"
echo "  Inclusion Preconf Kurtosis Testnet Setup"
echo "========================================"
echo "Enclave: ${ENCLAVE_NAME}"
echo "Kurtosis dir: ${KURTOSIS_DIR}"
echo "Repo root: ${REPO_ROOT}"
if [ "$DOCKER_ONLY" = true ]; then
    echo "Mode: DOCKER_ONLY (skipping Kurtosis teardown/creation)"
fi
echo ""

# Step 1: Stop existing Docker Compose services
echo ""
echo "[Step 1/11] Stopping existing Docker Compose services..."
cd "${CONFIG_DIR}"
docker compose down -v 2>/dev/null || echo "  No existing compose services to stop"

# Step 2: Tear down existing Kurtosis enclave
if [ "$DOCKER_ONLY" = true ]; then
    echo "[Step 2/11] Skipping Kurtosis teardown (--docker-only mode)"
else
    echo "[Step 2/11] Tearing down existing Kurtosis enclave..."
    kurtosis enclave rm "${ENCLAVE_NAME}" --force 2>/dev/null || echo "  No existing enclave to remove"
fi

# Step 3: Run Kurtosis to create the network
if [ "$DOCKER_ONLY" = true ]; then
    echo ""
    echo "[Step 3/11] Skipping Kurtosis network creation (--docker-only mode)"
else
    echo ""
    echo "[Step 3/11] Starting Kurtosis network..."
    cd "${REPO_ROOT}"
    kurtosis run github.com/ethpandaops/ethereum-package --enclave "${ENCLAVE_NAME}" --args-file "${CONFIG_DIR}/kurtosis-network-params.yaml"
fi

# Step 4: Extract the ports from the Kurtosis network and update the configuration files
echo ""
echo "[Step 4/11] Extracting ports from Kurtosis network and updating configuration files..."
cd "${SCRIPTS_DIR}"
python3 get_config_ports.py \
  --enclave ${ENCLAVE_NAME} \
  --fabric-config ${FABRIC_CONFIG} \

# Step 5: Update the chain spec in the fabric config
echo ""
echo "[Step 5/11] Updating chain spec in fabric config..."
cd "${SCRIPTS_DIR}"
./get_chain_spec.sh ${ENCLAVE_NAME} ${FABRIC_CONFIG}

# Step 6: Download a copy of the proposer keystores
echo ""
echo "[Step 6/11] Downloading proposer keystores..."
cd "${SCRIPTS_DIR}"
./get_kurtosis_keys.sh ${ENCLAVE_NAME} ${OUTPUT_DIR}

# Step 7: Fetch data from Kurtosis
echo ""
echo "[Step 7/11] Fetching genesis, JWT, and bootnode data..."
cd "${SCRIPTS_DIR}"
./get_kurtosis_data.sh ${ENCLAVE_NAME} ${DATA_DIR}

# Load the Bootnodes into environment variables (comma-separated)
export BOOTNODES_EL=$(grep -m 1 '^enode://' ${DATA_DIR}/genesis/bootnode.txt)
export BOOTNODES_CL=$(grep -m 1 '^enr:' ${DATA_DIR}/genesis/bootstrap_nodes.txt)
echo "BOOTNODES_EL=$BOOTNODES_EL"
echo "BOOTNODES_CL=$BOOTNODES_CL"

# Export network name for docker-compose
export KURTOSIS_NETWORK="kt-${ENCLAVE_NAME}"

# Fee recipient (can be overridden via environment)
export FEE_RECIPIENT="${FEE_RECIPIENT:-0x0000000000000000000000000000000000000000}"

# Step 8: Get the Kurtosis lighthouse container name, IP, and peer ID for direct libp2p connection
echo ""
echo "[Step 8/11] Fetching Kurtosis lighthouse peer info..."
CL_SERVICE="cl-1-lighthouse-geth"

if kurtosis service inspect ${ENCLAVE_NAME} ${CL_SERVICE} &>/dev/null; then
    # Get HTTP URL for the Lighthouse API (includes http:// prefix)
    CL_HTTP_URL=$(kurtosis port print ${ENCLAVE_NAME} ${CL_SERVICE} http)
    
    # Get peer ID and p2p address from Lighthouse HTTP API (more reliable than parsing logs)
    CL_IDENTITY=$(curl -s "${CL_HTTP_URL}/eth/v1/node/identity")
    CL_PEER_ID=$(echo "$CL_IDENTITY" | jq -r '.data.peer_id')
    
    # Get internal IP from --enr-address in the service config
    CL_IP=$(kurtosis service inspect ${ENCLAVE_NAME} ${CL_SERVICE} 2>/dev/null | grep -o '\-\-enr-address=[0-9.]*' | cut -d= -f2)
    
    # Use internal IP for container-to-container communication
    export LIBP2P_ADDR="/ip4/${CL_IP}/tcp/9000/p2p/${CL_PEER_ID}"
    export TRUSTED_PEER="${CL_PEER_ID}"
    
    echo "  CL Service: $CL_SERVICE"
    echo "  CL HTTP URL: $CL_HTTP_URL"
    echo "  CL Internal IP: $CL_IP"
    echo "  CL Peer ID: $CL_PEER_ID"
    echo "  Libp2p Address: $LIBP2P_ADDR"
    echo "  Trusted Peer: $TRUSTED_PEER"
else
    echo "  WARNING: Could not find Kurtosis lighthouse service '${CL_SERVICE}'"
    export LIBP2P_ADDR=""
fi

# Verify that LIBP2P_ADDR and TRUSTED_PEER are set
if [ -z "${LIBP2P_ADDR}" ] || [ -z "${TRUSTED_PEER}" ]; then
    echo ""
    echo "ERROR: LIBP2P_ADDR or TRUSTED_PEER is not set."
    echo "  LIBP2P_ADDR: '${LIBP2P_ADDR}'"
    echo "  TRUSTED_PEER: '${TRUSTED_PEER}'"
    echo "Failed to extract lighthouse peer info from Kurtosis. Exiting."
    exit 1
fi


# Step 9: Clean any previous fabric databases 
echo ""
echo "[Step 9/11] Cleaning any previous fabric databases (if they exist)..."
if [ -d "${OUTPUT_DIR}/rocksdb" ]; then
    rm -r "${OUTPUT_DIR}/rocksdb"
fi

# Step 10: Auto generate the configs
echo ""
echo "[Step 10/11] Auto generating the configs..."
cd "${REPO_ROOT}" && just setup-docker-simulation 

# Step 11: Start the fabric services
echo ""
echo "[Step 11/11] Starting the fabric services via Docker Compose..."
cd "${CONFIG_DIR}"
docker compose up -d

echo ""
echo "========================================"
echo "  Setup Complete!"
echo "========================================"
echo ""
echo "Kurtosis services:"
echo "  kurtosis enclave inspect ${ENCLAVE_NAME}"
echo ""
echo "External services:"
echo "  docker compose logs -f"
echo ""
echo "Block explorer:"
echo "  $(kurtosis port print ${ENCLAVE_NAME} dora http)"
echo ""