# LanShare

A peer-to-peer text sharing application built with Tauri and Rust.

## Features

- **UDP Peer Discovery**: Automatically discover other LanShare instances on the local network
- **Real-time Text Sharing**: Instant text synchronization between discovered peers
- **Real-time Peer Management**: Maintain an up-to-date list of available peers
- **Automatic Cleanup**: Remove stale peers that haven't been seen for 30 seconds
- **Cross-platform**: Works on Windows, macOS, and Linux

## UDP Peer Discovery System

The application includes a comprehensive UDP peer discovery system that:

### Broadcasting
- Broadcasts presence every 5 seconds on UDP port 7878
- Includes peer ID, port, hostname, and timestamp
- Uses broadcast address to reach all devices on the network

### Listening
- Listens for discovery messages on UDP port 7878
- Automatically adds new peers to the registry
- Ignores its own broadcast messages

### Peer Registry
- Thread-safe peer management with `Arc<RwLock<HashMap>>`
- Automatic cleanup of stale peers (30-second timeout)
- Provides methods to add, remove, and list peers

### API Commands

The application exposes the following Tauri commands for the frontend:

- `get_peers()`: Returns a list of all discovered peers
- `get_peer_count()`: Returns the number of discovered peers
- `get_peer_id()`: Returns the current peer's ID
- `send_text_to_all_peers(text)`: Sends text to all discovered peers
- `send_text_to_peer(peer_id, text)`: Sends text to a specific peer

## Text Sharing

LanShare now supports real-time text sharing between peers:

- **Instant Sync**: Text typed in one instance appears in all other instances
- **UDP-based**: Uses the same UDP infrastructure as peer discovery
- **Cross-platform**: Works seamlessly between Windows, macOS, and Linux
- **Event-driven**: Real-time updates via Tauri's event system

## Architecture

### Core Components

1. **`Peer`**: Represents a discovered peer with ID, IP, port, last_seen timestamp, and hostname
2. **`PeerRegistry`**: Thread-safe registry for managing discovered peers
3. **`UdpBroadcaster`**: Handles broadcasting presence messages
4. **`UdpListener`**: Listens for discovery messages from other peers
5. **`DiscoveryService`**: Coordinates broadcasting and listening tasks

### Error Handling

- Comprehensive error handling for network operations
- Graceful degradation when network issues occur
- Detailed logging for debugging and monitoring

## Development

### Prerequisites

- **Rust**: 1.88.0 (tested with 1.88.0)
- **Cargo**: 1.88.0 (tested with 1.88.0)
- **Tauri CLI**: 2.6.2 (tested with 2.6.2)
- **Node.js**: 16+
- **Tauri Dependencies**: 
  - `tauri = "2"` (as specified in Cargo.toml)
  - `tauri-build = "2"` (as specified in Cargo.toml)
- **macOS**: `create-dmg` (for DMG creation)
- **Windows**: Microsoft Visual Studio C++ Build Tools
- **Linux**: `sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`

> **⚠️ Version Compatibility Note**: This project is tested and confirmed working with:
> - Rust 1.88.0
> - Cargo 1.88.0
> - Tauri CLI 2.6.2
> - Tauri framework 2.x
> 
> Using older versions may cause build failures. If you encounter issues, upgrade to these specific versions.

### Building

```bash
# Install dependencies
cargo build

# Run in development mode
cargo tauri dev

# Build for production (creates app bundle only)
cargo tauri build

# Create DMG file (macOS only)
./create_dmg.sh
```

### macOS DMG Creation

To create a DMG installer on macOS:

1. Install `create-dmg`:
   ```bash
   brew install create-dmg
   ```

2. Build the application:
   ```bash
   cargo tauri build
   ```

3. Create the DMG:
   ```bash
   ./create_dmg.sh
   ```

The DMG file will be created at `src-tauri/target/release/bundle/dmg/lanshare_0.1.0_aarch64.dmg`.

### Windows EXE Creation

To create a Windows executable and installer:

1. **Prerequisites** (if not already installed):
   ```bash
   # Install Microsoft Visual Studio C++ Build Tools
   # Download from: https://visualstudio.microsoft.com/visual-cpp-build-tools/
   ```

2. Build the application:
   ```bash
   cargo tauri build
   ```

