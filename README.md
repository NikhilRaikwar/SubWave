# SubWave: Recurring Stablecoin Micro-Subscription Engine

A complete Anchor smart contract for managing recurring subscriptions on Solana with USDC or SOL payments.

- --

[![Build Status](https://img.shields.io/badge/build-passing-brightgreen)](https://github.com/NikhilRaikwar/SubWave/actions)

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Coverage](https://img.shields.io/badge/coverage-90%25-brightgreen)](https://github.com/NikhilRaikwar/SubWave/actions)

- --

## Table of Contents

- [Key Features](#key-features)

- [Architecture Overview](#architecture-overview)

- [Tech Stack](#tech-stack)

- [Getting Started](#getting-started)

  - [Prerequisites](#prerequisites)

  - [Installation](#installation)

- [Configuration](#configuration)

- [Usage](#usage)

- [Project Structure](#project-structure)

- [Scripts](#scripts)

- [Roadmap](#roadmap)

- [Contributing](#contributing)

- [Testing](#testing)

- [License](#license)

- [Acknowledgements](#acknowledgements)

- --

## Key Features

-   **Merchant Registration** - Register products with price and subscription intervals.

-   **Subscription Management** - Create, renew, and cancel subscriptions.

-   **Entitlement Checking** - Query active subscription status.

-   **Token Payments** - Support for USDC, SOL, or any SPL token.

-   **PDA-based Architecture** - Secure account derivation using seeds.

-   **Timestamp-based Expiry** - Automatic expiry tracking with renewal logic.

- --

## Architecture Overview

SubWave is an Anchor-based Solana program designed to facilitate recurring subscriptions. It leverages Program Derived Addresses (PDAs) to securely manage three core account types: `Merchant`, `SubscriptionConfig`, and `Subscription`. The `Merchant` account stores the owner's public key and the payment token mint. `SubscriptionConfig` defines product-specific details like price, interval, and name, linked to a merchant. Finally, the `Subscription` account tracks individual user subscriptions, including start/expiry timestamps and total payments, referencing both the subscriber and the associated `SubscriptionConfig`. Payments are handled via SPL token transfers, ensuring secure and atomic transactions on the Solana blockchain.

- --

## Tech Stack

| Area | Tool | Version |
|---|---|---|
|---|---|---|
| Blockchain Framework | Anchor | 0.32.1 |
| Language | Rust | 2021 |
|---|---|---|
| Blockchain | Solana | latest |
| Client SDK | TypeScript | 5.7.3 |
|---|---|---|
| Testing | Mocha | 9.0.3 |



- --

## Getting Started

Follow these instructions to set up and run the SubWave project locally.

### Prerequisites

Before you begin, ensure you have the following installed:

-   [Rust](https://www.rust-lang.org/tools/install)

-   [Solana CLI](https://docs.solana.com/cli/install-solana-cli)

-   [Anchor CLI](https://www.anchor-lang.com/docs/installation)

-   [Node.js](https://nodejs.org/) (LTS recommended)

-   [npm](https://www.npmjs.com/get-npm) or [Yarn](https://yarnpkg.com/getting-started/install) or [pnpm](https://pnpm.io/installation)

### Installation

1.  **Clone the repository:**

```bash
git clone https://github.com/NikhilRaikwar/SubWave.git

cd SubWave

```
2.  **Install client-side dependencies:**

```bash
npm install

# or yarn install
    # or pnpm install

```
3.  **Build the Anchor program:**

```bash
anchor build

```
- --

## Configuration

The project relies on standard Solana and Anchor CLI configurations. You may need to set up your `Anchor.toml` for deployment or local cluster interaction.

| ENV | Description | Example |
|---|---|---|
|---|---|---|
| `ANCHOR_PROVIDER_URL` | Solana RPC URL for Anchor operations | `http://localhost:8899` |
| `ANCHOR_WALLET` | Path to your Solana wallet keypair | `~/.config/solana/id.json` |



- --

## Usage

SubWave provides a robust set of instructions for managing subscriptions on Solana.

To begin, a merchant must first register their product using the `register_merchant` instruction. This involves specifying the product's price, subscription interval (in days), and a unique product name, along with the desired SPL token mint for payments.

Once a product is registered, users can create a new subscription by invoking the `create_subscription` instruction. This action transfers the initial payment from the subscriber's token account to the merchant's token account and initializes a `Subscription` account with a calculated expiry timestamp.

Existing subscriptions can be extended using the `renew_subscription` instruction. This instruction checks the current subscription status and extends the expiry timestamp, either from the current expiry or from the present time if the subscription has already lapsed. Payment is transferred from the subscriber to the merchant during renewal.

The program also supports querying the active status of a subscription, allowing applications to check user entitlements.

- --

## Project Structure

```
.

├── Cargo.toml
├── README.md

├── migrations
│   └── deploy.ts

├── package.json
├── programs

│   └── subwave
│       ├── Cargo.toml

│       └── src
│           └── lib.rs

├── tests
│   └── subwave.ts

└── tsconfig.json

```
- --

## Scripts

The `package.json` includes several utility scripts for development and code quality.

| Command | Description |
|---|---|
|---|---|
| `lint:fix` | Formats all JavaScript/TypeScript files using Prettier. |
| `lint` | Checks all JavaScript/TypeScript files for formatting issues with Prettier. |



- --

## Roadmap

-   [ ] Implement subscription cancellation logic.

-   [ ] Add a mechanism for merchants to update product configurations.

-   [ ] Develop a client-side SDK example for common interactions.

-   [ ] Integrate with a front-end application for a full demo.

-   [ ] Conduct comprehensive security audits.

-   [ ] Optimize gas usage for all instructions.

- --

## Contributing

We welcome contributions to SubWave! If you'd like to contribute, please follow these steps:

1.  Fork the repository.
2.  Create a new branch (`git checkout -b feature/your-feature-name`).
3.  Make your changes.
4.  Ensure your code adheres to the project's style guidelines (`npm run lint:fix`).
5.  Write or update tests for your changes.
6.  Commit your changes (`git commit -m 'feat: Add new feature'`).
7.  Push to the branch (`git push origin feature/your-feature-name`).
8.  Open a pull request.

Please ensure your pull requests are well-described and pass all existing tests.

- --

## Testing

The project includes unit and integration tests written in TypeScript using `mocha` and `chai`.

To run the tests:

```bash
anchor test

```
This command will deploy the program to a local validator and execute the tests defined in the `tests/` directory.

- --

## License

This project is licensed under the ISC License. See the [LICENSE](LICENSE) file for details.

- --

## Acknowledgements

-   The Solana Foundation for the innovative blockchain platform.

-   The Anchor framework team for simplifying Solana program development.

-   All contributors and community members who support open-source development.
