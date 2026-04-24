# rust-chat

A lightweight TCP chat server with CLI and TUI clients written in Rust.

## Quick Start

Start the server:

```bash
cargo run
```

In another terminal, start the client:

```bash
cargo run -- <server-ip>:42069
```

The server listens on `0.0.0.0:42069`.

## Recommended Network Model

This project is designed primarily for private use over Tailscale.

The intended setup is:

- one central TCP server
- clients connect over Tailscale using MagicDNS or Tailscale IP
- no public internet exposure by default

Other VPN or tunneling methods may work, but they are not the primary deployment model for this project. If you expose the server outside your private tailnet, you are responsible for understanding the security and privacy implications.

### Other Network Options

These may work, but they are not the primary or recommended setup for this project.

- WireGuard
- Hamachi
- Tailscale Funnel
- Cloudflare Tunnel
- ngrok

**Warning:**
Exposing the chat server beyond your private network may increase risk. This project currently has no authentication beyond nickname uniqueness, no persistence, and no moderation/admin controls. Use public exposure methods at your own risk.

**Note:**
TCP tunnel options expose the raw TCP chat service. They do not automatically create a web UI or browser client.

## Connecting

**Important:** Start the server first, then connect clients to it.

### Option 1: Tailscale

1. Install [Tailscale](https://tailscale.com/install)
2. Ensure Tailscale is running on both machines
3. Get your Tailscale IP:

   ```bash
   tailscale ip -4
   ```

4. Connect from the client:

   ```bash
   nc <server-tailscale-ip> 42069
   ```

   Or use the included CLI client:

   ```bash
   cargo run --bin cli -- <server-tailscale-ip>:42069
   ```

   Or use the included TUI client:

   ```bash
   cargo run -- <server-tailscale-ip>:42069
   ```

### Option 2: Tailscale Funnel

If you want to allow connections from machines not on your Tailscale network:

```bash
tailscale funnel 42069
```

Then clients can connect to your public Funnel URL.

This changes the trust model and may expose the service more broadly. Use with caution.

### Option 3: Connecting from Mobile Devices

You can connect from your phone or tablet using a terminal app:

1. **iOS**: Install [Termius](https://termius.com) or [Blink Shell](https://blink.sh)
2. **Android**: Install [Termux](https://termux.com)

Then connect using netcat/telnet:

```bash
nc <server-ip> 42069
```

Or from Termux:

```bash
telnet <server-ip> 42069
```

Enter your nickname when prompted, then start chatting!

### Option 4: Cloudflare Tunnel

```bash
cloudflared tunnel --url tcp://localhost:42069
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
ngrok tcp 42069
```

Then connect to the provided ngrok TCP endpoint.

## Usage

### Server

Start the server:

```bash
cargo run
```

### CLI Client

```bash
cargo run --bin cli -- <server-ip>:42069
```

### TUI Client

The TUI client provides an interactive terminal UI:

```bash
cargo run -- <server-ip>:42069
```

Controls:

- `Enter` - send message
- `Esc` - quit
- `Up/Down` - scroll
- `PageUp/PageDown` - page scroll
- `End` - jump back to bottom/live mode

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

This produces binaries such as:

- `target/release/rust-chat`
- `target/release/cli`
- `target/release/tui`

