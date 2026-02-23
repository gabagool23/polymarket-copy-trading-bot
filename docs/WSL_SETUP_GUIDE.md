# WSL Setup Guide for Windows

This guide will walk you through setting up WSL (Windows Subsystem for Linux) to run your Rust Polymarket bot on Windows. WSL gives you a real Linux environment without needing a VPS.

## Table of Contents

1. [What is WSL?](#what-is-wsl)
2. [Prerequisites](#prerequisites)
3. [Step 1: Install WSL](#step-1-install-wsl)
4. [Step 2: Set Up Ubuntu](#step-2-set-up-ubuntu)
5. [Step 3: Update Your System](#step-3-update-your-system)
6. [Step 4: Install Rust](#step-4-install-rust)
7. [Step 5: Install Additional Tools](#step-5-install-additional-tools)
8. [Step 6: Access Your Project](#step-6-access-your-project)
9. [Step 7: Build and Run Your Bot](#step-7-build-and-run-your-bot)
10. [Step 8: Configure Your Bot](#step-8-configure-your-bot)
11. [Troubleshooting](#troubleshooting)
12. [Tips and Best Practices](#tips-and-best-practices)

---

## What is WSL?

WSL (Windows Subsystem for Linux) allows you to run a Linux environment directly on Windows without dual-booting or using a virtual machine. It's perfect for:
- Running Linux-native tools and applications
- Development that requires Linux
- Better performance than VMs
- Accessing Windows files from Linux

---

## Prerequisites

Before starting, make sure you have:
- Windows 10 version 2004 or higher, or Windows 11
- Administrator access to your computer
- An internet connection
- At least 1GB of free disk space (more recommended)

---

## Step 1: Install WSL

### Method 1: Using PowerShell (Recommended - Fastest)

1. **Open PowerShell as Administrator:**
   - Press `Windows Key + X`
   - Click "Windows PowerShell (Admin)" or "Terminal (Admin)"
   - If prompted, click "Yes" to allow changes

2. **Run the WSL installation command:**
   ```powershell
   wsl --install
   ```
   
   This single command will:
   - Enable the required Windows features
   - Download and install the latest Ubuntu distribution
   - Set WSL 2 as the default version

3. **Restart your computer** when prompted (required for WSL to work)

### Method 2: Manual Installation (Alternative)

If Method 1 doesn't work, you can install manually:

1. **Enable WSL feature:**
   ```powershell
   dism.exe /online /enable-feature /featurename:Microsoft-Windows-Subsystem-Linux /all /norestart
   ```

2. **Enable Virtual Machine Platform:**
   ```powershell
   dism.exe /online /enable-feature /featurename:VirtualMachinePlatform /all /norestart
   ```

3. **Restart your computer**

4. **Set WSL 2 as default:**
   ```powershell
   wsl --set-default-version 2
   ```

5. **Install Ubuntu from Microsoft Store:**
   - Open Microsoft Store
   - Search for "Ubuntu" (or "Ubuntu 22.04 LTS")
   - Click "Install"

---

## Step 2: Set Up Ubuntu

After restarting your computer:

1. **Launch Ubuntu:**
   - Press `Windows Key`
   - Type "Ubuntu" and press Enter
   - Or open "Ubuntu" from the Start menu

2. **Wait for installation:**
   - First launch may take a few minutes
   - Ubuntu is setting up its environment

3. **Create a user account:**
   - You'll be prompted to create a username
   - Enter a username (lowercase, no spaces): `yourname`
   - Press Enter
   - Enter a password (you won't see it as you type - this is normal)
   - Press Enter
   - Confirm the password
   - Press Enter

   **Important:** Remember this password! You'll need it for `sudo` commands.

4. **Verify installation:**
   ```bash
   wsl --list --verbose
   ```
   
   You should see Ubuntu listed with version 2.

---

## Step 3: Update Your System

1. **Open Ubuntu** (if not already open)

2. **Update package lists:**
   ```bash
   sudo apt update
   ```
   - Enter your password when prompted
   - Wait for the update to complete

3. **Upgrade installed packages:**
   ```bash
   sudo apt upgrade -y
   ```
   - This may take 5-10 minutes
   - Type `Y` and press Enter if prompted

4. **Install essential build tools:**
   ```bash
   sudo apt install -y build-essential curl git pkg-config libssl-dev
   ```
   
   These are required for compiling Rust projects.

---

## Step 4: Install Rust

1. **Download and run the Rust installer:**
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Follow the prompts:**
   - Press `1` and Enter for default installation
   - Wait for installation (2-5 minutes)

3. **Reload your shell environment:**
   ```bash
   source ~/.cargo/env
   ```
   
   Or simply close and reopen Ubuntu.

4. **Verify Rust installation:**
   ```bash
   rustc --version
   cargo --version
   ```
   
   You should see version numbers like:
   ```
   rustc 1.xx.x (xxxxx xxxx-xx-xx)
   cargo 1.xx.x (xxxxx xxxx-xx-xx)
   ```

5. **Update Rust to latest version (optional but recommended):**
   ```bash
   rustup update
   ```

---

## Step 5: Install Additional Tools

Install tools that might be useful for your bot:

```bash
sudo apt install -y vim nano wget unzip
```

**Optional but recommended:**
- `vim` or `nano` - Text editors for editing files
- `wget` - Download files from the web
- `unzip` - Extract ZIP files

---

## Step 6: Access Your Project

You have two options for accessing your Rust project:

### Option A: Access Windows Files from WSL (Recommended for Quick Setup)

Windows drives are mounted in WSL at `/mnt/`:

1. **Navigate to your project:**
   ```bash
   cd /mnt/d/project/teraus\ github/Polymarket\ BOT/my\ github/rust\(backup\)/rust
   ```
   
   **Note:** Use backslashes to escape spaces in paths.

2. **Verify you're in the right directory:**
   ```bash
   ls -la
   ```
   
   You should see `Cargo.toml`, `src/`, etc.

### Option B: Copy Project to WSL (Recommended for Better Performance)

1. **Create a projects directory:**
   ```bash
   mkdir -p ~/projects
   cd ~/projects
   ```

2. **Copy your project from Windows:**
   ```bash
   cp -r /mnt/d/project/teraus\ github/Polymarket\ BOT/my\ github/rust\(backup\)/rust ./pm_bot
   cd pm_bot
   ```

   **Or use git if your project is in a repository:**
   ```bash
   git clone <your-repo-url> pm_bot
   cd pm_bot/rust
   ```

3. **Verify files:**
   ```bash
   ls -la
   ```

**Why Option B?** 
- Better performance (Linux filesystem is faster)
- No issues with Windows file permissions
- Better compatibility with Linux tools

---

## Step 7: Build and Run Your Bot

1. **Navigate to your project directory:**
   ```bash
   cd ~/projects/pm_bot  # or wherever you put it
   ```

2. **Verify Rust can see the project:**
   ```bash
   cargo --version
   ls -la
   ```

3. **Build the project (first time will take 5-10 minutes):**
   ```bash
   cargo build --release
   ```
   
   This downloads all dependencies and compiles your bot.

4. **Run the validation tool:**
   ```bash
   cargo run --release setup system-status
   ```

5. **Test run (mock mode):**
   ```bash
   cargo run --release
   ```

---

## Step 8: Configure Your Bot

1. **Create your `.env` file:**
   ```bash
   cp .env.example .env
   ```

2. **Edit the `.env` file:**
   ```bash
   nano .env
   ```
   
   Or use `vim` if you prefer:
   ```bash
   vim .env
   ```

3. **Fill in your configuration:**
   - `PRIVATE_KEY` - Your wallet's private key
   - `FUNDER_ADDRESS` - Your wallet address
   - `TARGET_WHALE_ADDRESS` - The whale address to copy
   - `ALCHEMY_API_KEY` - Your Alchemy API key
   - Set `ENABLE_TRADING=false` and `MOCK_TRADING=true` for testing

4. **Save and exit:**
   - In `nano`: Press `Ctrl+X`, then `Y`, then `Enter`
   - In `vim`: Press `Esc`, type `:wq`, press `Enter`

5. **Validate your configuration:**
   ```bash
   cargo run --release setup system-status
   ```

6. **Run your bot:**
   ```bash
   cargo run --release
   ```

---

## Troubleshooting

### Issue: "WSL command not found"

**Solution:**
- Make sure you've restarted your computer after installing WSL
- Try running `wsl --install` again in PowerShell as Administrator

### Issue: "Permission denied" errors

**Solution:**
- If accessing Windows files, you might need to fix permissions:
  ```bash
  sudo chown -R $USER:$USER /mnt/d/path/to/your/project
  ```
- Better: Copy the project to WSL's filesystem (Option B in Step 6)

### Issue: "Command not found: cargo"

**Solution:**
- Reload your shell: `source ~/.cargo/env`
- Or close and reopen Ubuntu
- Verify: `echo $PATH` should include `~/.cargo/bin`

### Issue: Build errors related to OpenSSL

**Solution:**
```bash
sudo apt install -y libssl-dev pkg-config
```

### Issue: Slow file access from Windows

**Solution:**
- Copy your project to WSL's filesystem instead of accessing Windows files
- Use `~/projects/` or similar Linux directory

### Issue: Can't access Windows files

**Solution:**
- Windows drives are at `/mnt/c/`, `/mnt/d/`, etc.
- Use forward slashes: `/mnt/d/path/to/file`
- Escape spaces: `/mnt/d/path\ with\ spaces/`

### Issue: Ubuntu won't start

**Solution:**
1. Check WSL status: `wsl --status` (in PowerShell)
2. Update WSL: `wsl --update` (in PowerShell as Admin)
3. Set default version: `wsl --set-default-version 2`

---

## Tips and Best Practices

### 1. Use WSL Terminal Integration

- **Windows Terminal** (recommended): Install from Microsoft Store for better experience
- **VS Code**: Install "Remote - WSL" extension to edit files in WSL from VS Code

### 2. File Location Best Practices

- **Store projects in WSL filesystem** (`~/projects/`) for better performance
- **Access Windows files** only when necessary (use `/mnt/` paths)

### 3. Running in Background

To run your bot in the background:

```bash
# Using nohup
nohup cargo run --release > bot.log 2>&1 &

# Or using screen (install first: sudo apt install screen)
screen -S bot
cargo run --release
# Press Ctrl+A then D to detach
# Reattach with: screen -r bot
```

### 4. Accessing WSL from Windows

- **File Explorer**: Type `\\wsl$` in address bar to access WSL files
- **VS Code**: Use "Remote - WSL" extension
- **Terminal**: Just open Ubuntu app or use Windows Terminal

### 5. Updating WSL

Keep WSL updated:
```bash
# In PowerShell (as Admin)
wsl --update
```

### 6. Managing WSL

Useful commands (run in PowerShell):
```powershell
# List installed distributions
wsl --list --verbose

# Shutdown WSL
wsl --shutdown

# Set default distribution
wsl --set-default Ubuntu

# Run command in WSL from PowerShell
wsl ls -la
```

### 7. Performance Tips

- **Store files in WSL filesystem** (`~`) not Windows (`/mnt/`)
- **Use WSL 2** (faster than WSL 1)
- **Allocate more memory** (if needed, edit `.wslconfig` in Windows)

---

## Next Steps

Once WSL is set up and your bot is running:

1. ✅ Read the [Complete Setup Guide](02_SETUP_GUIDE.md) for bot configuration
2. ✅ Review [Configuration Guide](03_CONFIGURATION.md) for all settings
3. ✅ Test in mock mode before live trading
4. ✅ Check [Troubleshooting Guide](06_TROUBLESHOOTING.md) if you encounter issues

---

## Quick Reference Commands

```bash
# Navigate to project
cd ~/projects/pm_bot

# Build
cargo build --release

# Run validation
cargo run --release setup system-status

# Run bot (mock mode)
cargo run --release

# Edit .env file
nano .env

# View logs
tail -f bot.log

# Check Rust version
rustc --version
cargo --version
```

---

## Need Help?

- Check the main [Troubleshooting Guide](06_TROUBLESHOOTING.md)
- Verify your setup: `cargo run --release setup system-status`
- Review error messages carefully
- Make sure all prerequisites are installed

---

**Congratulations!** 🎉 You now have WSL set up and ready to run your Rust bot on Windows!






