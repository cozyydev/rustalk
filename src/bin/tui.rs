use std::env;
use std::io;
use std::time::Duration;

use anyhow::{Context, Result, anyhow};
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::mpsc,
};

#[path = "../tui/mod.rs"]
mod tui;

use tui::{
    app::App,
    draw::{draw_ui, max_scroll},
    message::{SystemKind, UiMessage},
    parser::{parse_message, parse_online_line},
    theme::Theme,
};

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::args()
        .nth(1)
        .ok_or_else(|| anyhow!("usage: cargo run --bin tui -- <host:port>"))?;

    let nick = prompt_for_nickname()?;
    run_tui(addr, nick).await
}

fn prompt_for_nickname() -> Result<String> {
    println!("Enter Your Handle:");
    let mut nick = String::new();
    io::stdin()
        .read_line(&mut nick)
        .context("failed to read handle")?;

    let nick = nick.trim().to_string();
    if nick.is_empty() {
        return Err(anyhow!("your handle cannot be empty"));
    }

    Ok(nick)
}

async fn run_tui(addr: String, nick: String) -> Result<()> {
    let stream = TcpStream::connect(&addr)
        .await
        .with_context(|| format!("failed to connect to {addr}"))?;

    let (reader, mut writer) = stream.into_split();

    writer
        .write_all(format!("{nick}\n").as_bytes())
        .await
        .context("failed to send handle")?;

    let (net_tx, mut net_rx) = mpsc::unbounded_channel::<String>();

    tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();

        loop {
            match lines.next_line().await {
                Ok(Some(line)) => {
                    let _ = net_tx.send(line);
                }
                Ok(None) => {
                    let _ = net_tx.send("[disconnected]".to_string());
                    break;
                }
                Err(err) => {
                    let _ = net_tx.send(format!("[read error] {err}"));
                    break;
                }
            }
        }
    });

    enable_raw_mode().context("failed to enable raw mode")?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen).context("failed to enter alternate screen")?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).context("failed to create terminal")?;

    let result = tui_loop(&mut terminal, &mut writer, &mut net_rx, addr, nick).await;

    disable_raw_mode().ok();
    execute!(terminal.backend_mut(), LeaveAlternateScreen).ok();
    terminal.show_cursor().ok();

    result
}

async fn tui_loop(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    writer: &mut tokio::net::tcp::OwnedWriteHalf,
    net_rx: &mut mpsc::UnboundedReceiver<String>,
    addr: String,
    nick: String,
) -> Result<()> {
    let mut app = App::new(addr, nick);
    let theme = Theme::default();

    app.push_message(UiMessage::System(
        SystemKind::Connected,
        "Connected.".to_string(),
    ));
    app.push_message(UiMessage::System(
        SystemKind::Info,
        "Type messages and press Enter.".to_string(),
    ));
    app.push_message(UiMessage::System(
        SystemKind::Info,
        "Commands: /who, /quit".to_string(),
    ));
    app.push_message(UiMessage::System(
        SystemKind::Info,
        "Scroll: ↑ ↓ PgUp PgDn End".to_string(),
    ));
    app.push_message(UiMessage::System(
        SystemKind::Info,
        "Press Esc to quit.".to_string(),
    ));

    loop {
        while let Ok(line) = net_rx.try_recv() {
            if let Some((count, users)) = parse_online_line(&line) {
                app.online_count = Some(count);
                app.online_users = users;
            }

            app.push_message(parse_message(line));
        }

        let frame_height = terminal.size()?.height;
        let message_area_height = frame_height.saturating_sub(5);
        let max = max_scroll(app.messages.len(), message_area_height);
        app.clamp_scroll(max);

        terminal.draw(|f| draw_ui(f, &app, &theme))?;

        if app.should_quit {
            break;
        }

        if event::poll(Duration::from_millis(50)).context("event poll failed")? {
            if let Event::Key(key) = event::read().context("event read failed")? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                let page = message_area_height.saturating_sub(2).max(1);

                match key.code {
                    KeyCode::Esc => {
                        writer
                            .write_all(b"/quit\n")
                            .await
                            .context("failed to send /quit")?;
                        break;
                    }
                    KeyCode::Enter => {
                        let line = app.input.trim().to_string();

                        if !line.is_empty() {
                            writer
                                .write_all(format!("{line}\n").as_bytes())
                                .await
                                .context("failed to send line")?;

                            if line == "/quit" {
                                break;
                            }
                        }

                        app.input.clear();
                    }
                    KeyCode::Backspace => {
                        app.input.pop();
                    }
                    KeyCode::Up => {
                        app.scroll_up(1);
                    }
                    KeyCode::Down => {
                        app.scroll_down(1, max);
                    }
                    KeyCode::PageUp => {
                        app.scroll_up(page);
                    }
                    KeyCode::PageDown => {
                        app.scroll_down(page, max);
                    }
                    KeyCode::End => {
                        app.scroll_to_bottom(max);
                    }
                    KeyCode::Char(c) => {
                        app.input.push(c);
                    }
                    _ => {}
                }
            }
        }
    }

    Ok(())
}
