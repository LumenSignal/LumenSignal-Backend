# LumenSignal Soroban Smart Contracts

This directory contains the Soroban smart contracts for the LumenSignal betting platform.

## 📁 Project Structure

```
contract/
├── Cargo.toml              # Workspace configuration
├── Makefile                # Build and test automation
├── contracts/
│   ├── common/             # Shared types and utilities
│   │   ├── src/
│   │   │   ├── lib.rs      # Main exports
│   │   │   ├── enums.rs    # Shared enums (BetStatus, BetOutcome)
│   │   │   ├── errors.rs   # Contract error definitions
│   │   │   └── events.rs   # Event type definitions
│   │   └── Cargo.toml
│   ├── settlement/         # Settlement contract
│   │   ├── src/
│   │   │   ├── lib.rs      # Contract implementation
│   │   │   └── test.rs     # Unit tests
│   │   └── Cargo.toml
│   ├── betting/            # Betting/Escrow contract
│   │   ├── src/
│   │   │   ├── lib.rs      # Contract implementation
│   │   │   └── test.rs     # Unit tests
│   │   └── Cargo.toml
└── target/                 # Build artifacts
```

## 🏗️ Contracts

### Common Crate
Provides shared functionality used across all contracts:
- **Enums**: `BetStatus`, `BetOutcome`
- **Errors**: `ContractError` with standardized error codes
- **Events**: Contract event type definitions for bet lifecycle

### Settlement Contract
Handles the on-chain settlement of bets after match results are determined.

### Betting Contract
Manages bet creation, escrow, and bet lifecycle management.


## 📚 Additional Resources

- [Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Documentation](https://developers.stellar.org/)
- [Soroban Examples](https://github.com/stellar/soroban-examples)