3. **Output files** will be created in `src-tauri/target/release/bundle/`:
   - **Executable**: `msi/lanshare_0.1.0_x64_en-US.msi` (Windows installer)
   - **Portable**: `wix/lanshare_0.1.0_x64_en-US.msi` (alternative installer)
   - **Direct EXE**: `app/lanshare.exe` (standalone executable)

4. **Optional - Create portable executable**:
   ```bash
   # The standalone .exe file can be distributed directly
   # Located at: src-tauri/target/release/lanshare.exe
   ```

### Linux AppImage Creation

To create a Linux AppImage:

1. Build the application:
   ```bash
   cargo tauri build
   ```

2. **Output files** will be created in `src-tauri/target/release/bundle/`:
   - **AppImage**: `appimage/lanshare_0.1.0_amd64.AppImage`
   - **Debian package**: `deb/lanshare_0.1.0_amd64.deb`

### Testing

```bash
# Run all tests
cargo test

# Run tests with logging
RUST_LOG=debug cargo test
```

## Configuration

### Discovery Settings

- **Broadcast Interval**: 5 seconds (configurable)
- **Peer Timeout**: 30 seconds (configurable)
- **UDP Port**: 7878 (for both discovery and text sharing)
- **Cleanup Interval**: 10 seconds (configurable)

### Logging

Set the `RUST_LOG` environment variable to control logging level:

```bash
RUST_LOG=debug cargo tauri dev
RUST_LOG=info cargo tauri dev
RUST_LOG=warn cargo tauri dev
```

## Network Requirements

- UDP port 7878 must be open for discovery and text sharing
- Network must support UDP broadcast
- Firewall should allow UDP traffic on port 7878

## Security Considerations

- Discovery and text messages are not encrypted (for local network use)
- Peer IDs are randomly generated UUIDs
- No authentication mechanism (trusts local network)

## Current Status

### ✅ Implemented Features
- **UDP Peer Discovery**: Automatic discovery of peers on local network
- **Real-time Text Sharing**: Instant text synchronization between peers
- **Cross-platform Support**: Windows, macOS, and Linux
- **Automatic Cleanup**: Stale peer removal
- **Debug Interface**: Real-time peer information display
- **Event-driven Architecture**: Real-time updates via Tauri events

## Getting Started

### Prerequisites
1. Install Rust (1.88.0 or later): https://rustup.rs/
2. Install Cargo (1.88.0 or later): comes with Rustup
3. Install Tauri CLI (2.6.2 or later):
   ```bash
   cargo install tauri-cli --version 2.6.2
   ```
4. Install Tauri prerequisites:
   - **macOS**: Xcode Command Line Tools
   - **Linux**: `sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`
   - **Windows**: Microsoft Visual Studio C++ Build Tools

> **⚠️ Important**: This project is tested with specific versions. If you encounter build issues:
> - Use Rust 1.88.0: `rustup install 1.88.0 && rustup default 1.88.0`
> - Use Cargo 1.88.0: comes with Rustup
> - Use Tauri CLI 2.6.2: `cargo install tauri-cli --version 2.6.2`
> - The project uses Tauri framework 2.x (specified in Cargo.toml)

### Installation & Development

1. Clone the repository:
   ```sh
   git clone https://github.com/RushitSolanki/lanshare.git
   cd lanshare
   ```

2. Build and run in development mode:
   ```sh
   cargo tauri dev
   ```

3. Build for production:
   ```sh
   cargo tauri build
   ```

### Testing Text Sharing

1. Run LanShare on two different machines on the same network
2. Wait for peer discovery (should see each other in the debug panel)
3. Type text in the textarea on one machine
4. The text should appear in the textarea on the other machine

## Project Structure
```
LanShare/
├── src-tauri/           # Tauri-specific configuration
│   ├── src/
│   │   ├── main.rs      # Tauri main entry point
│   │   └── discovery.rs # UDP discovery and text sharing
│   ├── Cargo.toml       # Tauri dependencies
│   ├── tauri.conf.json  # Tauri configuration
│   └── capabilities/    # Tauri 2.x permissions
├── dist/                # Frontend files
│   ├── index.html       # Main HTML file
│   ├── main.js          # Frontend JavaScript
│   └── style.css        # Styling
├── ARCHITECTURE.md      # Detailed architecture documentation
└── README.md            # This file
```

## Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Issues
If you encounter any issues, please [open an issue](https://github.com/RushitSolanki/lanshare/issues) on GitHub.

## Acknowledgments
- Built with [Tauri](https://tauri.app/)
- Inspired by the need for simple LAN-based text sharing
