# LanShare

Peer‑to‑peer text sharing for local networks, built with Tauri and Rust.

## Features

- **UDP peer discovery**: Automatically finds other LanShare instances on your LAN
- **Real‑time text sync**: Instantly mirrors text between discovered peers
- **Large message support**: Up to 256 KB per message with automatic chunking
- **Reliable reassembly**: Checksums, duplicate handling, and timeout cleanup
- **Live peer management**: Keeps an up‑to‑date list of available peers
- **Cross‑platform**: Windows, macOS, and Linux
 - **Auto‑copy on receive**: Received text is automatically copied to the system clipboard

## Requirements

- **Rust**: 1.88.0 (tested)
- **Cargo**: 1.88.0 (tested)
- **Tauri CLI**: 2.6.2 (tested)

Platform prerequisites:
- **macOS**: Xcode Command Line Tools; optional `create-dmg` via `brew install create-dmg`
- **Linux**: `sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`
- **Windows**: Microsoft Visual Studio C++ Build Tools

> If you hit build issues, use the verified versions:
> ```sh
> rustup install 1.88.0 && rustup default 1.88.0
> cargo install tauri-cli --version 2.6.2
> ```

## Quick start

```sh
# Clone and enter the project
git clone https://github.com/RushitSolanki/lanshare.git
cd lanshare

# Build & run in development
cargo tauri dev

# Build production bundles
cargo tauri build
```

## Packaging (platform‑specific)

### macOS (DMG)
```bash
# After `cargo tauri build`
./create_dmg.sh
```
The DMG appears at `src-tauri/target/release/bundle/dmg/LanShare_0.1.0_aarch64.dmg` (filename may vary by arch/version).

### Windows (MSI/EXE)
- Run `cargo tauri build`
- Artifacts are placed under `src-tauri/target/release/bundle/` (exact names vary by arch/locale)

### Linux (AppImage/Deb)
- Run `cargo tauri build`
- Artifacts are placed under `src-tauri/target/release/bundle/`

## Test

```bash
cd src-tauri
cargo test

# With debug logs
RUST_LOG=debug cargo test
```

## Configuration

- **Broadcast interval**: 5s (configurable)
- **Peer timeout**: 30s (configurable)
- **UDP port**: 7878 (discovery and text sharing)
- **Cleanup interval**: 10s (configurable)

Logging (examples):
```bash
RUST_LOG=debug cargo tauri dev
RUST_LOG=info cargo tauri dev
RUST_LOG=warn cargo tauri dev
```

## Using LanShare

1. Run LanShare on two machines on the same network
2. Wait for peer discovery (peers appear in the debug panel)
3. Type in one textarea — the text mirrors to the other machine
4. On the receiving machine, the received text is auto‑copied to the clipboard, so you can paste immediately (Cmd/Ctrl+V). The manual copy button remains available as a fallback.

Message size behavior:
- **≤ 1100 bytes**: Single UDP packet
- **> 1100 bytes**: Chunked automatically
- **Max**: 256 KB per message (configurable)

## Networking & Security

- Ensure UDP 7878 is allowed by firewall and the network supports UDP broadcast
- Messages are not encrypted (intended for trusted local networks)
- Peer IDs are random UUIDs; no authentication is implemented

## Project structure
```
LanShare/
├── src-tauri/           # Tauri app (Rust)
│   ├── src/
│   │   ├── main.rs      # Tauri main entry point
│   │   └── discovery.rs # UDP discovery and text sharing
│   ├── Cargo.toml
│   ├── tauri.conf.json
│   └── capabilities/
├── dist/                # Frontend assets (static)
│   ├── index.html
│   ├── main.js
│   └── style.css
├── ARCHITECTURE.md
├── README.md
└── LICENSE
```

## Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/your-change`)
3. Commit (`git commit -m "Describe your change"`)
4. Push (`git push origin feature/your-change`)
5. Open a Pull Request

## License

MIT — see `LICENSE`.

## Issues

Please open issues at `[GitHub Issues](https://github.com/RushitSolanki/lanshare/issues)`.

## Acknowledgments

- Built with `[Tauri](https://tauri.app/)`
- Inspired by the need for simple LAN‑based text sharing
