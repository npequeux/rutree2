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

// Unix permission bit constants
#[cfg(unix)]
const SETUID_BIT: u32 = 0o4000;
#[cfg(unix)]
const SETGID_BIT: u32 = 0o2000;
#[cfg(unix)]
const STICKY_BIT: u32 = 0o1000;
#[cfg(unix)]
const EXECUTABLE_BITS: u32 = 0o111;
#[cfg(unix)]
const WORLD_WRITABLE_BIT: u32 = 0o002;

// Tree drawing characters
const TREE_BRANCH: &str = "├── ";
const TREE_LAST: &str = "└── ";
const TREE_VERTICAL: &str = "│   ";
const TREE_SPACE: &str = "    ";

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

    // Validate the path exists
    if !cli.path.exists() {
        eprintln!("Error: Path '{}' does not exist", cli.path.display());
        std::process::exit(1);
    }

    match display_tree(&cli.path, cli.all, cli.depth, "", 0) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error reading directory: {}", e);
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
                (TREE_LAST, format!("{}{}", prefix, TREE_SPACE))
            } else {
                (TREE_BRANCH, format!("{}{}", prefix, TREE_VERTICAL))
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
            // Check for sticky bit on directories
            if mode & STICKY_BIT != 0 {
                return name.green().on_blue(); // Sticky bit directory (e.g., /tmp)
            }
        }
        return name.blue().bold();
    }

    // Get file permissions (Unix-specific)
    #[cfg(unix)]
    {
        let mode = metadata.permissions().mode();

        // Check for setuid bit
        let is_setuid = mode & SETUID_BIT != 0;

        // Check for setgid bit
        let is_setgid = mode & SETGID_BIT != 0;

        // Check if file is executable
        let is_executable = mode & EXECUTABLE_BITS != 0;

        // Check if file is writable by others
        let is_world_writable = mode & WORLD_WRITABLE_BIT != 0;

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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs as unix_fs;

    #[test]
    fn test_colorize_filename_directory() {
        let temp_dir = std::env::temp_dir().join("rutree2_test_dir");
        fs::create_dir_all(&temp_dir).unwrap();

        let colored = colorize_filename("test_dir/", &temp_dir);
        // Directories should be colored (we can't easily test the exact color without complex setup)
        assert!(!colored.to_string().is_empty());

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_colorize_filename_archive() {
        let name = "test.zip";
        let path = Path::new(name);
        let colored = colorize_filename(name, path);
        // Should return colored string
        assert_eq!(colored.to_string(), name);
    }

    #[test]
    fn test_colorize_filename_image() {
        let name = "test.png";
        let path = Path::new(name);
        let colored = colorize_filename(name, path);
        assert_eq!(colored.to_string(), name);
    }

    #[test]
    fn test_colorize_filename_video() {
        let name = "test.mp4";
        let path = Path::new(name);
        let colored = colorize_filename(name, path);
        assert_eq!(colored.to_string(), name);
    }

    #[cfg(unix)]
    #[test]
    fn test_colorize_filename_executable() {
        use std::fs::File;
        use std::os::unix::fs::PermissionsExt;

        let temp_file = std::env::temp_dir().join("rutree2_test_exec");
        File::create(&temp_file).unwrap();

        let mut perms = fs::metadata(&temp_file).unwrap().permissions();
        perms.set_mode(0o755); // Make it executable
        fs::set_permissions(&temp_file, perms).unwrap();

        let colored = colorize_filename("test_exec", &temp_file);
        assert!(!colored.to_string().is_empty());

        fs::remove_file(&temp_file).unwrap();
    }

    #[test]
    fn test_display_tree_file_instead_of_directory() {
        let temp_file = std::env::temp_dir().join("rutree2_test_file");
        fs::write(&temp_file, "content").unwrap();

        // Calling display_tree on a file (not a directory) should still work
        // as it handles files gracefully
        let result = display_tree(&temp_file, false, None, "", 0);
        assert!(result.is_ok());

        fs::remove_file(&temp_file).unwrap();
    }

    #[test]
    fn test_display_tree_with_depth_limit() {
        // Create a temporary directory structure
        let temp_dir = std::env::temp_dir().join("rutree2_test_depth");
        fs::create_dir_all(temp_dir.join("level1/level2/level3")).unwrap();

        // Test with depth limit
        let result = display_tree(&temp_dir, false, Some(1), "", 0);
        assert!(result.is_ok());

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_display_tree_show_hidden() {
        let temp_dir = std::env::temp_dir().join("rutree2_test_hidden");
        fs::create_dir_all(&temp_dir).unwrap();
        fs::write(temp_dir.join(".hidden"), "hidden content").unwrap();
        fs::write(temp_dir.join("visible"), "visible content").unwrap();

        // Test with show_hidden = true
        let result = display_tree(&temp_dir, true, None, "", 0);
        assert!(result.is_ok());

        // Test with show_hidden = false
        let result = display_tree(&temp_dir, false, None, "", 0);
        assert!(result.is_ok());

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn test_display_tree_with_symlink() {
        let temp_dir = std::env::temp_dir().join("rutree2_test_symlink");
        fs::create_dir_all(&temp_dir).unwrap();
        let target = temp_dir.join("target");
        let link = temp_dir.join("link");
        fs::write(&target, "content").unwrap();
        unix_fs::symlink(&target, &link).unwrap();

        let result = display_tree(&temp_dir, false, None, "", 0);
        assert!(result.is_ok());

        fs::remove_dir_all(&temp_dir).unwrap();
    }

    #[test]
    fn test_cli_default_values() {
        use clap::Parser;

        // Test default CLI values
        let cli = Cli::parse_from(["rutree2"]);
        assert_eq!(cli.path, PathBuf::from("."));
        assert!(!cli.all);
        assert_eq!(cli.depth, None);
        assert_eq!(cli.color, "auto");
    }

    #[test]
    fn test_cli_with_all_flag() {
        use clap::Parser;

        let cli = Cli::parse_from(["rutree2", "--all"]);
        assert!(cli.all);
    }

    #[test]
    fn test_cli_with_depth() {
        use clap::Parser;

        let cli = Cli::parse_from(["rutree2", "--depth", "3"]);
        assert_eq!(cli.depth, Some(3));
    }

    #[test]
    fn test_cli_with_color() {
        use clap::Parser;

        let cli = Cli::parse_from(["rutree2", "--color", "always"]);
        assert_eq!(cli.color, "always");
    }

    #[test]
    fn test_cli_with_path() {
        use clap::Parser;

        let cli = Cli::parse_from(["rutree2", "/tmp"]);
        assert_eq!(cli.path, PathBuf::from("/tmp"));
    }
}
