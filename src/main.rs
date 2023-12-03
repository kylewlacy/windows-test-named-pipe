use std::time::Duration;

use clap::Parser;
use color_eyre::owo_colors::OwoColorize;
use tokio::io::AsyncWriteExt as _;
use tokio::net::windows::named_pipe::{PipeMode, ServerOptions};

const PIPE_NAME: &str = "test-pipe";
const PIPE_PATH: &str = r"\\.\pipe\test-pipe";

#[derive(Debug, Parser)]
struct Args {
    #[clap(short, long, default_value = "25")]
    num_messages: u32,

    #[clap(short, long, default_value = "5000")]
    message_size: usize,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let Args {
        num_messages,
        message_size,
    } = Args::parse();

    println!("Running with {num_messages} messages of size {message_size}");

    let mut server = ServerOptions::new()
        .first_pipe_instance(true)
        .pipe_mode(PipeMode::Message)
        .create(PIPE_PATH)?;

    let server_task = tokio::task::spawn(async move {
        server.connect().await?;
        println!("{} Client connected", "[server]".magenta());
        for i in 1..=num_messages {
            let message: Vec<u8> = std::iter::repeat(())
                .enumerate()
                .map(|(i, _)| i as u8)
                .take(message_size)
                .collect();
            server.write_all(&message).await?;
            server.flush().await?;
            println!("{} Sent message {i} / {num_messages}", "[server]".magenta());
        }

        Result::<_, eyre::Error>::Ok(())
    });

    let client_task = tokio::task::spawn_blocking(move || -> eyre::Result<()> {
        let (client_reader, _) = win_pipes::NamedPipeClientOptions::new(PIPE_NAME)
            .wait()
            .mode_message()
            .access_inbound()
            .create()?;

        for i in 1..=num_messages {
            println!("{} Reading message {i} / {num_messages}", "[client]".cyan());
            let message = client_reader.read_full()?;
            let length = message.len();
            println!(
                "{} Read message {i} / {num_messages}: {length}",
                "[client]".cyan()
            );
            std::thread::sleep(Duration::from_millis(50));
        }

        Ok(())
    });

    let (client_result, server_result) = tokio::try_join!(client_task, server_task)?;
    client_result?;
    server_result?;

    Ok(())
}
