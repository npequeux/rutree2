//! # rutree2
//!
//! A Rust command-line tool for displaying directory structures in a tree format.
//!
//! ## Features
//!
//! - Display directory trees with proper indentation and tree characters
//! - Show hidden files with the `-a` or `--all` flag
//! - Limit traversal depth with the `-d` or `--depth` option
//! - Sort entries alphabetically
//! - Color-coded output based on file types and permissions
//! - Clean, readable output with visual tree structure
//!
//! ## Usage
//!
//! ```bash
//! # Display the current directory
//! rutree2
//!
//! # Display a specific directory
//! rutree2 /path/to/directory
//!
//! # Show hidden files
//! rutree2 --all
//!
//! # Limit depth to 2 levels
//! rutree2 --depth 2
//!
//! # Always use colors
//! rutree2 --color always
//! ```

use clap::Parser;
use colored::*;
use std::fs;
use std::io::IsTerminal;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

/// Command-line interface configuration for rutree2
#[derive(Parser)]
#[command(name = "rutree2")]
#[command(about = "Display directory tree structure", long_about = None)]
struct Cli {
    /// Path to display (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Show hidden files
    #[arg(short = 'a', long)]
    all: bool,

    /// Maximum depth to traverse
    #[arg(short = 'd', long)]
    depth: Option<usize>,

    /// Use colors to distinguish file types and permissions (auto, always, never)
    #[arg(short = 'C', long, default_value = "auto")]
    color: String,
}

/// Main entry point for the rutree2 application.
///
/// Parses command-line arguments and initiates the directory tree display.
fn main() {
    let cli = Cli::parse();

    // Configure colored output based on --color option
    match cli.color.as_str() {
        "never" => colored::control::set_override(false),
        "always" => colored::control::set_override(true),
        _ => {
            // Auto-detect: use colors if stdout is a terminal
            if !std::io::stdout().is_terminal() {
                colored::control::set_override(false);
            }
        }
    }

    match display_tree(&cli.path, cli.all, cli.depth, "", 0) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

/// Recursively displays a directory tree structure.
///
/// # Arguments
///
/// * `path` - The path to display
/// * `show_hidden` - Whether to show hidden files (starting with '.')
/// * `max_depth` - Maximum depth to traverse (None for unlimited)
/// * `prefix` - The prefix string for tree formatting
/// * `current_depth` - The current depth in the traversal
///
/// # Returns
///
/// Returns `Ok(())` on success, or an `std::io::Error` if directory reading fails.
///
/// # Examples
///
/// ```ignore
/// use std::path::Path;
/// let path = Path::new(".");
/// display_tree(&path, false, Some(2), "", 0).unwrap();
/// ```
fn display_tree(
    path: &Path,
    show_hidden: bool,
    max_depth: Option<usize>,
    prefix: &str,
    current_depth: usize,
) -> std::io::Result<()> {
    // Check if we've reached max depth
    if let Some(max) = max_depth
        && current_depth > max
    {
        return Ok(());
    }

    // Get the file name
    let name = path.file_name().and_then(|n| n.to_str()).unwrap_or(".");

    // Print current directory/file
    if current_depth == 0 {
        let colored_name = colorize_filename(name, path);
        println!("{}", colored_name);
    }

    // Check if it's a directory
    if path.is_dir() {
        // Read directory entries
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(Result::ok)
            .filter(|entry| {
                // Filter hidden files if needed
                if !show_hidden && let Some(name) = entry.file_name().to_str() {
                    return !name.starts_with('.');
                }
                true
            })
            .collect();

        // Sort entries by name
        entries.sort_by_key(|entry| entry.file_name());

        let total = entries.len();
        for (index, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_last = index == total - 1;

            let (connector, new_prefix) = if is_last {
                ("└── ", format!("{}    ", prefix))
            } else {
                ("├── ", format!("{}│   ", prefix))
            };

            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            // Add directory indicator
            let display_name = if path.is_dir() {
                format!("{}/", name_str)
            } else {
                name_str.to_string()
            };

            // Colorize the filename based on permissions and type
            let colored_name = colorize_filename(&display_name, &path);

            println!("{}{}{}", prefix, connector, colored_name);

            // Recursively display subdirectories
            if path.is_dir() {
                display_tree(
                    &path,
                    show_hidden,
                    max_depth,
                    &new_prefix,
                    current_depth + 1,
                )?;
            }
        }
    }

    Ok(())
}

/// Colorize a file name based on its metadata (permissions and file type).
///
/// # Arguments
///
/// * `name` - The file name to colorize
/// * `path` - The path to the file (to check metadata)
///
/// # Returns
///
/// A colored string based on file type and permissions:
/// - Directories: Blue and bold
/// - Executable files: Green
/// - Writable by others (o+w): Yellow (warning)
/// - Symlinks: Cyan
/// - Default: No color
fn colorize_filename(name: &str, path: &Path) -> ColoredString {
    // Check if it's a symlink first (using symlink_metadata to avoid following the link)
    if path
        .symlink_metadata()
        .map(|m| m.is_symlink())
        .unwrap_or(false)
    {
        return name.cyan();
    }

    // Try to get metadata for the path (follows symlinks if present)
    let metadata = match path.metadata() {
        Ok(m) => m,
        Err(_) => return name.normal(), // If we can't read metadata, return uncolored
    };

    // Check if it's a directory
    if metadata.is_dir() {
        return name.blue().bold();
    }

    // Get file permissions (Unix-specific)
    #[cfg(unix)]
    {
        let mode = metadata.permissions().mode();

        // Check if file is executable (0o111 = user/group/other execute bits)
        let is_executable = mode & 0o111 != 0;

        // Check if file is writable by others (0o002 = other write bit)
        let is_world_writable = mode & 0o002 != 0;

        if is_world_writable {
            return name.yellow(); // Warning: writable by others
        } else if is_executable {
            return name.green(); // Executable file
        }
    }

    // Default: no special color
    name.normal()
}
