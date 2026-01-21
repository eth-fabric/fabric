# Fabric

This repo contains an end-to-end reference implementation of L1 inclusion preconfs. It's meant to serve as a stepping stone to simplify working with Fabric's Constraints API and Commitments API specs. This is not audited and subject to change, use at your own risk!

## Reference implementation
- **Gateway**: 
  - Hosts a `GatewayRpc` server that implements the Commitments spec  and specifically accepts commitment requests for `InclusionPayload` types.
  - Runs a `DelegationManager` task that periodically queries the relay using its `HttpConstraintsClient` to fetch any upcoming delegations.
  - Runs a `ConstraintManager` task that posts `SignedConstraints` to the relay right before the target slot. 
- **Relay**: 
  - Hosts a `RelayServer` that implements the Constraints spec. To avoid fully re-implementing a relay from scratch, all non-constraints spec calls are proxied and passed to a configured downstream relay.
  - Runs a `LookaheadManager` task that tracks the beacon chain lookahead to know if a `SignedDelegation` is valid for a slot.
- **Proposer**:
  - Runs a `DelegationManager` task that signs tracks the beacon chain lookahead. If their proposer key is scheduled, they will post a `SignedDelegation` to the relay via their `HttpConstraintsClient`.
- **Constraints Builder**:
  - Runs a modified rbuilder that appends inclusion preconf transactions to the bottom of the block.

#### Implementation Opinions
- Relay only accepts one valid `SignedDelegation` per slot
- Relay only accepts one valid `SignedConstraints` per slot
- After the target slot has elapsed, the relay no longer enforces the  whitelist for `GET /constraints`

## Crate Structure

### Binaries (`bin/`)
- **`gateway.rs`** - Launches the `GatewayRpc`, `DelegationManager`, and `ConstraintManager`.
- **`proposer.rs`** - Launches the `DelegationManager` 
- **`relay.rs`** - Launches the `RelayServer` and `LookaheadManager`.
- **`spammer.rs`** - Spams the `GatewayRpc` with inclusion commitments requests
- **`local-signer-module.rs`** - Local Commit-Boost signing module for development and testing
- **`beacon-mock.rs`** - Mock beacon node that exposes the lookahead for local testing
- **`simulation-setup.rs`** - Generates config and .env files for the above binaries from a central config file for consistency.


### Crates (`crates/`)
- **`commitments/`** - Commitments API implementation
  - types
  - JSON-RPC client implementation
  - JSON-RPC server trait 

- **`common/`** - Shared infrastructure
  - minimal DB lib (RocksDB)

- **`constraints/`** - Constraints API implementation
  - types
  - rest client implementation
  - rest server trait 
  
- **`lookahead/`** - Beacon chain utils
  - minimal beacon node client implementation
  - slot timing utils

- **`signing/`** - BLS/ECDSA signing utils
  - minimal wrapper around the Commit-Boost `SignerClient`

- **`proposer/`** - Proposer delegation module
  - service to sign delegations based on lookahead 

- **`urc/`** - Universal Registry Contract utils
  - bindings for the URC contract
  - hashing utils for signing data s.t., URC contracts can verify
  - Task coordinator

- **`inclusion/`** - Reference implementation of inclusion preconfs
  - types
  - merkle inclusion proof utils
  - gateway implementation
  - relay implementation

## Usage

### Prerequisites

- **Rust** (stable) - Install from [rustup.rs](https://rustup.rs/)
- **Just** - Task runner (`brew install just` on macOS)
- **Foundry** - For URC contract bindings (optional, only if modifying contracts)
- **Kurtosis CLI** - See [installation guide](https://docs.kurtosis.com/install)

#### Update the submodules
```bash
git submodule init
git submodule update
```

#### Building URC Bindings (Optional)

If you need to regenerate the Rust bindings for URC contracts:

```bash
# Install Foundry if not already installed
curl -L https://foundry.paradigm.xyz | bash
foundryup

# Initialize submodules
git submodule update --init --recursive

# Generate bindings
cd ./vendor/urc
forge bind --crate-name urc --overwrite --bindings-path ../../crates/urc/src/bindings
```

### Building
The following commands build the Docker images expected by Kurtosis.

```bash
export VERSION=dev

# Build local Commit-Boost Signer Module Docker image
just build-signer $VERSION

# Build Gateway Docker image
just build-signer $VERSION

# Build Relay Docker image
just build-signer $VERSION

# Build Proposer Module Docker image
just build-proposer $VERSION

# Build Spammer Module Docker image
just build-spammer $VERSION

# Build Constraints Builder Docker image
just build-builder $VERSION
```

### Running
#### Kurtosis setup
The Kurtosis config is defined at `./kurtosis/config/kurtosis-network-params.yaml`. By default, it launches two full nodes, each running two validators, a MEV-Boost relay, a transaction spammer, and blocker explorer utilities. 

The `./kurtosis/config/docker-compose.yml` and `./kurtosis/scripts` files work together so that the Kurtosis network members can communicate with the Dockerized fabric members, i.e., so the Constraint Builder's node can sync with the network.

#### Launching Kurtosis
The following will launch the Kurtosis network with all of the Fabric services locally. It's recommended that your computer has sufficient disk space.

```bash
# Tears down and relaunches everything
just restart-testnet          

# Stops everything
just stop-testnet

# Only restarts the Docker containers
# (Useful for developing)
just restart-testnet-docker

# Prints the URLs of every Kurtosis member
just inspect-testnet

# Prints the URL of the block explorer
just block-explorer
```

#### Some notes
- It takes a while (dozens of slots) for the MEV-Boost Relay to supply the Constraints Builder with registration data. This means it'll be delayed before posting blocks.
- If any Kurtosis services crash it's recommended to restart everything `just restart-testnet`
- The Spammer service will only update it's nonce after its inclusion transcation lands on-chain. Since inclusion requests happen earlier than the block inclusion, expect to see some errors like: `"Failed to submit blocks with proofs: Constraint types length mismatch, received 0 constraints, expected 1`. The reason is the builder will exclude transactions with failed nonces by default. Once the inclusion lands on-chain, the Spammer will update the nonce and the simulation will progress.

## Specifications

This project implements the following Ethereum preconfirmation specifications:

- [Commitments API](https://github.com/eth-fabric/commitments-specs/blob/main/specs/commitments-api.md) - User-facing API for requesting preconfirmations
- [Constraints API](https://github.com/eth-fabric/constraints-specs/blob/main/specs/constraints-api.md) - Builder/relay coordination API
- [Gateway Specification](https://github.com/eth-fabric/constraints-specs/blob/main/specs/gateway.md) - Gateway architecture and behavior

## License

See [LICENSE](LICENSE) file for details.
