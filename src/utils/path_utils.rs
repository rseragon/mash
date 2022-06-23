/// Path utils

use paris;
use crate::cliparser::Config;

use crate::server::response::ResponseCode;
use super::err_and_expl::ErrAndExpl;
use std::path::{Path, PathBuf};


/// Checks if the path is valid and returns the path w.r.t to the
/// servers CWD
/// Checks
///     - If path exists
///     - Access control (does the use have the acccess to path?)
pub fn server_cwd_path(path_str: &str, config: &Config) -> Result<String, ErrAndExpl<ResponseCode>> {

    use ResponseCode::*;

    if path_str.len() < 1 {
        return Err(ErrAndExpl::new(BAD_REQUEST_400, 
                                   format!("Invalid path")));
    }

    let mut req_path_str = path_str.to_string(); 
    req_path_str.remove(0); // To move the `/` at the start
                            // Eg: /index.html -> index.html

    let mut pth = std::path::PathBuf::new();

    pth.push(&config.path); // The server path
    pth.push(req_path_str); // The requested path

    if !pth.exists() {
        return Err(ErrAndExpl::new(NOT_FOUND_404,
                                   format!("Path not found: {}", path_str)));
    }

    if let Err(code) = verify_path(&pth, config) {

        return Err(ErrAndExpl::new(code,
                                   format!("Invalid path requested: {}", path_str)));
    }

    Ok(pth.display().to_string())

}

// Check if you have read rights on that path
pub fn verify_path(req_path: &PathBuf, config: &Config) -> Result<(), ResponseCode> {
    // Get the canocnical paths to check the ancestors

    // TODO
    let cwd_pth = Path::new(&config.path).canonicalize().unwrap();
    let req_path = req_path.canonicalize().unwrap();

    // checks the path ancestors
    // If the ancestors are diff, that means the request is trying
    // to acces files out of the CWD
    let req_ancestors: Vec<_> = req_path.ancestors().collect();
    let cwd_ancestors: Vec<_>  = cwd_pth.ancestors().collect();

    let secure = cwd_ancestors.iter().all(|dir| req_ancestors.contains(dir));

    if !secure {
        return Err(ResponseCode::FORBIDDEN_403);
    }

    Ok(())
}
