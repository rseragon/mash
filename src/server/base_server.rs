use crate::cliparser::Config;

pub fn serve(config: &Config) {

    println!("Started serving [{}:{}]: {}", config.host, config.port, config.path);
}
