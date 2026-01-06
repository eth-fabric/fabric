#!/bin/bash

# Stop existing fabric containers
docker compose down

# Clean up any existing Kurtosis networks
kurtosis clean -a

# Run the Kurtosis network
kurtosis run --enclave preconf-testnet \
  github.com/ethpandaops/ethereum-package \
  --args-file ./kurtosis-network-params.yaml \
  2>&1 | tee /tmp/kurtosis-deploy.log
  
# Extract the ports from the Kurtosis network and update the configuration files
python3 extract_ports.py \
  --enclave preconf-testnet \
  --fabric-config ../config/docker.config.toml \
  --rbuilder-config ../../constraints_builder/docker/config/constraints-builder-docker-config.toml

# Update the chain spec in the fabric config
./update_chain_line.sh preconf-testnet ../config/docker.config.toml

# Make a local copy of the reth chainspec for the rbuilder
docker cp $(docker ps -qf "name=el-2-reth-builder-lighthouse"):/network-configs/genesis.json /tmp/reth-genesis.json

# Download a copy of the proposer keystores
kurtosis files download preconf-testnet 1-lighthouse-geth-0-3 /tmp/keystores

# Get the volume name from the kurtosis builder's reth
export RETH_VOLUME=$(docker volume ls -q | grep el-2-reth-builder-lighthouse)

# Clean any previous fabric databases
rm -r /tmp/rocksdb

# Setup the configs
cd .. && just setup-docker-simulation 

# Run the containers
cd kurtosis && docker compose up -d