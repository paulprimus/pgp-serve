mod web;
mod gpg;

use std::env;
use clap::{Parser, Subcommand};
use anyhow::{Context, Result};
use dotenvy;

#[derive(Parser)]
#[command(name = "gs")]
#[command(author, version, about = "Serve Keys", long_about = None,)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Serve { property_name: String }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().with_env_filter("debug,tower=trace").init();
    let cli = Cli::parse();


    match &cli.command {
        Some(Commands::Serve{property_name}) => {
            let current_dir = env::current_dir()?;
            let property = current_dir.join(property_name);
            dbg!("Env property: {}", &property);
            dotenvy::from_path(property).expect("Env konnte nicht gelesen werden!");
            tracing::info!("Starting Server");
            web::start().await.context("Webserver konnte nicht gestartet werden")?;
        }
        None => {}
    };
    Ok(())
}
