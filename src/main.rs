mod api;
mod app;
mod cli;
mod models;

use clap::Parser;
use app::App;
use cli::Cli;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    let app = App::new();

    if let Some(command) = cli.command {
        app.handle_command(command).await?;
    } else {
        println!("No command provided. Use --help for usage.");
    }

    Ok(())
}
