# LumenSignal-Backend

A Web3-powered sports engagement platform integrating live football data with Soroban smart contracts on the Stellar network.

[![Stellar](https://img.shields.io/badge/Stellar-Blockchain-black)](https://stellar.org)
[![Soroban](https://img.shields.io/badge/Soroban-Smart_Contracts-black)](https://soroban.stellar.org)
[![Rust](https://img.shields.io/badge/Rust-Language-orange)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
![Version](https://img.shields.io/badge/version-0.0.1-blue)
![CI](https://img.shields.io/badge/CI-passing-brightgreen)

## Project Overview

LumenSignal is a decentralized application bridging Web2 sports data with Web3 blockchain capabilities. It enables users to place bets on live football matches, participate in gamified prediction leaderboards, stake tokens for yields, and collect NFT player cards. The system interacts seamlessly with on-chain Soroban smart contracts to guarantee transparent bet settlement, escrow, and reward distribution.

This repository encompasses the entire platform architecture, including a modular API backend and the smart contract source code.

This project is built upon the following technologies:
* **Stellar & Soroban Smart Contracts**: For trustless bet settlement, staking, and NFT tracking, written in Rust.
* **TypeScript & NestJS**: The core backend framework orchestrating all data flows and APIs.
* **PostgreSQL (TypeORM)**: Primary relational database for user profiles, matches, and application state.
* **Redis**: Used for high-speed caching and rate-limiting.

## Table of Contents

* [Project Overview](#project-overview)
* [Key Features](#key-features)
* [Technology Stack](#technology-stack)
* [Architecture](#architecture)
* [Project Structure](#project-structure)
* [Prerequisites](#prerequisites)
* [Installation](#installation)
* [Environment Variables](#environment-variables)
* [Running the Project](#running-the-project)
* [Testing](#testing)
* [API](#api)
* [Smart Contracts](#smart-contracts)
* [Database / Data Storage](#database--data-storage)
* [Security](#security)
* [Background Services / Workers](#background-services--workers)
* [Deployment](#deployment)
* [License](#license)

## Key Features

* **Live Match Tracking**: Fetches and serves live football data, fixtures, and odds.
* **On-Chain Betting & Escrow**: Allows users to place bets that are escrowed and settled automatically via Soroban smart contracts.
* **NFT Marketplace & Player Cards**: Mints and trades digital player cards.
* **Staking & Treasury Management**: Users can stake tokens for yields, backed by an on-chain treasury.
* **Gamification & Leaderboards**: Ranks users based on predictions and awards free bet vouchers and spin-to-win mechanics.
* **Wallet Linking**: Securely links Stellar wallets (like Freighter) to user accounts.
* **Administrative Controls**: Comprehensive admin panels for auditing, role management, solvency monitoring, and fraud detection.

## Technology Stack

* **Blockchain**: Stellar Network, Soroban Smart Contracts
* **Smart Contracts**: Rust, Soroban SDK v20.5.0, Solidity (Legacy/Fractional NFTs)
* **Backend**: Node.js, NestJS v11, TypeScript
* **Database**: PostgreSQL (via TypeORM)
* **Caching**: Redis (via ioredis and Cache Manager)
* **Testing**: Jest

## Architecture

The backend operates as a central oracle and orchestration layer. It exposes RESTful APIs to client applications while fetching real-world sports data (via RapidAPI). Based on match outcomes, the backend triggers signed transactions to Soroban smart contracts to settle bets, release escrowed funds, and distribute rewards to users' Stellar wallets.

## Project Structure

```text
LumenSignal-Backend/
├── backend/                  # NestJS API Backend
│   ├── src/                  # Source code (auth, bets, blockchain, etc.)
│   ├── test/                 # E2E Tests
│   ├── scripts/              # Database setup scripts
│   ├── package.json          # Node dependencies
│   └── nest-cli.json         # NestJS configuration
├── contract/                 # Soroban Smart Contracts
│   ├── contracts/            # Rust & Solidity contract source code
│   │   ├── balance_ledger/   # On-chain balances
│   │   ├── betting/          # Betting escrow logic
│   │   ├── settlement/       # Settlement resolution
│   │   └── ...               # (staking, nft_player_cards, treasury, etc.)
│   ├── Cargo.toml            # Rust workspace configuration
│   └── Makefile              # Build automation
├── docs/                     # Documentation files
└── package.json              # Workspace root package configuration
```

## Prerequisites

Ensure you have the following installed:
* **Node.js** (v18+ recommended)
* **pnpm** (Package manager)
* **Rust & Cargo** (For smart contracts)
* **Soroban CLI** (`cargo install --locked soroban-cli`)
* **PostgreSQL**
* **Redis**

## Installation

1. Clone the repository:
   ```bash
   git clone https://github.com/LumenSignal/LumenSignal-Backend.git
   cd LumenSignal-Backend
   ```

2. Install backend dependencies:
   ```bash
   cd backend
   pnpm install
   ```

3. Install smart contract dependencies:
   ```bash
   cd ../contract
   make install-deps
   ```

## Environment Variables

Create a `.env` file in the `backend/` directory based on `.env.example`. 

**Important Configuration Keys:**
* `DB_*`: PostgreSQL credentials and host configuration.
* `JWT_SECRET` & `JWT_EXPIRES_IN`: Authentication security keys.
* `REDIS_*`: Caching backend configuration.
* `STELLAR_NETWORK`, `STELLAR_RPC_URL`: Stellar blockchain connectivity.
* `SOROBAN_*_CONTRACT_ID`: Addresses of deployed smart contracts.
* `SOROBAN_ADMIN_SECRET`: Admin wallet secret for executing authorized operations.
* `RAPIDAPI_FOOTBALL_KEY`: API key for fetching real-time match data.
* `SMTP_*`: Credentials for email notifications.

> **Security Note:** Never commit your `.env` file or expose your `SOROBAN_ADMIN_SECRET` in source control.

## Running the Project

### Database Setup
Run the following from the `backend/` directory to initialize the database:
```bash
pnpm run db:setup
pnpm run schema:sync
```

### Starting the Backend
```bash
# Development mode
pnpm run start:dev

# Production build
pnpm run build
pnpm run start:prod
```

### Compiling Smart Contracts
Run the following from the `contract/` directory:
```bash
# Build all contracts for WebAssembly
make build

# Optimize WASM files for deployment
make optimize
```

## Testing

**Backend Tests (Jest):**
```bash
cd backend
pnpm run test        # Unit tests
pnpm run test:e2e    # End-to-end tests
pnpm run test:cov    # Coverage report
```

**Smart Contract Tests (Cargo):**
```bash
cd contract
make test            # Run all contract tests
```

## API

The backend exposes numerous RESTful endpoints. Key resource paths include:

* **Authentication**: `POST /auth/...`
* **Wallets**: `GET /wallet/connections`, `POST /wallet/admin/...`
* **Matches & Teams**: `GET /matches`, `GET /teams`
* **Betting**: `POST /bets`, `POST /bets/parlay`
* **Settlement**: `POST /blockchain/settlement`
* **NFT Marketplace**: `GET /nft`, `POST /nft/...`
* **Staking & Treasury**: `POST /staking`, `GET /treasury`
* **Leaderboards**: `GET /leaderboards`

For full interactive API documentation (Swagger), start the backend and visit:
`http://localhost:3000/api/docs`

## Smart Contracts

The project utilizes modular Soroban smart contracts written in Rust:

* **`betting` & `betting_escrow`**: Manages wager lockups and conditional payouts.
* **`settlement`**: Receives trusted outcomes from the backend oracle to disburse funds.
* **`balance_ledger`**: Maintains an on-chain ledger of user deposit and withdrawal balances.
* **`staking` & `treasury`**: Handles yield generation, locking durations, and protocol reserves.
* **`nft_player_cards`**: Manages the minting, ownership, and trading of unique footballer NFTs.
* **`lumensignal_spin_to_win`**: On-chain logic for randomized rewards and gamification.

**Deployment**:
Contracts can be deployed using the automated scripts inside `contract/deployment` or via Makefile targets:
```bash
make deploy-all
```

## Database / Data Storage

The backend leverages **PostgreSQL** with **TypeORM** for robust data modeling. Key domains managed in the relational schema include:
* Users, Roles, and Authentication states
* Football Teams, Players, Matches, and Odds
* Bets, Parlays, and Transaction histories
* Leaderboards, Seasons, and User rankings

## Security

The platform implements several verifiable security mechanisms:
* **Authentication**: JWT-based stateless authentication (`@nestjs/jwt`, `@nestjs/passport`).
* **Rate Limiting**: Configurable API rate limiting to prevent abuse (`@nestjs/throttler`).
* **On-Chain Reentrancy Protection**: Smart contracts utilize explicit state locks (e.g., `TREASURY_LOCK`) to prevent reentrancy attacks.
* **Fraud Detection**: Administrative modules dedicated to monitoring and freezing suspicious activities (`fraud.controller.ts`).

## Background Services / Workers

The backend utilizes `@nestjs/schedule` to run automated background jobs, including:
* Fetching live sports odds and updating match statuses.
* Processing automated bet settlements once matches conclude.
* Sending batch email or push notifications.
* Performing routine solvency checks on the treasury.

## Deployment

The backend application can be compiled and deployed via Docker (using standard Node images) or any traditional Node.js hosting platform. 
Smart contracts must be compiled to `.wasm`, optimized, and deployed to the Stellar network using the `soroban-cli` tooling provided in the `Makefile`.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
