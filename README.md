# EchoMarkets 🔊
### High-Frequency Social Prediction Markets on Linera

**EchoMarkets** is a real-time, agentic prediction market platform built on the **Linera Protocol**. It leverages Linera's microchain architecture to enable **instant copy-trading** without gas wars or latency bottlenecks.

## 🚀 The Problem
Traditional blockchains (Ethereum, Solana) force all trades into a global queue. During high-volatility events:
1.  **Latency spikes:** Copy-trading signals arrive too late.
2.  **Front-running:** MEV bots steal value from followers.
3.  **Congestion:** Gas fees make micro-bets impossible.

## ⚡ The Solution: EchoMarkets
We utilize Linera's **Cross-Chain Messaging** to create a "Pulse" network:
*   **Leader Chains:** Pro traders or AI Agents broadcast trade signals.
*   **Follower Chains:** Thousands of user microchains subscribe to these signals and execute trades **in parallel** on their own local chains.

**Result:** 10,000 users can copy-trade a signal in <50ms, with 0 gas wars.

## 🛠️ Architecture

### 1. The Contract (`src/contract.rs`)
*   `CreateMarket`: Initializes a binary option market.
*   `PlaceBet`: Updates the local market state.
*   `Subscribe`: Registers a user's chain ID with a Trader's chain.
*   **`TradeSignal` (The Innovation):** A cross-chain message sent from Leader -> Subscriber that triggers an *atomic, auto-executed* bet on the subscriber's chain.

### 2. The Service (`src/service.rs`)
*   Exposes a GraphQL API for the frontend to fetch market stats and trader profiles in real-time.

### 3. The Frontend (`web/`)
*   A Cyberpunk-style React dashboard visualizing the low-latency "Network Pulse".
*   Demonstrates the "Auto-Copy" UX where signals are received and executed instantly.

## 📦 How to Run

### Prerequisites
*   Linera Toolchain (v0.15.6)
*   Node.js & NPM
*   Rust & Cargo

### Backend (Contract)
```bash
cd echo-markets
cargo build --release --target wasm32-unknown-unknown
# (Optional) Run tests
cargo test
```

### Frontend (Demo)
```bash
cd web
npm install
npm run dev
```

## 🏆 Hackathon Tracks
*   **Real-Time Markets:** We demonstrate sub-second trade execution.
*   **Market Infrastructure:** We provide the "Social Layer" for prediction markets.

---
*Built for the Linera Wave 3 Buildathon.*
