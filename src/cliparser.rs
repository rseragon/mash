use clap::Parser;

/// A minimal asynchronous static http webserver written in rust
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
pub struct Config {
    /// Path for webserver to run (Defaults to current working directory)
    #[clap(value_parser, default_value_t = String::from("."))]
    pub path: String,

    /// Host IP of the server 
    #[clap(short, long, value_parser, default_value_t = String::from("0.0.0.0"))]
    pub host: String,

    /// Port for the server 
    #[clap(short, long, value_parser, default_value_t = 8080)]
    pub port: u16,

    /// Open index.html if present in directory
    #[clap(long, short)]
    pub detect_index_html: bool
}

impl Clone for Config {
    fn clone(&self) -> Self {
        Config {
            host: self.host.clone(),
            port: self.port,
            path: self.path.clone(),
            detect_index_html: self.detect_index_html
        }
    }
}

pub fn parse() -> Config {
    Config::parse()
}

pub fn verify_config(conf: &mut Config) -> Result<(), String> {

    verify_ip(&conf.host)?;
    verify_path(conf)?;

    Ok(())
}

fn verify_ip(ip: &String) -> Result<(), String> {

    let ip_octals: Vec<&str> = ip.split('.').collect();

    if ip_octals.len() != 4 { // x.x.x.x (4 parts)
        return Err(format!("Ill formatted IPv4: {}", ip));
    }

    for octet in ip_octals.iter().take(4) {
        // Confirms that the octets are in range 0..=255
        match octet.parse::<u8>() {
            Ok(x) => x,
            Err(_) => return Err(format!("Ill formatted IPv4: {}", ip)),
        };
    }
    //
    // for idx in 0..4 {
    //     match ip_octals[idx].parse::<u8>() {
    //         Ok(x) => x,
    //         Err(_) => return Err(format!("Ill formatted IPv4: {}", ip)),
    //     };
    // }

    Ok(())
}

fn verify_path(conf: &mut Config) -> Result<(), String> {
    use std::path::Path;

    let path_string = conf.path.clone();

    let p = Path::new(&path_string);

    // TODO: Switch to try_exists
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
