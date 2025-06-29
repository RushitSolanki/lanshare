# LanShare

LanShare is a cross-platform, zero-configuration P2P text sharing app for Windows, macOS, and Linux. It auto-discovers peers on the LAN and syncs text in real-time.

## Features
- Auto-discover peers on LAN (UDP broadcast)
- Real-time text sync (WebSocket)
- Zero configuration
- Clean, modern UI
- Cross-platform support (Windows, macOS, Linux)

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
   git clone https://github.com/yourusername/lanshare.git
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
If you encounter any issues, please [open an issue](https://github.com/yourusername/lanshare/issues) on GitHub.

## Acknowledgments
- Built with [Tauri](https://tauri.app/)
- Inspired by the need for simple LAN-based text sharing
