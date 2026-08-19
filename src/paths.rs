//! Shared filesystem path helpers.

use std::fs;
use std::path::{Path, PathBuf};

use etcetera::{BaseStrategy, choose_base_strategy};

/// Expands a leading `~` in a path using the current user's home directory.
pub fn expand_path(path: &Path) -> PathBuf {
    PathBuf::from(shellexpand::tilde(&path.to_string_lossy()).into_owned())
}

/// Returns the writable runtime asset root used for scene-backed object files.
pub fn runtime_asset_root() -> PathBuf {
    choose_base_strategy()
        .map(|strategy| strategy.cache_dir())
        .unwrap_or_else(|_| std::env::temp_dir())
        .join(env!("CARGO_PKG_NAME"))
        .join("assets")
}

/// Ensures that a directory exists, creating all parent components if necessary.
pub fn ensure_dir_exists(path: &Path) -> std::io::Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Returns the platform-dependent user configuration directory for Mobius.
pub fn user_config_dir() -> Option<PathBuf> {
    choose_base_strategy()
        .map(|strategy| strategy.config_dir().join(env!("CARGO_PKG_NAME")))
        .ok()
}
