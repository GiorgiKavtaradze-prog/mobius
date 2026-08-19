//! Command-line argument parsing.

use std::path::PathBuf;

use clap::Parser;

/// Default window title.
pub const DEFAULT_WINDOW_TITLE: &str = "Mobius";

/// Command-line arguments for Mobius.
#[derive(Debug, Parser, Clone)]
#[command(
    name = env!("CARGO_PKG_NAME"),
    version,
    about = "A GPU-rendered terminal emulator with inline 3D graphics",
    long_about = "Mobius is a high-performance GPU-rendered terminal emulator featuring inline 3D graphics, customized view modes, and seamless terminal capabilities.",
    trailing_var_arg = true
)]
pub struct Cli {
    /// Specify an alternative configuration file.
    #[arg(short = 'c', long = "config-file", value_name = "CONFIG_FILE")]
    pub config_file: Option<PathBuf>,

    /// Command and args to execute (must be last argument).
    #[arg(
        short = 'e',
        long = "command",
        value_name = "COMMAND",
        num_args = 1..,
        allow_hyphen_values = true
    )]
    pub command: Option<Vec<String>>,

    /// Defines the window title.
    #[arg(
        short = 'T',
        long = "title",
        value_name = "TITLE",
        default_value = DEFAULT_WINDOW_TITLE
    )]
    pub title: String,
}

impl Cli {
    /// Returns `true` if a custom command was provided via command line.
    pub fn has_custom_command(&self) -> bool {
        self.command.as_ref().is_some_and(|cmd| !cmd.is_empty())
    }

    /// Returns the primary executable name if a command override is specified.
    pub fn executable_name(&self) -> Option<&str> {
        self.command.as_ref()?.first().map(|s| s.as_str())
    }
}