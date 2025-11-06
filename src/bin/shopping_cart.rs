use std::net::SocketAddr;

use clap::{CommandFactory, Parser, Subcommand};

use shopping_cart::app::{create_app, AppState};
use shopping_cart::config::APP_CONFIG;

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<Command>,

    #[clap(short, long)]
    version: bool,
}

#[derive(Subcommand, Debug)]
enum Command {
    Start,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    if args.version {
        println!(env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    match args.command {
        Some(Command::Start) => {
            let app_state = AppState::new(APP_CONFIG.database_url.as_str(), &None).await?;

            let app = create_app(app_state).await;
            let address = format!("0.0.0.0:{}", APP_CONFIG.port);

            let listener = tokio::net::TcpListener::bind(address).await?;
            axum::serve(
                listener,
                app.into_make_service_with_connect_info::<SocketAddr>(),
            )
            .await
            .expect("Failed to start server");

            Ok(())
        }
        None => {
            Cli::command().print_help()?;
            Ok(())
        }
    }
}
