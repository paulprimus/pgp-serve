mod web;
mod gpg;

use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use dotenv;

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
    dotenv::dotenv().expect("Env konnte nicht gelesen werden!");
    tracing_subscriber::fmt().with_env_filter("debug,tower=trace").init();
    let cli = Cli::parse();


    match &cli.command {
        Some(Commands::Serve) => {
            tracing::info!("Starting Server");
            web::start().await.context("Webserver konnte nicht gestartet werden")?;
        }
        None => {}
    };
    Ok(())
}
