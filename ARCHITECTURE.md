# LanShare Architecture

## System Overview

LanShare is a peer-to-peer file sharing application built with Tauri 2.x and Rust. The application uses UDP-based peer discovery to automatically find other LanShare instances on the local network and enables **real-time text sharing** between discovered peers.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        LanShare Application                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │   Frontend UI   │    │   Tauri Bridge  │    │  Rust Backend│ │
│  │   (HTML/CSS/JS) │◄──►│   (IPC Layer)   │◄──►│   (Core Logic)│ │
│  │                 │    │                 │    │              │ │
│  │ • Text Area     │    │ • Event System  │    │ • UDP Text   │ │
│  │ • Debug Panel   │    │ • Command Invoke│    │   Sharing    │ │
│  │ • Peer Display  │    │ • Real-time     │    │ • Peer       │ │
│  │ • Status Info   │    │   Updates       │    │   Discovery  │ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘ │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Network Layer (UDP)                          │
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │   Broadcasting  │    │    Listening    │    │   Peer Mgmt  │ │
│  │   (Port 7878)   │    │   (Port 7878)   │    │   (Registry) │ │
│  │                 │    │                 │    │              │ │
│  │ • Peer Discovery│    │ • Peer Discovery│    │ • Thread-safe│ │
│  │ • Text Messages │    │ • Text Messages │    │   HashMap    │ │
│  │ • 5s Intervals  │    │ • Event Emission│    │ • Auto Cleanup│ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Component Architecture

### 1. Frontend Layer (dist/)

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend Components                      │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │   index.html    │    │    main.js      │    │   style.css  │ │
│  │                 │    │                 │    │              │ │
│  │ • App Structure │    │ • Tauri API     │    │ • UI Styling │ │
│  │ • Debug Panel   │    │ • Event Listeners│   │ • Responsive │ │
│  │ • Text Area     │    │ • Text Input    │    │ • Modern UI  │ │
│  │ • Status Display│    │ • Peer Updates  │    │ • Real-time  │ │
│  │ • Peer List     │    │ • Error Handling│    │   Updates    │ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 2. Tauri Bridge Layer

```
┌─────────────────────────────────────────────────────────────────┐
│                      Tauri IPC Bridge                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │   Frontend      │    │   Tauri IPC     │    │   Backend    │ │
│  │   Commands      │◄──►│   Invoke        │◄──►│   Handlers    │ │
│  │                 │    │                 │    │              │ │
│  │ • get_peers()   │    │ • Serialization │    │ • get_peers  │ │
│  │ • get_peer_count│    │ • Deserialization│   │ • get_peer_  │ │
│  │ • get_peer_id() │    │ • Error Handling│    │   count      │ │
│  │ • send_text_to_ │    │ • Type Safety   │    │ • get_peer_id│ │
│  │   all_peers()   │    │ • Event System  │    │ • send_text_ │ │
│  │ • Event Listen  │    │ • Real-time     │    │   to_all_    │ │
│  └─────────────────┘    └─────────────────┘    └   peers      ┘ │
└─────────────────────────────────────────────────────────────────┘
```

### 3. Backend Core (src-tauri/src/)

```
┌─────────────────────────────────────────────────────────────────┐
│                      Rust Backend Core                         │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │    main.rs      │    │   discovery.rs  │    │   App State  │ │
│  │                 │    │                 │    │              │ │
│  │ • App Entry     │    │ • Peer Discovery│    │ • Global     │ │
│  │ • Tauri Setup   │    │ • UDP Broadcast │    │   State      │ │
│  │ • Command       │    │ • UDP Listen    │    │ • Service    │ │
│  │   Handlers      │    │ • Peer Registry │    │   Management│ │
│  │ • Text Sharing  │    │ • Cleanup Tasks │    │ • Thread     │ │
│  │ • Event Emission│    │ • Message       │    │   Safety     │ │
│  │                 │    │   Handling      │    │              │ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

## Detailed Component Breakdown

### Discovery System Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                    Discovery Service                            │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │ DiscoveryService│    │  PeerRegistry   │    │     Peer     │ │
│  │                 │    │                 │    │              │ │
│  │ • Orchestrates  │    │ • Thread-safe   │    │ • Peer Info  │ │
│  │   all discovery │    │   HashMap       │    │ • ID, IP,    │ │
│  │   components    │    │ • Add/Remove    │    │   Port,      │ │
│  │ • Manages peer  │    │   peers         │    │   Hostname   │ │
│  │   ID generation │    │ • Cleanup stale │    │ • Timestamp  │ │
│  │ • Service       │    │   peers         │    │ • Stale      │ │
│  │   lifecycle     │    │ • Query peers   │    │   detection  │ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘ │
│           │                       │                       │      │
│           ▼                       ▼                       ▼      │
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │UdpBroadcaster   │    │  UdpListener    │    │DiscoveryMsg  │ │
│  │                 │    │                 │    │              │ │
│  │ • Broadcasts    │    │ • Listens on    │    │ • JSON       │ │
│  │   presence      │    │   UDP port 7878 │    │   format     │ │
│  │ • 5-second      │    │ • Processes     │    │ • Peer ID    │ │
│  │   intervals     │    │   incoming      │    │ • Port       │ │
│  │ • JSON messages │    │   messages      │    │ • Hostname   │ │
│  │ • Broadcast     │    │ • Ignores own   │    │ • Timestamp  │ │
│  │   address       │    │   messages      │    │ • Text Data  │ │
│  └─────────────────┘    └─────────────────┘    └──────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Text Sharing Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   User      │───►│  Frontend   │───►│   Backend   │───►│   UDP       │
│   Input     │    │  (main.js)  │    │  (main.rs)  │    │   Network   │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Text Area   │    │ invoke()    │    │ send_text_  │    │ UDP Packet  │
│ Change      │    │ Call        │    │ to_all_     │    │ to Port     │
│ Event       │    │             │    │ peers()     │    │ 7878        │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ Receiving   │
                                                    │ Peer        │
                                                    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ Event       │
                                                    │ Emission    │
                                                    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ Frontend    │
                                                    │ Update      │
                                                    └─────────────┘
```

