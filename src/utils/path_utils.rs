/// Path utils

use paris;
use crate::cliparser::Config;

/// Verifies the path relative the working directory of the server
pub fn verify_server_relative_path(path_str: &str, config: &Config) -> bool {

    if path_str.len() < 1 {
        return false;
    }

    let mut pth = std::path::PathBuf::new();
    pth.push(&config.path);

    let mut path_str = path_str.to_string();
    path_str.remove(0);

    pth.push(path_str);

    paris::success!("Requested path: {}", pth.display());

    pth.exists()
}
