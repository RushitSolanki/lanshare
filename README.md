# LanShare

A peer-to-peer file sharing application built with Tauri and Rust.

## Features

- **UDP Peer Discovery**: Automatically discover other LanShare instances on the local network
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

- Rust 1.70+
- Node.js 16+
- Tauri CLI
- **macOS**: `create-dmg` (for DMG creation)

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
- **UDP Port**: 7878 (configurable)
- **Cleanup Interval**: 10 seconds (configurable)

### Logging

Set the `RUST_LOG` environment variable to control logging level:

```bash
RUST_LOG=debug cargo tauri dev
RUST_LOG=info cargo tauri dev
RUST_LOG=warn cargo tauri dev
```

## Network Requirements

- UDP port 7878 must be open for discovery
- Network must support UDP broadcast
- Firewall should allow UDP traffic on port 7878

## Security Considerations

- Discovery messages are not encrypted (for local network use)
- Peer IDs are randomly generated UUIDs
- No authentication mechanism (trusts local network)
- Consider implementing encryption for production use

## Future Enhancements

- [ ] Encrypted discovery messages
- [ ] Peer authentication
- [ ] Custom network interfaces
- [ ] Discovery over multiple networks
- [ ] Peer status indicators
- [ ] Manual peer addition

## Screenshots
*Coming soon*

## Getting Started

### Prerequisites
1. Install Rust (1.70 or later): https://rustup.rs/
2. Install Tauri prerequisites:
   - **macOS**: Xcode Command Line Tools
   - **Linux**: `sudo apt install libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev`
   - **Windows**: Microsoft Visual Studio C++ Build Tools

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

## Project Structure
```
LanShare/
├── src/                 # Rust backend (Tauri)
├── src-tauri/          # Tauri-specific configuration
│   ├── src/
│   │   └── main.rs     # Tauri main entry point
│   ├── Cargo.toml      # Tauri dependencies
│   └── tauri.conf.json # Tauri configuration
├── Cargo.toml          # Main Rust dependencies
└── README.md           # This file
```

## Development

### Architecture
- **Backend**: Rust with Tauri framework
- **Frontend**: HTML/CSS/JavaScript (Tauri webview)
- **Networking**: UDP broadcast for peer discovery, WebSocket for real-time sync
- **Data**: In-memory text storage with real-time synchronization

### Key Components
- `src/main.rs`: Main application logic
- `src-tauri/src/main.rs`: Tauri-specific setup and window management
- `src-tauri/tauri.conf.json`: Application configuration

## Contributing
1. Fork the repository
2. Create a feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Roadmap
- [ ] Add file sharing capabilities
- [ ] Implement encryption for secure sharing
- [ ] Add clipboard sync
- [ ] Create mobile companion app
- [ ] Add user authentication
- [ ] Implement persistent storage

## Issues
If you encounter any issues, please [open an issue](https://github.com/RushitSolanki/lanshare/issues) on GitHub.

## Acknowledgments
- Built with [Tauri](https://tauri.app/)
- Inspired by the need for simple LAN-based text sharing
