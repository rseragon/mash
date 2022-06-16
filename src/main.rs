use std::process;

mod cliparser;
mod server;

//#[tokio::main]
fn main() {
    let config = cliparser::parse();

    if let Err(x) = cliparser::verify_config(&config) {
       println!("{x}");
       process::exit(1);
    }

    server::serve(&config);
}
