//! # rutree2
//!
//! A Rust command-line tool for displaying directory structures in a tree format.
//!
//! ## Features
//!
//! - Display directory trees with proper indentation and tree characters
//! - Visualize symbolic links with `->` indicator showing both origin and destination
//! - Show hidden files with the `-a` or `--all` flag
//! - Limit traversal depth with the `-d` or `--depth` option
//! - Sort entries alphabetically
//! - Enhanced color-coded output based on file types and permissions:
//!   - Setuid/setgid files (security sensitive)
//!   - Sticky bit directories
//!   - Executable files
//!   - World-writable files
//!   - Directories and symbolic links
//!   - Archive, image, and audio/video files
//!   - Special files (devices, sockets, pipes)
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

            // Check if it's a symlink
            let display_name = if let Ok(metadata) = path.symlink_metadata() {
                if metadata.is_symlink() {
                    // Read the symlink target
                    if let Ok(target) = fs::read_link(&path) {
                        let target_str = target.display();
                        // Add directory indicator for symlinks that point to directories
                        if path.is_dir() {
                            format!("{}/ -> {}", name_str, target_str)
                        } else {
                            format!("{} -> {}", name_str, target_str)
                        }
                    } else {
                        // Broken symlink
                        format!("{} -> [broken link]", name_str)
                    }
                } else if path.is_dir() {
                    format!("{}/", name_str)
                } else {
                    name_str.to_string()
                }
            } else {
                // Fallback if metadata can't be read
                if path.is_dir() {
                    format!("{}/", name_str)
                } else {
                    name_str.to_string()
                }
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
/// - Setuid files: White on red (security sensitive)
/// - Setgid files: Black on yellow (security sensitive)
/// - Sticky bit directories: Green on blue
/// - Executable files: Green
/// - World-writable files: Yellow (warning)
/// - Symlinks: Cyan
/// - Archive files: Red
/// - Image files: Magenta
/// - Audio/video files: Bright magenta
/// - Special files (devices, sockets, pipes): Yellow bold
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
        #[cfg(unix)]
        {
            let mode = metadata.permissions().mode();
            // Check for sticky bit (0o1000) on directories
            if mode & 0o1000 != 0 {
                return name.green().on_blue(); // Sticky bit directory (e.g., /tmp)
            }
        }
        return name.blue().bold();
    }

    // Get file permissions (Unix-specific)
    #[cfg(unix)]
    {
        let mode = metadata.permissions().mode();

        // Check for setuid bit (0o4000)
        let is_setuid = mode & 0o4000 != 0;

        // Check for setgid bit (0o2000)
        let is_setgid = mode & 0o2000 != 0;

        // Check if file is executable (0o111 = user/group/other execute bits)
        let is_executable = mode & 0o111 != 0;

        // Check if file is writable by others (0o002 = other write bit)
        let is_world_writable = mode & 0o002 != 0;

        // Check for special file types using file_type()
        let file_type = metadata.file_type();

        #[cfg(unix)]
        {
            use std::os::unix::fs::FileTypeExt;

            // Character or block devices
            if file_type.is_char_device() || file_type.is_block_device() {
                return name.yellow().bold();
            }

            // Socket or FIFO (named pipe)
            if file_type.is_socket() || file_type.is_fifo() {
                return name.yellow();
            }
        }

        // Setuid files (highest priority - security sensitive)
        if is_setuid {
            return name.white().on_red(); // White text on red background
        }

        // Setgid files (high priority - security sensitive)
        if is_setgid {
            return name.black().on_yellow(); // Black text on yellow background
        }

        // World-writable files (warning)
        if is_world_writable {
            return name.yellow();
        }

        // Executable files
        if is_executable {
            return name.green();
        }
    }

    // Check file extension for type-based coloring
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext_lower = ext.to_lowercase();

        // Archive files
        if matches!(
            ext_lower.as_str(),
            "zip" | "tar" | "gz" | "bz2" | "xz" | "7z" | "rar" | "tgz" | "tbz2" | "txz"
        ) {
            return name.red();
        }

        // Image files
        if matches!(
            ext_lower.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "ico" | "webp" | "tiff" | "tif"
        ) {
            return name.magenta();
        }

        // Audio/video files
        if matches!(
            ext_lower.as_str(),
            "mp3" | "mp4" | "avi" | "mkv" | "flac" | "wav" | "ogg" | "mov" | "wmv" | "webm" | "m4a"
        ) {
            return name.bright_magenta();
        }
    }

    // Default: no special color
    name.normal()
}
