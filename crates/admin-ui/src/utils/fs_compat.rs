//! File system compatibility layer for WASM and native targets

use crate::models::WebConfigError;
use crate::utils::time_compat::current_time_secs;

#[cfg(target_arch = "wasm32")]
pub async fn read_to_string(_path: &str) -> Result<String, WebConfigError> {
    // In WASM, we can't read files directly from the filesystem
    // This would typically be handled by the backend API
    // For now, return a mock configuration
    Ok(r#"
[server]
server_name = "example.com"
port = 8008

[database]
connection_string = "postgresql://user:pass@localhost/palpo"
"#.to_string())
}

#[cfg(target_arch = "wasm32")]
pub async fn write(path: &str, content: String) -> Result<(), WebConfigError> {
    // In WASM, we can't write files directly to the filesystem
    // This would typically be handled by sending data to the backend API
    // For now, we'll just log the operation
    web_sys::console::log_1(&format!("Would write to {}: {}", path, content).into());
    Ok(())
}

#[cfg(target_arch = "wasm32")]
pub async fn metadata(_path: &str) -> Result<FileMetadata, WebConfigError> {
    // Mock metadata for WASM
    Ok(FileMetadata {
        len: 1024,
        modified: current_time_secs(),
    })
}

#[cfg(target_arch = "wasm32")]
pub async fn copy(from: &str, to: &str) -> Result<(), WebConfigError> {
    // Mock copy operation for WASM
    web_sys::console::log_1(&format!("Would copy from {} to {}", from, to).into());
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn read_to_string(path: &str) -> Result<String, WebConfigError> {
    tokio::fs::read_to_string(path)
        .await
        .map_err(|e| WebConfigError::internal(format!("Failed to read file {}: {}", path, e)))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn write(path: &str, content: String) -> Result<(), WebConfigError> {
    tokio::fs::write(path, content)
        .await
        .map_err(|e| WebConfigError::internal(format!("Failed to write file {}: {}", path, e)))
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn metadata(path: &str) -> Result<FileMetadata, WebConfigError> {
    use std::time::UNIX_EPOCH;
    let metadata = tokio::fs::metadata(path)
        .await
        .map_err(|e| WebConfigError::internal(format!("Failed to get metadata for {}: {}", path, e)))?;
    
    let modified = metadata.modified()
        .map(|t| t.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0))
        .unwrap_or(0);
    
    Ok(FileMetadata {
        len: metadata.len(),
        modified,
    })
}

#[cfg(not(target_arch = "wasm32"))]
pub async fn copy(from: &str, to: &str) -> Result<(), WebConfigError> {
    tokio::fs::copy(from, to)
        .await
        .map_err(|e| WebConfigError::internal(format!("Failed to copy from {} to {}: {}", from, to, e)))?;
    Ok(())
}

pub struct FileMetadata {
    pub len: u64,
    pub modified: u64,
}