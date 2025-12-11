# Fabric

This repo provides rust components to simplify working with Fabric's Constraints API and Commitments API specs as well as a reference implementation of L1 inclusion preconfs. This is not audited and subject to change, use at your own risk!



## Reference implementation
This repo contains a reference implementation of a Gateway, Relay, and Proposer, all coordinating to issue L1 inclusion preconfs. 

- **Gateway**: 
  - Hosts a `GatewayRpc` server that implements the Commitments spec  and specifically accepts commitment requests for `InclusionPayload` types.
  - Runs a `DelegationManager` task that periodically queries the relay using its `HttpConstraintsClient` to fetch any upcoming delegations.
  - Runs a `ConstraintManager` task that posts `SignedConstraints` to the relay right before the target slot. 
- **Relay**: 
  - Hosts a `RelayServer` that implements the Constraints spec. To avoid fully re-implementing a relay from scratch, all non-constraints spec calls are proxied and passed to a configured downstream relay.
  - Runs a `LookaheadManager` task that tracks the beacon chain lookahead to know if a `SignedDelegation` is valid for a slot.
- **Proposer**:
  - Runs a `DelegationManager` task that signs tracks the beacon chain lookahead. If their proposer key is scheduled, they will post a `SignedDelegation` to the relay via their `HttpConstraintsClient`.

**Implementation Opinions**
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

- **`urc/`** - Universal Registry Contract utils
  - bindings for the URC contract
  - hashing utils for signing data s.t., URC contracts can verify
  - Task coordinator

- **`inclusion/`** - Reference implementation of inclusion preconfs
  - types
  - merkle inclusion proof utils
  - gateway implementation
  - proposer implementation
  - relay implementation

## Local Development

### Prerequisites

- **Rust** (stable) - Install from [rustup.rs](https://rustup.rs/)
- **Just** - Task runner (`brew install just` on macOS)
- **Foundry** - For URC contract bindings (optional, only if modifying contracts)

### Building

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

### Running Locally

#### 1. Setup Simulation Configs

Generate the config and .env files for the binaries

```bash
just setup-simulation
```

#### 2. Start Services

Run each service in a separate terminal:

```bash
# Terminal 1: Signer module
just run-local-signer

# Terminal 2: Mock beacon node
just run-local-beacon-mock

# Terminal 3: Relay
just run-local-relay

# Terminal 4: Proposer
just run-local-proposer

# Terminal 5: Gateway
just run-local-gateway

# Terminal 6: Spammer
just run-local-spammer
```

## Docker Deployment

### Prerequisites

- Docker Desktop (for macOS/Windows) or Docker Engine (for Linux)
- `just` command runner (`brew install just`)

### Building Docker Images

Build all service images with a version tag:

```bash
# Build all images with 'dev' tag
just build-all dev

# Or build specific services individually
just build-gateway dev
just build-proposer dev
just build-relay dev
just build-spammer dev
just build-signer dev
just build-beacon-mock dev
```

Images are tagged as `fabric/<service>:<version>`.

### Running with Docker Compose

#### Setup Docker Simulation Configs

Generate the config and .env files for the containers

```bash
just setup-docker-simulation
```

#### Run containers

```bash
just up
```

## Specifications

This project implements the following Ethereum preconfirmation specifications:

- [Commitments API](https://github.com/eth-fabric/commitments-specs/blob/main/specs/commitments-api.md) - User-facing API for requesting preconfirmations
- [Constraints API](https://github.com/eth-fabric/constraints-specs/blob/main/specs/constraints-api.md) - Builder/relay coordination API
- [Gateway Specification](https://github.com/eth-fabric/constraints-specs/blob/main/specs/gateway.md) - Gateway architecture and behavior

## License

See [LICENSE](LICENSE) file for details.
