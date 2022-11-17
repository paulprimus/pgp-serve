mod web;

use clap::{Parser, Subcommand};
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(name = "gs")]
#[command(author, version, about = "Serve Keys", long_about = None,)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Serve
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("debug,tower=trace").init();
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Serve) => {
            tracing::info!("Starting Server");
            web::start().await.context("Webserver konnte nicht gestartet werden")?;
        }
        None => {}
    };
    Ok(())
}
