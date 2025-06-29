# LanShare

LanShare is a cross-platform, zero-configuration P2P text sharing app for Windows, macOS, and Linux. It auto-discovers peers on the LAN and syncs text in real-time.

## Features
- Auto-discover peers on LAN (UDP broadcast)
- Real-time text sync (WebSocket)
- Zero configuration
- Clean, modern UI

## Getting Started
1. Install Rust and Tauri prerequisites (see [Tauri docs](https://tauri.app/v1/guides/getting-started/prerequisites/)).
2. Build and run:
   ```sh
   cargo tauri dev
   ```

## Project Structure
- `src/` - Rust backend (Tauri)
- `dist/` - Frontend (HTML/CSS/JS)
- `tauri.conf.json` - Tauri configuration
- `Cargo.toml` - Rust dependencies

## License
MIT
