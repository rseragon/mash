use std::process;
use tokio;

mod cliparser;
mod utils;
mod server;

#[tokio::main]
async fn main() {
    let config = cliparser::parse();

    if let Err(x) = cliparser::verify_config(&config) {
       println!("{x}");
       process::exit(1);
    }

    // Handles Ctrl-c in a different task
    tokio::spawn(async {
        tokio::signal::ctrl_c().await.unwrap();
        eprintln!("Ctrl-C detected! Stopping server");
        std::process::exit(1);
    });

    server::serve(&config).await;
}
