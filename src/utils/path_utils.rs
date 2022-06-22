/// Path utils

use paris;

/// Verifies the path relative the working directory of the server
pub fn verify_server_relative_path(path_str: &str) -> bool {

    let mut curr_path = std::env::current_dir().unwrap_or_else(|err| {
        paris::error!("Failed to get current working directory");
        std::process::exit(1);
    });

    if path_str.len() < 1 {
        return false;
    }

    let mut path_str = path_str.to_string();
    path_str.remove(0); // rmove the extra '/' at the start
                        // eg: /index.html -> index.html

    curr_path.push(path_str); 

    let pth = curr_path.as_path();

    pth.exists()
}
