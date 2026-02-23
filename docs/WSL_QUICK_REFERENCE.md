# WSL Quick Reference Card

Quick commands and tips for using WSL with your Rust bot.

## Installation (One-Time Setup)

```powershell
# In PowerShell (as Administrator)
wsl --install
# Restart computer when prompted
```

```bash
# In Ubuntu (after restart)
sudo apt update && sudo apt upgrade -y
sudo apt install -y build-essential curl git pkg-config libssl-dev
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

## Daily Use Commands

### Navigate to Project
```bash
# If project is in WSL filesystem
cd ~/projects/pm_bot

# If project is on Windows drive
cd /mnt/d/path/to/your/project/rust
```

### Build and Run
```bash
# Build (first time takes 5-10 min)
cargo build --release

# Validate configuration
cargo run --release main run setup system-status

# Run bot
cargo run --release main run

# Run with existing script
./run.sh
```

### Edit Configuration
```bash
# Edit .env file
nano .env
# Or
vim .env
```

### Run in Background
```bash
# Using nohup
nohup cargo run --release main run > bot.log 2>&1 &

# View logs
tail -f bot.log

# Using screen (install: sudo apt install screen)
screen -S bot
cargo run --release main run
# Detach: Ctrl+A then D
# Reattach: screen -r bot
```

## File Access

### Windows → WSL
- File Explorer: Type `\\wsl$` in address bar
- VS Code: Install "Remote - WSL" extension

### WSL → Windows
- Windows drives: `/mnt/c/`, `/mnt/d/`, etc.
- Example: `/mnt/d/Users/YourName/Documents`

## Useful WSL Commands

```powershell
# In PowerShell
wsl --list --verbose    # List distributions
wsl --shutdown          # Shutdown WSL
wsl --update            # Update WSL
wsl ls -la              # Run command in WSL
```

```bash
# In Ubuntu
pwd                     # Current directory
ls -la                  # List files
cd ~                    # Home directory
history                 # Command history
```

## Troubleshooting Quick Fixes

```bash
# Fix "cargo not found"
source ~/.cargo/env

# Fix permission errors
sudo chown -R $USER:$USER ~/projects

# Fix OpenSSL errors
sudo apt install -y libssl-dev pkg-config

# Update Rust
rustup update

# Check versions
rustc --version
cargo --version
```

## Project Setup Checklist

- [ ] WSL installed and Ubuntu set up
- [ ] Rust installed (`rustc --version` works)
- [ ] Project copied to WSL or accessible
- [ ] `.env` file created and configured
- [ ] Configuration validated (`validate_setup` passes)
- [ ] Test run successful (mock mode)
- [ ] Ready for live trading

## Need More Help?

- Full guide: [WSL_SETUP_GUIDE.md](WSL_SETUP_GUIDE.md)
- Troubleshooting: [06_TROUBLESHOOTING.md](06_TROUBLESHOOTING.md)
- Configuration: [03_CONFIGURATION.md](03_CONFIGURATION.md)






