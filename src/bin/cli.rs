use anyhow::{Context, Result, bail};
use std::env;
use std::io::{self, Write};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<()> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| "127.0.0.1:2323".to_string());

    println!("Connecting to {addr}...");
    let stream = TcpStream::connect(&addr)
        .await
        .with_context(|| format!("failed to connect to {addr}"))?;

    let (reader, mut writer) = stream.into_split();

    print!("Enter Your Handle: ");
    io::stdout().flush().context("failed to flush stdout")?;

    let mut nick = String::new();
    io::stdin()
        .read_line(&mut nick)
        .context("failed to read your handle")?;
    let nick = nick.trim();

    if nick.is_empty() {
        bail!("your handle cannot be empty");
    }

    writer
        .write_all(format!("{nick}\n").as_bytes())
        .await
        .context("failed to send handle")?;

    println!("Connected. Type messages and press Enter.");
    println!("Commands: /who, /quit");

    let read_task = tokio::spawn(async move {
        let mut lines = BufReader::new(reader).lines();

        while let Some(line) = lines.next_line().await? {
            println!("{line}");
        }

        Ok::<(), anyhow::Error>(())
    });

    let write_task = tokio::spawn(async move {
        let stdin = tokio::io::stdin();
        let mut input_lines = BufReader::new(stdin).lines();

        while let Some(line) = input_lines.next_line().await? {
            writer
                .write_all(format!("{line}\n").as_bytes())
                .await
                .context("failed to send line to server")?;

            if line.trim() == "/quit" {
                break;
            }
        }

        Ok::<(), anyhow::Error>(())
    });

    tokio::select! {
        result = read_task => {
            result??;
        }
        result = write_task => {
            result??;
        }
    }

    println!("Disconnected.");
    Ok(())
}
