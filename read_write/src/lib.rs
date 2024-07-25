use extism_pdk::*;
use std::fs;

#[plugin_fn]
pub fn try_read() -> FnResult<Vec<u8>> {
    // Retrieve the path from the configuration
    let path = match config::get("path") {
        Ok(Some(path)) => path,
        Ok(None) => return Err(WithReturnCode::new(extism_pdk::Error::msg("Path not found in config"), 1)),
        Err(e) => return Err(WithReturnCode::new(extism_pdk::Error::msg(format!("Failed to get path from configuration: {}", e)), 1)),
    };

    // Read the content of the file
    match fs::read(&path) {
        Ok(data) => Ok(data),
        Err(e) => Err(WithReturnCode::new(extism_pdk::Error::msg(format!("Failed to read file: {}", e)), 1)),
    }
}

#[plugin_fn]
pub fn try_write(line: String) -> FnResult<Vec<u8>> {
    // Retrieve the path from the configuration
    let path = match config::get("path") {
        Ok(Some(path)) => path,
        Ok(None) => return Err(WithReturnCode::new(extism_pdk::Error::msg("Path not found in config"), 1)),
        Err(e) => return Err(WithReturnCode::new(extism_pdk::Error::msg(format!("Failed to get path from configuration: {}", e)), 1)),
    };

    // Read the content of the file
    let mut data = match fs::read(&path) {
        Ok(data) => data,
        Err(e) => return Err(WithReturnCode::new(extism_pdk::Error::msg(format!("Failed to read file: {}", e)), 1)),
    };

    // Append the input line to the file content
    data.extend_from_slice(line.as_bytes());

    // Write the new content back to the file
    match fs::write(&path, &data) {
        Ok(_) => Ok(data),
        Err(e) => Err(WithReturnCode::new(extism_pdk::Error::msg(format!("Failed to write file: {}", e)), 1)),
    }
}