### Network Communication Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   Peer A    │    │   Peer B    │    │   Peer C    │
│             │    │             │    │             │
│ ┌─────────┐ │    │ ┌─────────┐ │    │ ┌─────────┐ │
│ │Broadcast│ │    │ │Broadcast│ │    │ │Broadcast│ │
│ │  Task   │ │    │ │  Task   │ │    │ │  Task   │ │
│ └─────────┘ │    │ └─────────┘ │    │ └─────────┘ │
│     │       │    │     │       │    │     │       │
│     ▼       │    │     ▼       │    │     ▼       │
│ ┌─────────┐ │    │ ┌─────────┐ │    │ ┌─────────┐ │
│ │UDP Port │ │    │ │UDP Port │ │    │ │UDP Port │ │
│ │  7878   │ │    │ │  7878   │ │    │ │  7878   │ │
│ └─────────┘ │    │ └─────────┘ │    │ └─────────┘ │
│     │       │    │     │       │    │     │       │
│     ▼       │    │     ▼       │    │     ▼       │
│ ┌─────────┐ │    │ ┌─────────┐ │    │ ┌─────────┐ │
│ │Listener │ │    │ │Listener │ │    │ │Listener │ │
│ │  Task   │ │    │ │  Task   │ │    │ │  Task   │ │
│ └─────────┘ │    │ └─────────┘ │    │ └─────────┘ │
└─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │
       └───────────────────┼───────────────────┘
                           │
                    ┌─────────────┐
                    │   Network   │
                    │  Broadcast  │
                    │   (UDP)     │
                    └─────────────┘
```

## Data Flow Architecture

### 1. Application Startup Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│   main()    │───►│ Tauri Setup │───►│Discovery    │───►│ Background  │
│             │    │             │    │Service Init │    │ Tasks Start │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ App State   │    │ Command     │    │ Peer ID     │    │ Broadcaster │
│ Creation    │    │ Handlers    │    │ Generation  │    │ Task        │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ Listener    │
                                                    │ Task        │
                                                    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ Cleanup     │
                                                    │ Task        │
                                                    └─────────────┘
```

### 2. Peer Discovery Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Broadcast   │───►│ UDP Network │───►│ Listener    │───►│ Peer        │
│ Message     │    │ (Port 7878) │    │ Receives    │    │ Registry    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ JSON        │    │ Broadcast   │    │ Message     │    │ Add/Update  │
│ Serialize   │    │ to All      │    │ Deserialize │    │ Peer Entry  │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### 3. Text Sharing Flow

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Frontend    │───►│ Tauri IPC   │───►│ Command     │───►│ Backend     │
│ (main.js)   │    │ Bridge      │    │ Handler     │    │ Logic       │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                   │                   │                   │
       ▼                   ▼                   ▼                   ▼
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Text Input  │    │ Serialize   │    │ send_text_  │    │ UDP Send    │
│ Event       │    │ Request     │    │ to_all_peers│    │ to all known│
│             │    │             │    │ (iterates)  │    │ peers       │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ UDP Receive │
                                                    │ (Port 7878) │
                                                    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ Event       │
                                                    │ Emission    │
                                                    └─────────────┘
                                                              │
                                                              ▼
                                                    ┌─────────────┐
                                                    │ Frontend    │
                                                    │ Update      │
                                                    └─────────────┘
