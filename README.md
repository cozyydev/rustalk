# rust-chat

A lightweight TCP chat server and client written in Rust.

## Quick Start

```bash
cargo run --bin rust-chat
```

The server listens on `0.0.0.0:2323`.

## Recommended Network Model

This project is designed primarily for private use over Tailscale.

The intended setup is:

- one central TCP server
- clients connect over Tailscale using MagicDNS or Tailscale IP
- no public internet exposure by default

Other VPN or tunneling methods may work, but they are not the primary deployment model for this project. If you expose the server outside your private tailnet, you are responsible for understanding the security and privacy implications.

## Other Network Options

These may work, but they are not the primary or recommended setup for this project.

- WireGuard
- Hamachi
- Tailscale Funnel
- Cloudflare Tunnel
- ngrok

Warning:
Exposing the chat server beyond your private network may increase risk. This project currently has no authentication beyond nickname uniqueness, no persistence, and no moderation/admin controls. Use public exposure methods at your own risk.

Note:
TCP tunnel options expose the raw TCP chat service. They do not automatically create a web UI or browser client.

## Connecting

### Option 1: Tailscale

1. Install [Tailscale](https://tailscale.com/install)
2. Ensure Tailscale is running on both machines
3. Get your Tailscale IP:

   ```bash
   tailscale ip -4
   ```

4. Connect from the client:

   ```bash
   nc <server-tailscale-ip> 2323
   ```

   Or use the included TUI client:

   ```bash
   cargo run --bin rust-chat -- --host <server-tailscale-ip>
   ```

### Option 2: Tailscale Funnel

If you want to allow connections from machines not on your Tailscale network:

```bash
tailscale funnel 2323
```

Then clients can connect to your public Funnel URL.
This changes the trust model and may expose the service more broadly. Use with caution.

### Option 4: Connecting from Mobile Devices

You can connect from your phone or tablet using a terminal app:

1. **iOS**: Install [Termius](https://termius.com) or [Blink Shell](https://blink.sh)
2. **Android**: Install [Termux](https://termux.com)

Then connect using netcat/telnet:

```bash
nc <server-ip> 2323
```

Or from Termux:

```bash
telnet <server-ip> 2323
```

Enter your nickname when prompted, then start chatting!

### Option 4: Cloudflare Tunnel

```bash
cloudflared tunnel --url tcp://localhost:2323
```

This exposes the raw TCP service. It does not provide a browser UI.

### Option 5: WireGuard

1. Set up WireGuard between machines
2. Connect using the WireGuard endpoint IP

### Option 6: Hamachi

1. Install Hamachi
2. Connect using your Hamachi network IP

### Option 7: ngrok

```bash
ngrok tcp 2323
```

Then connect to the provided ngrok URL.

## Usage

### Server

```bash
cargo run
```

### CLI Client

```bash
cargo run --bin rust-chat
```

### TUI Client

The TUI client provides an interactive terminal UI:

```bash
cargo run --bin tui -- --host <server-ip> [--port <port>]
```

Controls:

- `Enter` - Send message
- `Ctrl+c` - Quit
- `Up/Down` - Navigate message history

## Protocol

The server supports two modes:

### Plain Text Mode (default)

1. Send your nickname to join
2. Send messages to chat
3. `/who` - List online users
4. `/quit` - Disconnect

### JSON Mode

Send JSON messages:

```json
{"type": "hello", "nick": "yourname"}
{"type": "chat", "body": "Hello!"}
{"type": "who"}
{"type": "quit"}
```

## Project Status

Currently implemented:

- centralized TCP chat server
- CLI client
- Dracula-themed ratatui TUI client
- plain-text protocol support for telnet, netcat, CLI, and TUI clients
- JSON protocol support on the server for future richer clients
- bounded in-memory chat history
- nickname uniqueness enforcement
- online user tracking
- /who
- /quit
- manual scrolling in the TUI
- Tailscale-based connectivity via MagicDNS or Tailscale IP

Not yet implemented:

- multiple rooms/channels
- persistence/database storage
- authentication beyond nickname uniqueness
- native desktop GUI client
- web client
- Android app
- TUI JSON mode
- timestamps in the TUI
- online user side panel in the TUI

## Building

```bash
cargo build --release
```

The binary will be at `target/release/rust-chat`.
