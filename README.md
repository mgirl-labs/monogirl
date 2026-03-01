<p align="center">
  <img src="./banner.png" alt="monogirl banner" width="100%" />
</p>

<h1 align="center">monogirl</h1>

<p align="center"><strong>Conditional Parallel Execution on Solana. The Anti-Jito.</strong></p>

<p align="center"><em>Proof of synchronicity.</em></p>

<p align="center">
  <a href="https://github.com/mgirl-labs/monogirl/actions/workflows/ci.yml">
    <img src="https://img.shields.io/github/actions/workflow/status/mgirl-labs/monogirl/ci.yml?branch=main&style=flat-square&label=CI" alt="CI Status" />
  </a>
  <a href="https://github.com/mgirl-labs/monogirl">
    <img src="https://img.shields.io/github/stars/mgirl-labs/monogirl?style=flat-square" alt="Stars" />
  </a>
  <a href="https://github.com/mgirl-labs/monogirl/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License" />
  </a>
  <img src="https://img.shields.io/badge/solana-mainnet-green?style=flat-square" alt="Solana" />
  <img src="https://img.shields.io/badge/version-0.4.2-orange?style=flat-square" alt="Version" />
  <a href="https://x.com/mgirl_fun">
    <img src="https://img.shields.io/badge/X-@mgirl__fun-black?style=flat-square&logo=x&logoColor=white" alt="X" />
  </a>
  <a href="https://mgirl.fun">
    <img src="https://img.shields.io/badge/website-mgirl.fun-dc2626?style=flat-square" alt="Website" />
  </a>
</p>

<hr />

## What is Conditional Parallel Execution

Jito guarantees sequential ordering: transaction A runs before B. MonoGirl guarantees the opposite -- parallel ordering: transactions A and B execute in the same parallel batch on Sealevel.

CPE bundles specify a set of transactions along with the conditions under which they must run concurrently. The on-chain program validates account set independence, schedules parallel batches, and generates a proof of synchronous execution.

| | Jito | MonoGirl |
|---|---|---|
| Ordering | Sequential (A then B) | Parallel (A and B together) |
| Use case | MEV extraction | MEV protection, atomic parallelism |
| Execution model | Ordered bundles | CPE bundles |
| Token | JTO | $MONO |

## Architecture

```mermaid
flowchart TB
    subgraph Client["TypeScript SDK"]
        style Client fill:#f5f5f5,stroke:#1a1a1a,color:#1a1a1a
        A[CPE Bundle Builder] --> B[Parallel Validator]
        B --> C[Bundle Submitter]
    end

    subgraph OnChain["Solana Program"]
        style OnChain fill:#1a1a1a,stroke:#dc2626,color:#ffffff
        D[monogirl_core] --> E[CPE Scheduler]
        E --> F[Proof Generator]
        F --> G[Epoch Tracker]
        D --> H[Conflict Resolver]
    end

    subgraph Execution["Parallel Execution Layer"]
        style Execution fill:#ffffff,stroke:#dc2626,color:#1a1a1a
        I[Account Set Analyzer] --> J[Dependency Graph]
        J --> K[Batch Scheduler]
    end

    C -->|"Submit CPE Bundle"| D
    B -->|"Validate Accounts"| I
    G -->|"Epoch Data"| A
    K -->|"Execution Plan"| E
```

## Usage

### Submitting a CPE Bundle

```typescript
import { CPEClient, BundleConfig } from "./monogirl-sdk";

const client = new CPEClient({
  rpcUrl: "https://api.mainnet-beta.solana.com",
  programId: "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS",
});

const bundle = await client.createBundle({
  transactions: txList,
  epoch: currentEpoch,
  maxDepth: 8,
});

const result = await client.submitBundle(bundle);
console.log("CPE proof:", result.signature);
```

### CLI Usage

```bash
# Create a CPE bundle from transaction set
monogirl-cli bundle --input transactions.json --epoch 420

# Verify parallel execution proof
monogirl-cli verify --proof proof.json

# Inspect dependency graph
monogirl-cli inspect --dag-output dag.dot
```

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on how to submit pull requests.

## Installation

### Prerequisites

- Rust 1.75+ with cargo
- Node.js 18+ with npm
- Solana CLI 1.17+
- Anchor Framework 0.29+

### Clone and Build

```bash
git clone https://github.com/mgirl-labs/monogirl.git
cd monogirl

# Build Rust programs
cargo build --release

# Install SDK dependencies
cd sdk && npm install
```

### Run Tests

```bash
# Rust unit tests
cargo test

# TypeScript SDK tests
cd sdk && npm test
```

## Features

| Feature | Description |
|---------|-------------|
| CPE Bundle Validation | Validates account set independence for parallel execution eligibility |
| Parallel Batch Scheduling | Assigns transactions to Sealevel parallel batches based on dependency graphs |
| Proof of Synchronicity | Generates on-chain proof that transactions executed in the same slot and batch |
| Epoch-Aware Processing | Tracks Solana epochs for proof validity windows |
| Conflict Resolution | Deterministic resolution when account sets overlap |
| TypeScript SDK | Client library for CPE bundle submission and proof querying |
| CLI Tools | Command-line interface for bundle creation and inspection |

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.