```

## Technical Specifications

### Dependencies

#### Backend (Rust)
- **tauri**: ^2.0 - Desktop application framework
- **serde**: 1.0 - Serialization framework
- **tokio**: 1.38 - Async runtime
- **uuid**: 1.7 - UUID generation
- **log**: 0.4.21 - Logging framework
- **anyhow**: 1.0 - Error handling
- **chrono**: 0.4.38 - Date/time handling

#### Frontend
- **HTML5**: Structure and semantics
- **CSS3**: Styling and layout
- **JavaScript**: Interactivity and Tauri API integration

### Network Protocol

#### UDP Discovery Protocol
- **Port**: 7878 (for both discovery and text sharing)
- **Broadcast Interval**: 5 seconds
- **Peer Timeout**: 30 seconds
- **Cleanup Interval**: 10 seconds
- **Message Format**: JSON

```json
{
  "message_type": "PeerDiscovery|TextMessage",
  "peer_id": "uuid-string",
  "port": 7878,
  "hostname": "optional-hostname",
  "timestamp": "ISO-8601-timestamp",
  "text": "optional-text-content"
}
```

### Threading Model

```
┌─────────────────────────────────────────────────────────────────┐
│                    Thread Architecture                          │
├─────────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐    ┌──────────────┐ │
│  │   Main Thread   │    │ Broadcaster     │    │ Listener     │ │
│  │                 │    │ Thread          │    │ Thread       │ │
│  │ • Tauri Runtime │    │ • UDP Broadcast │    │ • UDP Listen │ │
│  │ • UI Updates    │    │ • 5s Intervals  │    │ • Message    │ │
│  │ • Command       │    │ • JSON Messages │    │   Processing │ │
│  │   Handling      │    │ • Error Handling│    │ • Peer       │ │
│  │ • State Mgmt    │    │ • Text Messages │    │   Registry   │ │
│  │ • Event Emission│    │                 │    │ • Event      │ │
│  └─────────────────┘    └─────────────────┘    └   Emission  ┘ │
│           │                       │                       │      │
│           └───────────────────────┼───────────────────────┘      │
│                                   │                              │
│                                   ▼                              │
│                          ┌─────────────────┐                     │
│                          │ Cleanup Thread  │                     │
│                          │                 │                     │
│                          │ • Stale Peer    │                     │
│                          │   Removal       │                     │
│                          │ • 10s Intervals │                     │
│                          │ • Registry      │                     │
│                          │   Maintenance   │                     │
│                          └─────────────────┘                     │
└─────────────────────────────────────────────────────────────────┘
```

### Error Handling Strategy

#### Backend Error Handling
- **anyhow**: Comprehensive error propagation
- **Result<T, E>**: Type-safe error handling
- **Logging**: Structured logging with different levels
- **Graceful Degradation**: Continue operation on non-critical errors

#### Frontend Error Handling
- **Try-Catch Blocks**: JavaScript error handling
- **Status Updates**: User-visible error states
- **Retry Logic**: Automatic reconnection attempts
- **Debug Information**: Detailed error reporting

### Security Considerations

#### Current Implementation
- **Local Network Only**: UDP broadcast limited to local network
- **No Authentication**: Trusts local network peers
- **No Encryption**: Discovery and text messages are plain text
- **UUID-based IDs**: Random peer identification

## Deployment Architecture

### Build Process

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ Source Code │───►│ Cargo Build │───►│ Tauri Build │───►│ App Bundle  │
│             │    │ (Rust)      │    │ (Frontend)  │    │ (Platform)  │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
```

### Platform Support

#### macOS
- **Format**: `.app` bundle, `.dmg` installer
- **Architecture**: x86_64, aarch64
- **Dependencies**: None (self-contained)

#### Windows
- **Format**: `.exe`, `.msi` installer
- **Architecture**: x64
- **Dependencies**: Visual C++ Redistributable

#### Linux
- **Format**: `.AppImage`, `.deb` package
- **Architecture**: x86_64
- **Dependencies**: WebKit2GTK, GTK3

## Performance Characteristics

### Network Performance
- **Discovery Latency**: < 5 seconds
- **Text Sync Latency**: < 1 second
- **Peer Timeout**: 30 seconds
- **Broadcast Overhead**: Minimal (JSON messages)
- **Memory Usage**: Low (peer registry only)

### Application Performance
- **Startup Time**: < 2 seconds
- **Memory Footprint**: < 50MB
- **CPU Usage**: < 1% (idle)
- **Disk Usage**: < 10MB

## Current Features

### ✅ Implemented
- **UDP Peer Discovery**: Automatic discovery of peers on local network
- **Real-time Text Sharing**: Instant text synchronization between peers
- **Cross-platform Support**: Windows, macOS, and Linux
- **Automatic Cleanup**: Stale peer removal
- **Debug Interface**: Real-time peer information display
- **Event-driven Architecture**: Real-time updates via Tauri events