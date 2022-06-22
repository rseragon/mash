use clap::Parser;

/// Cli parser 
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Config {
    /// Host ip of the server
    #[clap(value_parser)]
    pub host: String,

    /// Port of the server
    #[clap(value_parser)]
    pub port: u16,

    /// Path for webserver to run (Defaults to current working directory)
    #[clap(value_parser, default_value_t = String::from("."))]
    pub path: String,

}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            host: self.host.clone(),
            port: self.port,
            path: self.path.clone(),
        }
    }
}

pub fn parse() -> Config {
    let args = Config::parse();

    args
}

pub fn verify_config(conf: &mut Config) -> Result<(), String> {

    verify_ip(&conf.host)?;
    verify_path(conf)?;

    Ok(())
}

fn verify_ip(ip: &String) -> Result<(), String> {

    let ip_octals: Vec<&str> = ip.split(".").collect();

    if ip_octals.len() != 4 { // x.x.x.x (4 parts)
        return Err(format!("Ill formatted IPv4: {}", ip));
    }

    for idx in 0..4 {
        // Confirms that the octets are in range 0..=255
        match ip_octals[idx].parse::<u8>() {
            Ok(x) => x,
            Err(_) => return Err(format!("Ill formatted IPv4: {}", ip)),
        };
    }

    Ok(())
}

fn verify_path(conf: &mut Config) -> Result<(), String> {
    use std::path::Path;

    let path_string = &conf.path;

    let p = Path::new(path_string);

    if !p.exists() {
        return Err(format!("Path doesn't exists: {}", path_string));
    }

    if p.is_file() {
        return Err(format!("Path provided is a file: {}", path_string));
    }

    // Set path to canonical path for future purpose
    conf.path = p.display().to_string();

    Ok(())
}
