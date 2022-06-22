use std::process;
use tokio;
use paris;

mod cliparser;
mod utils;
mod server;

#[tokio::main]
async fn main() {

    // Get the config
    let mut config = cliparser::parse();

    // Verify the config
    if let Err(e) = cliparser::verify_config(&mut config) {
       paris::error!("{e}");
       process::exit(1);
    }

    // Handles Ctrl-c in a different task
    tokio::spawn(async {
        tokio::signal::ctrl_c().await.unwrap();
        paris::info!("Ctrl-C detected! Stopping server");
        std::process::exit(1);
    });

    // Serve the server :)
    server::serve(&config).await;
}
