mod config;
mod ctx;
mod error;
mod models;
mod run;
mod services;
mod web;

use clap::Parser;
use std::process;

use config::{Args, Commands, Config};
use run::run;

// Re-exports
pub use error::{Error, Result};

#[tokio::main]
async fn main() {
    // Set the RUST_LOG, if it hasn't been explicitly defined
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "memo_rs=info")
    }

    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let args = Args::parse();

    if let Err(e) = run_command(args).await {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}

async fn run_command(arg: Args) -> Result<()> {
    match arg.command {
        Commands::Server => {
            let config = Config::build()?;
            run(config).await?;
            Ok(())
        }
    }
}
