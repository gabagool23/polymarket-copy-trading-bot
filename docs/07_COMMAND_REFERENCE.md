# Command Reference Guide

Complete reference for all available commands in the Polymarket Copy Trading Bot (Rust).

## Table of Contents

1. [Setup & Configuration](#setup--configuration)
2. [Main Bot](#main-bot)
3. [Wallet Management](#wallet-management)
4. [Monitoring & Debugging](#monitoring--debugging)
5. [Testing & Validation](#testing--validation)
6. [Quick Reference](#quick-reference)

---

## Setup & Configuration

### System Status (Validate Configuration)

```bash
cargo run --release setup system-status
```

**Purpose:** Verify your configuration is correct before running the bot. This command validates configuration, checks balances, and tests connectivity.

**What it checks:**
- ✅ `PRIVATE_KEY` format (64 hex characters, no 0x prefix)
- ✅ `FUNDER_ADDRESS` format (40 hex characters)
- ✅ `TARGET_WHALE_ADDRESS` format (40 hex characters, no 0x prefix)
- ✅ `ALCHEMY_API_KEY` or `CHAINSTACK_API_KEY` is set
- ✅ Required environment variables are present
- ✅ Address format validation

**When to use:**
- After setting up `.env` file
- Before starting the bot
- After changing configuration
- Troubleshooting setup issues

**Example Output:**

```
✅ Configuration validation passed!
✅ PRIVATE_KEY: Valid (0xA35d8c52...)
✅ FUNDER_ADDRESS: Valid Gnosis Safe (0x2108FF2b...)
✅ TARGET_WHALE_ADDRESS: Valid (0x204f72f3...)
✅ ALCHEMY_API_KEY: Set
✅ All required configuration present
```

**Error Output Example:**

```
❌ Configuration validation failed!
❌ PRIVATE_KEY: Invalid format (must be 64 hex characters, no 0x prefix)
❌ FUNDER_ADDRESS: Invalid address format
⚠️  TARGET_WHALE_ADDRESS: Not set
```

---

### Check and Set Allowance

```bash
cargo run --release wallet check-allowance
```

**Purpose:** Approve USDC and Conditional Tokens for Polymarket exchange contracts. This command checks current allowances and sets them if needed.

**What it does:**
- Checks current token approvals
- Shows balances and allowance status
- Auto-approves tokens for EOA wallets
- Provides manual instructions for Gnosis Safe wallets

**Required approvals:**
1. **USDC (ERC-20)** - For buy orders
   - CTF Exchange: `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E`
   - Neg Risk Exchange: `0xC5d563A36AE78145C45a50134d48A1215220f80a`

2. **Conditional Tokens (ERC-1155)** - For sell orders
   - CTF Exchange: `0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E`
   - Neg Risk Exchange: `0xC5d563A36AE78145C45a50134d48A1215220f80a`

**When to use:**
- Before first trade
- After changing wallet
- If trades fail with "insufficient allowance" error
- Regular maintenance (check if approvals expired)

**For EOA Wallets (Regular Wallets):**

```bash
$ cargo run --release wallet check-allowance

🔐 Polymarket Token Approval Utility
=====================================

📝 Signer Wallet: 0xA35d8c52...
🏦 Funder Address: 0xA35d8c52... (same as signer)

📊 Checking current status...

   USDC Balance: 100.5 USDC
   USDC Allowance (CTF Exchange): 0 USDC
   USDC Allowance (Neg Risk Exchange): 0 USDC
   CTF Approved (CTF Exchange): false
   CTF Approved (Neg Risk Exchange): false

🔧 Setting approvals...

   ✅ USDC approved for CTF Exchange: 0xabc123...
   ✅ USDC approved for Neg Risk Exchange: 0xdef456...
   ✅ Conditional Tokens approved for CTF Exchange: 0x789abc...
   ✅ Conditional Tokens approved for Neg Risk Exchange: 0xdef789...

✅ All approvals verified successfully!
🚀 You can now trade on Polymarket!
```

**For Gnosis Safe Wallets:**

```bash
$ cargo run --release wallet check-allowance

🔐 Polymarket Token Approval Utility
=====================================

📝 Signer Wallet: 0xA35d8c52...
🏦 Funder Address (Gnosis Safe): 0x2108FF2b...

⚠️  IMPORTANT: Funder is a Gnosis Safe address!
   Direct approvals from private key won't work for Gnosis Safe.
   You need to approve through your Gnosis Safe interface:

   📝 Manual Approval Steps:
   1. Go to https://app.safe.global/
   2. Connect and select your Safe: 0x2108FF2b...
   3. Go to 'Apps' → Search 'Transaction Builder'
   4. Create transactions to approve:

   For USDC Approval:
     - Contract: 0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174
     - Method: approve(address,uint256)
     - Spender: 0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E (CTF Exchange)
     - Amount: Max

   For Conditional Tokens Approval:
     - Contract: 0x4d97dcd97ec945f40cf65f87097ace5ea0476045
     - Method: setApprovalForAll(address,bool)
     - Operator: 0x4bFb41d5B3570DeFd03C39a9A4D8dE6Bd8B8982E (CTF Exchange)
     - Approved: true

   5. Sign and execute the Safe transaction(s)

   ❌ Cannot auto-approve for Gnosis Safe. Please approve manually as shown above.
```

**Options:**
- `--dry-run` - Check approvals without executing transactions

```bash
cargo run --release wallet verify-allowance
```

---

## Main Bot

### Start Trading Bot

```bash
cargo run --release main run
```

**Purpose:** Start the copy trading bot in live trading mode.

**What it does:**
- Connects to Polymarket WebSocket
- Monitors target whale address for trades
- Executes scaled copy trades automatically
- Logs all activity to CSV files
- Handles order resubmission on failures

**When to use:**
- After completing setup and approvals
- To start copying trades
- Resume trading after stopping

**Prerequisites:**
1. ✅ `.env` file configured
2. ✅ Configuration validated (`validate_setup`)
3. ✅ Tokens approved (`approve_tokens`)
4. ✅ `ENABLE_TRADING=true` in `.env`
5. ✅ `MOCK_TRADING=false` in `.env`

**Example Output:**

```
📦 Loaded caches in 0ms: neg_risk=0, slugs=0, atp=0, ligue1=0, live=0
🔄 Cache refresh task started (interval: 1800s)
🚀 Starting trader. Trading: true, Mock: false
🔄 Resubmitter worker started
🔌 Connected. Subscribing...
👀 Watching whale: 0x204f72f35326db932158cba6adff0b9a1da95e14
⚡ [B:81872903] BUY_FILL | $1 | 200 OK [SCALED] | 2.94/3.03 filled @ 0.34 | whale 3.0 @ 0.33
⚡ [B:81872933] SELL_FILL | $1 | 200 OK [SCALED] | 2.13/2.13 filled @ 0.46 | whale 2.2 @ 0.45
```

**Stopping the bot:**
- Press `Ctrl+C` for graceful shutdown
- The bot will finish current operations before exiting

**Configuration (`.env`):**
```env
ENABLE_TRADING=true
MOCK_TRADING=false
PRIVATE_KEY=your_private_key_here
FUNDER_ADDRESS=your_funder_address_here
TARGET_WHALE_ADDRESS=target_whale_address_here
```

---

### Start Bot in Test Mode

```bash
# Set MOCK_TRADING=true in .env, then:
cargo run --release
```

**Purpose:** Run the bot in test/mock mode to see what it would do without executing real trades.

**What it does:**
- Monitors whale trades
- Calculates position sizes and prices
- Logs what trades would be placed
- **Does NOT execute actual trades**
- Safe for testing and validation

**When to use:**
- First-time setup testing
- Validating configuration
- Understanding bot behavior
- Testing new whale addresses
- Safe practice before live trading

**Example Output:**

```
📦 Loaded caches in 0ms: neg_risk=0, slugs=0, atp=0, ligue1=0, live=0
🚀 Starting trader. Trading: true, Mock: true
🔌 Connected. Subscribing...
👀 Watching whale: 0x204f72f35326db932158cba6adff0b9a1da95e14 (mock mode - no actual trades)
⚡ [B:12345] BUY_FILL | $1 | MOCK_ONLY | 2.17/2.17 filled @ 0.46 | whale 2.2 @ 0.46
⚡ [B:12346] SELL_FILL | $1 | MOCK_ONLY | 1.95/1.95 filled @ 0.51 | whale 2.0 @ 0.50
```

**Configuration (`.env`):**
```env
ENABLE_TRADING=true
MOCK_TRADING=true
```

---

## Wallet Management

### Check Wallet Balance

```bash
cargo run --release wallet check-proxy-wallet
```

**Purpose:** Check USDC and MATIC balance for your funder address.

**What it shows:**
- USDC balance (in USDC)
- MATIC balance (in MATIC)
- Warnings for low balances
- RPC connection status

**When to use:**
- Before starting trading
- Checking if wallet has sufficient funds
- Verifying Gnosis Safe balance
- Regular balance monitoring

**Example Output:**

```
💰 Wallet Balance Checker
=========================

📝 Signer Wallet: 0xA35d8c52...
🏦 Funder Address: 0x2108FF2b...
   ℹ️  Checking balance for Gnosis Safe address

🌐 Using RPC: Alchemy

🌐 Testing RPC connection...
   ✅ RPC connection successful

📊 Balance Summary:
   USDC Balance: 125.50 USDC
   MATIC Balance: 0.045 MATIC
```

**For Gnosis Safe:** Shows balance for the Gnosis Safe address (funder), not the signer address.

---

## Market Information

### Check Market Information

```bash
cargo run --release --bin check_market <token_id>
```

**Purpose:** Fetch and display detailed market information for a given token ID.

**What it shows:**
- Order book (best bid/ask, spread)
- Market information (question, outcome, condition ID)
- Live status
- Cache information (neg_risk, slug, buffers)

**When to use:**
- Researching markets before trading
- Checking market liquidity
- Verifying market status
- Debugging market data issues

**Example:**

```bash
cargo run --release --bin check_market 54829853978330669429551251905778214074128014124609781186771015417529556703558
```

**Example Output:**

```
📊 Market Information Checker
=============================

Token ID: 54829853978330669429551251905778214074128014124609781186771015417529556703558

📖 Fetching order book...
   Best Bid: $0.46 @ 892.5 shares
   Best Ask: $0.48 @ 1023.2 shares
   Spread: $0.02 (4.35%)

📈 Fetching market info...
   Market: Will the Rockets win vs Nuggets?
   Outcome: YES
   Question: Will the Rockets win vs Nuggets?
   Condition ID: 0x...
   Live: Yes

💾 Checking cache...
   Neg Risk: false
   Slug: rockets-vs-nuggets-2025-12-20
   Live (cached): Yes
```

---

### Refresh Market Cache

```bash
cargo run --release --bin refresh_cache
```

**Purpose:** Manually refresh all market data caches.

**What it does:**
- Refreshes neg_risk cache
- Refreshes slug cache
- Refreshes ATP tokens cache
- Refreshes Ligue1 tokens cache
- Refreshes live status cache
- Shows cache statistics

**When to use:**
- After cache becomes stale
- When market data seems incorrect
- Regular maintenance
- Troubleshooting cache issues

**Example Output:**

```
🔄 Market Cache Refresher
=========================

Loading caches...
📦 Loaded caches in 250ms: neg_risk=1234, slugs=1234, atp=45, ligue1=12, live=234

Refreshing caches...
🔄 Cache refresh: Loaded caches in 250ms: neg_risk=1234, slugs=1234, atp=45, ligue1=12, live=234

📊 Cache Refresh Results:
Loaded caches in 250ms: neg_risk=1234, slugs=1234, atp=45, ligue1=12, live=234

📈 Cache Statistics:
   Neg Risk: 1234 tokens
   Slugs: 1234 tokens
   ATP Tokens: 45 tokens
   Ligue1 Tokens: 12 tokens
   Live Status: 234 tokens

✅ Cache refresh completed successfully!
```

---

## Monitoring & Debugging

### Monitor Trades Only

```bash
cargo run --release --bin trade_monitor
```

**Purpose:** Monitor your own fills and trade activity without executing new trades.

**What it does:**
- Connects to Polymarket WebSocket
- Monitors your `FUNDER_ADDRESS` for fills
- Displays your trade activity
- Logs fills to console
- **Does NOT execute any trades**

**When to use:**
- Monitoring your bot's activity
- Verifying trades are executing
- Debugging order execution
- Passive monitoring

**Example Output:**

```
🔌 Connected to Polymarket WebSocket
👀 Monitoring trades for: 0x2108FF2b299800B7a904BD36A7cEd1c4Db5F47dC
📥 Fill received: BUY 2.94 shares @ $0.34
📥 Fill received: SELL 2.13 shares @ $0.46
```

---

### Test Connections

```bash
cargo run --release --bin test_connection
```

**Purpose:** Test RPC, WebSocket, and API connectivity.

**What it tests:**
- ✅ RPC connection (Polygon network)
- ✅ CLOB API accessibility
- ✅ WebSocket connection
- ✅ Configuration (env vars)

**When to use:**
- Before starting the bot
- Troubleshooting connection issues
- Verifying network connectivity
- Testing after configuration changes

**Example Output:**

```
🔌 Connection Tester
====================

1️⃣  Testing RPC connection...
   ✅ RPC: Connected (Chain ID: 137)

2️⃣  Testing CLOB API...
   ✅ CLOB API: Accessible

3️⃣  Testing WebSocket connection...
   ✅ WebSocket: Connected

4️⃣  Checking configuration...
   ✅ PRIVATE_KEY: Set
   ✅ FUNDER_ADDRESS: Set
   ✅ TARGET_WHALE_ADDRESS: Set
   ✅ API Key: Set

==================================================
✅ All connection tests passed!
```

---

### Monitor Mempool

```bash
cargo run --release --bin mempool_monitor
```

**Purpose:** Monitor mempool for faster trade detection (experimental).

**What it does:**
- Monitors Polygon mempool for pending transactions
- Detects trades before they're confirmed
- Faster trade detection (potentially)
- **Experimental feature - may be less reliable**

**When to use:**
- Testing faster execution
- Advanced users only
- Research/development

**Note:** Mempool monitoring is experimental and may not be as reliable as confirmed block monitoring.

---

## Testing & Validation

### Test Order Types

```bash
cargo run --release --bin test_order_types
```

**Purpose:** Test different order types (FAK, GTC, GTD) to verify they work correctly.

**What it does:**
- Tests order creation and signing
- Verifies different order types work
- Validates order format
- **Does NOT submit orders to exchange**

**When to use:**
- Testing order creation logic
- Debugging order issues
- Validating SDK integration
- Development/testing

**Warning:** This is a development tool. Use with caution.

---

## Quick Reference

### Most Common Commands

```bash
# Setup and validation
cargo run --release setup system-status     # Validate configuration, check balances, connectivity
cargo run --release wallet check-allowance  # Approve tokens for trading
cargo run --release setup system-status     # Test all connections (included in system-status)

# Wallet and market info
cargo run --release wallet check-proxy-wallet  # Check wallet balance
cargo run --release --bin check_market <token_id>  # Check market info

# Running the bot
cargo run --release main run                # Start bot (live or mock mode)

# Monitoring
cargo run --release --bin trade_monitor     # Monitor your fills only
cargo run --release --bin refresh_cache     # Refresh market caches

# Testing
cargo run --release --bin test_order_types  # Test order types (dev)
```

### Command Categories

| Category | Commands |
|----------|----------|
| **Setup** | `validate_setup`, `approve_tokens`, `test_connection` |
| **Wallet** | `check_balance` |
| **Market** | `check_market`, `refresh_cache` |
| **Trading** | `cargo run --release` (main bot) |
| **Monitoring** | `trade_monitor`, `mempool_monitor` |
| **Testing** | `test_order_types` |

### Using Helper Scripts

**Linux/macOS:**

```bash
# Run bot with helper script (validates config, builds, runs)
./run.sh

# Make executable if needed
chmod +x run.sh
```

**Windows:**

```batch
# Double-click or run from command prompt
run.bat
```

The helper scripts automatically:
1. Check if `.env` exists
2. Validate configuration
3. Build the bot
4. Run the bot

---

## Environment Variables

### Required Variables

| Variable | Description | Example |
|----------|-------------|---------|
| `PRIVATE_KEY` | Private key for signing (64 hex, no 0x) | `0123456789abcdef...` |
| `FUNDER_ADDRESS` | Wallet that holds funds (40 hex) | `0x2108FF2b...` |
| `TARGET_WHALE_ADDRESS` | Whale address to copy (40 hex, no 0x) | `204f72f35326db932158cba6adff0b9a1da95e14` |
| `ALCHEMY_API_KEY` OR `CHAINSTACK_API_KEY` | RPC provider API key | `abc123xyz789...` |

### Trading Configuration

| Variable | Description | Default | Options |
|----------|-------------|---------|---------|
| `ENABLE_TRADING` | Enable trading | `false` | `true` / `false` |
| `MOCK_TRADING` | Mock mode (no real trades) | `false` | `true` / `false` |

### Optional Configuration

See [Configuration Guide](03_CONFIGURATION.md) for complete list of optional settings including:
- Risk management (circuit breakers)
- Position sizing
- Order types
- Advanced features

---

## Output Files

### CSV Log Files

The bot creates CSV files for trade logging:

- `matches_optimized.csv` - All detected and executed trades
  - Columns: timestamp, whale address, token_id, side, size, price, status, etc.

### Cache Files

- `.clob_creds.json` - Auto-generated API credentials (don't modify)
- `.clob_market_cache.json` - Market data cache (auto-updated)

**Note:** Cache files are automatically managed. You don't need to interact with them directly.

---

## Getting Help

### Check Configuration

```bash
cargo run --release setup system-status
```

### Check Token Approvals

```bash
cargo run --release wallet check-allowance
```

### Review Documentation

- [Quick Start Guide](01_QUICK_START.md) - 5-minute setup
- [Setup Guide](02_SETUP_GUIDE.md) - Detailed setup instructions
- [Configuration Guide](03_CONFIGURATION.md) - All settings explained
- [Troubleshooting Guide](06_TROUBLESHOOTING.md) - Common issues and solutions

### Common Issues

1. **"Configuration validation failed"**
   - Run: `cargo run --release setup system-status`
   - Check `.env` file format
   - See [Troubleshooting Guide](06_TROUBLESHOOTING.md)

2. **"Not enough balance / allowance"**
   - Run: `cargo run --release wallet check-allowance`
   - For Gnosis Safe: Follow manual approval instructions
   - See [Troubleshooting Guide](06_TROUBLESHOOTING.md)

3. **"Invalid signature" error**
   - Verify `PRIVATE_KEY` format (64 hex, no 0x)
   - For Gnosis Safe: Verify signer is authorized
   - See [Troubleshooting Guide](06_TROUBLESHOOTING.md)

---

## Command Comparison

### Python Bot vs Rust Bot

| Feature | Python Bot | Rust Bot |
|---------|-----------|----------|
| **Setup** | `setup.setup` | Manual `.env` file |
| **Validation** | `setup.system_status` | `validate_setup` |
| **Approve Tokens** | `wallet.check_allowance` | `approve_tokens` |
| **Start Bot** | `main` | `cargo run --release` |
| **Monitor Trades** | `wallet.check_recent_activity` | `trade_monitor` |
| **Simulation** | `simulation.*` | Not available |
| **Research Tools** | `research.*` | Not available |

**Note:** The Rust bot focuses on core trading functionality with high performance. Additional features like simulation and research tools may be added in future versions.

---

## Tips & Best Practices

1. **Always validate before trading:**
   ```bash
   cargo run --release setup system-status
   ```

2. **Test in mock mode first:**
   - Set `MOCK_TRADING=true` in `.env`
   - Run bot and verify it detects trades correctly
   - Switch to live mode only after confirming everything works

3. **Monitor approvals regularly:**
   ```bash
   cargo run --release wallet verify-allowance
   ```

4. **Use helper scripts:**
   - Linux/macOS: `./run.sh`
   - Windows: `run.bat`
   - They handle validation and building automatically

5. **Keep `.env` file secure:**
   - Never commit to git (already in `.gitignore`)
   - Use a password manager for private keys
   - Don't share your configuration

6. **Monitor logs:**
   - Check `matches_optimized.csv` for trade history
   - Review console output for errors
   - Use `trade_monitor` for real-time monitoring

---

For more detailed information, see:

- [Getting Started Guide](01_QUICK_START.md)
- [Configuration Guide](03_CONFIGURATION.md)
- [Troubleshooting Guide](06_TROUBLESHOOTING.md)
- [Main README](../README.md)

