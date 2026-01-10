use clap::Parser;
use gamey::{self, CliArgs, Mode, run_cli_game, run_web_server};
use tracing_subscriber::prelude::*;

#[tokio::main] // We use tokio for both modes so the binary is consistent
async fn main() {
    tracing_subscriber::registry().init();
    let args = CliArgs::parse();

    if args.mode == Mode::Server {
        run_web_server(args.port).await;
    } else {
        run_cli_game().expect("End CLI game");
    }
}
