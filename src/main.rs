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
use colored::{Colorize as ColoredColorize, ColoredString};
use std::fs;
use std::io::IsTerminal;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

// For interactive mode
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, ListState};
use crossterm::event::{self, Event, KeyCode};
use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use std::time::Duration;
// Tree drawing constants for interactive mode
const TREE_LAST: &str = "└── ";
const TREE_BRANCH: &str = "├── ";
const TREE_SPACE: &str = "    ";
const TREE_VERTICAL: &str = "│   ";
// Unix permission bit constants for file mode checking
#[cfg(unix)]
const MODE_STICKY_BIT: u32 = 0o1000; // Sticky bit (e.g., /tmp directories)
#[cfg(unix)]
const MODE_SETUID: u32 = 0o4000; // Set user ID on execution
#[cfg(unix)]
const MODE_SETGID: u32 = 0o2000; // Set group ID on execution
#[cfg(unix)]
const MODE_EXECUTABLE: u32 = 0o111; // User/group/other execute bits
#[cfg(unix)]
const MODE_WORLD_WRITABLE: u32 = 0o002; // World writable bit

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
    #[arg(short = 'C', long, default_value = "auto", value_parser = validate_color)]
    color: String,

    /// Interactive collapsible/expandable tree view
    #[arg(short = 'i', long, help = "Interactive collapsible/expandable tree view")]
    interactive: bool,
}

/// Validates the color argument value
fn validate_color(s: &str) -> Result<String, String> {
    match s {
        "auto" | "always" | "never" => Ok(s.to_string()),
        _ => Err(format!(
            "invalid color value '{}', must be one of: auto, always, never",
            s
        )),
    }
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

    if cli.interactive {
        if let Err(e) = interactive_tree(&cli.path, cli.all, cli.depth) {
            eprintln!("Interactive mode error: {}", e);
            std::process::exit(1);
        }
    } else {
        match display_tree(&cli.path, cli.all, cli.depth, "", 0) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("Error reading directory '{}': {}", cli.path.display(), e);
                std::process::exit(1);
            }
            }
        }
    }


/// Interactive collapsible/expandable tree using ratatui
fn interactive_tree(path: &Path, show_hidden: bool, max_depth: Option<usize>) -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::terminal::EnterAlternateScreen)?;
    let backend = ratatui::backend::CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Build initial tree state
    let tree = TreeNode::from_path(path, show_hidden, max_depth, 0)?;
    let mut state = ListState::default();
    state.select(Some(0));

    let mut flat = tree.flatten();

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let items: Vec<ListItem> = flat.iter().map(|(prefix, node)| {
                let mut label = format!("{}{}", prefix, node.display_name());
                if node.is_dir && !node.expanded {
                    label.push_str(" [+]");
                } else if node.is_dir && node.expanded {
                    label.push_str(" [-]");
                }
                ListItem::new(label)
            }).collect();
            let list = List::new(items).block(Block::default().borders(Borders::ALL).title("rutree2 (interactive)"));
            f.render_stateful_widget(list, size, &mut state);
        })?;

        if event::poll(Duration::from_millis(200))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Down => {
                        let sel = state.selected().unwrap_or(0);
                        if sel + 1 < flat.len() {
                            state.select(Some(sel + 1));
                        }
                    }
                    KeyCode::Up => {
                        let sel = state.selected().unwrap_or(0);
                        if sel > 0 {
                            state.select(Some(sel - 1));
                        }
                    }
                    KeyCode::Right | KeyCode::Enter => {
                        let sel = state.selected().unwrap_or(0);
                        if let Some((_, node)) = flat.get_mut(sel) {
                            if node.is_dir && !node.expanded {
                                node.expanded = true;
                                flat = tree.flatten();
                                state.select(Some(sel));
                            }
                        }
                    }
                    KeyCode::Left => {
                        let sel = state.selected().unwrap_or(0);
                        if let Some((_, node)) = flat.get_mut(sel) {
                            if node.is_dir && node.expanded {
                                node.expanded = false;
                                flat = tree.flatten();
                                state.select(Some(sel));
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(std::io::stdout(), crossterm::terminal::LeaveAlternateScreen)?;
    Ok(())
}

/// Tree node for interactive mode
#[derive(Debug, Clone)]
struct TreeNode {
    name: String,
    is_dir: bool,
    expanded: bool,
    children: Vec<TreeNode>,
    depth: usize,
}

impl TreeNode {
    fn from_path(path: &Path, show_hidden: bool, max_depth: Option<usize>, depth: usize) -> std::io::Result<Self> {
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or(".").to_string();
        let is_dir = path.is_dir();
        let mut node = TreeNode {
            name,
            is_dir,
            expanded: depth == 0, // root expanded
            children: vec![],
            depth,
        };
        if is_dir && (max_depth.map_or(true, |m| depth < m)) {
            let mut entries: Vec<_> = fs::read_dir(path)?
                .filter_map(Result::ok)
                .filter(|entry| {
                    if !show_hidden {
                        if let Some(name) = entry.file_name().to_str() {
                            return !name.starts_with('.');
                        }
                    }
                    true
                })
                .collect();
            entries.sort_by_key(|entry| entry.file_name());
            for entry in entries {
                let child = TreeNode::from_path(&entry.path(), show_hidden, max_depth, depth + 1)?;
                node.children.push(child);
            }
        }
        Ok(node)
    }

    fn display_name(&self) -> String {
        if self.is_dir {
            format!("{}/", self.name)
        } else {
            self.name.clone()
        }
    }

    /// Flatten the tree for display, returning (prefix, &TreeNode)
    fn flatten(&self) -> Vec<(String, TreeNode)> {
        let mut out = vec![];
        self.flatten_inner("", &mut out, true);
        out
    }

    fn flatten_inner(&self, prefix: &str, out: &mut Vec<(String, TreeNode)>, is_last: bool) {
        let mut this_prefix = prefix.to_string();
        if self.depth > 0 {
            this_prefix += if is_last { TREE_LAST } else { TREE_BRANCH };
        }
        out.push((this_prefix.clone(), self.clone()));
        if self.is_dir && self.expanded {
            let n = self.children.len();
            for (i, child) in self.children.iter().enumerate() {
                let mut child_prefix = prefix.to_string();
                if self.depth > 0 {
                    child_prefix += if is_last { TREE_SPACE } else { TREE_VERTICAL };
                }
                child.flatten_inner(&child_prefix, out, i == n - 1);
            }
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
    #[allow(clippy::collapsible_if)]
    if let Some(max) = max_depth {
        if current_depth > max {
            return Ok(());
        }
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
                #[allow(clippy::collapsible_if)]
                if !show_hidden {
                    if let Some(name) = entry.file_name().to_str() {
                        return !name.starts_with('.');
                    }
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

            // Get metadata once and reuse it
            let symlink_meta = path.symlink_metadata().ok();
            let is_symlink = symlink_meta.as_ref().is_some_and(|m| m.is_symlink());

            // Check if it's a symlink
            let display_name = if is_symlink {
                // Read the symlink target
                if let Ok(target) = fs::read_link(&path) {
                    let target_str = target.display();
                    // Add directory indicator for symlinks that point to directories
                    // Use path.is_dir() which follows symlinks to determine if target is a directory
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
/// Colors are applied in order of precedence (first match wins):
/// 1. Symlinks: Cyan
/// 2. Setuid files: White on red (highest priority - security sensitive)
/// 3. Setgid files: Black on yellow (high priority - security sensitive)  
/// 4. Sticky bit directories: Green on blue (e.g., /tmp)
/// 5. Regular directories: Blue and bold
/// 6. Special files (devices, sockets, pipes): Yellow bold
/// 7. World-writable files: Yellow (warning)
/// 8. Executable files: Green
/// 9. Archive files: Red (.zip, .tar, .gz, etc.)
/// 10. Image files: Magenta (.png, .jpg, etc.)
/// 11. Audio/video files: Bright magenta (.mp3, .mp4, etc.)
/// 12. Default: No color
///
/// # Arguments
///
/// * `name` - The file name to colorize
/// * `path` - The path to the file (to check metadata)
///
/// # Returns
///
/// A colored string based on file type and permissions.
///
/// # Note
///
/// If metadata cannot be read (e.g., permission denied), the name is returned without coloring.
fn colorize_filename(name: &str, path: &Path) -> ColoredString {
    // Check if it's a symlink first (using symlink_metadata to avoid following the link)
    if path
        .symlink_metadata()
        .map(|m| m.is_symlink())
        .unwrap_or(false)
    {
        return <&str as ColoredCompat>::colored_cyan(name);
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
            if mode & MODE_STICKY_BIT != 0 {
                return <&str as ColoredCompat>::colored_green(name).on_blue(); // Sticky bit directory (e.g., /tmp)
            }
        }
        return <&str as ColoredCompat>::colored_blue(name).bold();
    }


    #[cfg(unix)] {
        let mode = metadata.permissions().mode();
        // Check for setuid bit
        let is_setuid = mode & MODE_SETUID != 0;
        // Check for setgid bit
        let is_setgid = mode & MODE_SETGID != 0;
        // Check if file is executable
        let is_executable = mode & MODE_EXECUTABLE != 0;
        // Check if file is writable by others
        let is_world_writable = mode & MODE_WORLD_WRITABLE != 0;
        // Check for special file types using file_type()
        let file_type = metadata.file_type();
        use std::os::unix::fs::FileTypeExt;
        // Character or block devices
        if file_type.is_char_device() || file_type.is_block_device() {
            return <&str as ColoredCompat>::colored_yellow(name).bold();
        }
        // Socket or FIFO (named pipe)
        if file_type.is_socket() || file_type.is_fifo() {
            return <&str as ColoredCompat>::colored_yellow(name);
        }
        // Setuid files (highest priority - security sensitive)
        if is_setuid {
            return <&str as ColoredCompat>::colored_white(name).on_red(); // White text on red background
        }
        // Setgid files (high priority - security sensitive)
        if is_setgid {
            return <&str as ColoredCompat>::colored_black(name).on_yellow(); // Black text on yellow background
        }
        // World-writable files (warning)
        if is_world_writable {
            return <&str as ColoredCompat>::colored_yellow(name);
        }
        // Executable files
        if is_executable {
            return <&str as ColoredCompat>::colored_green(name);
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
            return <&str as ColoredCompat>::colored_red(name);
        }
        // Image files
        if matches!(
            ext_lower.as_str(),
            "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "ico" | "webp" | "tiff" | "tif"
        ) {
            return <&str as ColoredCompat>::colored_magenta(name);
        }
        // Audio/video files
        if matches!(
            ext_lower.as_str(),
            "mp3" | "mp4" | "avi" | "mkv" | "flac" | "wav" | "ogg" | "mov" | "wmv" | "webm" | "m4a"
        ) {
            return <&str as ColoredCompat>::colored_bright_magenta(name);
        }
    }

    // Default: no special color
    name.normal()
}

// Disambiguate colored methods
trait ColoredCompat {
    fn colored_cyan(s: &str) -> ColoredString;
    fn colored_green(s: &str) -> ColoredString;
    fn colored_blue(s: &str) -> ColoredString;
    fn colored_yellow(s: &str) -> ColoredString;
    fn colored_white(s: &str) -> ColoredString;
    fn colored_black(s: &str) -> ColoredString;
    fn colored_red(s: &str) -> ColoredString;
    fn colored_magenta(s: &str) -> ColoredString;
    fn colored_bright_magenta(s: &str) -> ColoredString;
}

impl ColoredCompat for &str {
    fn colored_cyan(s: &str) -> ColoredString { colored::Colorize::cyan(s) }
    fn colored_green(s: &str) -> ColoredString { colored::Colorize::green(s) }
    fn colored_blue(s: &str) -> ColoredString { colored::Colorize::blue(s) }
    fn colored_yellow(s: &str) -> ColoredString { colored::Colorize::yellow(s) }
    fn colored_white(s: &str) -> ColoredString { colored::Colorize::white(s) }
    fn colored_black(s: &str) -> ColoredString { colored::Colorize::black(s) }
    fn colored_red(s: &str) -> ColoredString { colored::Colorize::red(s) }
    fn colored_magenta(s: &str) -> ColoredString { colored::Colorize::magenta(s) }
    fn colored_bright_magenta(s: &str) -> ColoredString { colored::Colorize::bright_magenta(s) }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_cli_interactive_flag() {
        use clap::Parser;
        let cli = Cli::parse_from(["rutree2", "-i"]);
        assert!(cli.interactive);
    }

    use super::*;
    use std::fs::{self, File};
    use std::path::PathBuf;

    /// Helper function to create a temporary test directory
    fn create_test_dir() -> (PathBuf, tempfile::TempDir) {
        let temp_dir = tempfile::TempDir::new().expect("Failed to create temp dir");
        let path = temp_dir.path().to_path_buf();
        (path, temp_dir)
    }

    #[test]
    fn test_validate_color_valid_values() {
        assert!(validate_color("auto").is_ok());
        assert!(validate_color("always").is_ok());
        assert!(validate_color("never").is_ok());
    }

    #[test]
    fn test_validate_color_invalid_value() {
        let result = validate_color("invalid");
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .contains("must be one of: auto, always, never")
        );
    }

    #[test]
    fn test_display_tree_empty_directory() {
        let (test_dir, _temp) = create_test_dir();
        let result = display_tree(&test_dir, false, None, "", 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_tree_with_files() {
        let (test_dir, _temp) = create_test_dir();

        // Create some test files
        File::create(test_dir.join("file1.txt")).expect("Failed to create file");
        File::create(test_dir.join("file2.rs")).expect("Failed to create file");
        fs::create_dir(test_dir.join("subdir")).expect("Failed to create directory");

        let result = display_tree(&test_dir, false, None, "", 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_tree_hidden_files() {
        let (test_dir, _temp) = create_test_dir();

        // Create hidden and visible files
        File::create(test_dir.join(".hidden")).expect("Failed to create file");
        File::create(test_dir.join("visible.txt")).expect("Failed to create file");

        // Should succeed with show_hidden=false
        let result = display_tree(&test_dir, false, None, "", 0);
        assert!(result.is_ok());

        // Should succeed with show_hidden=true
        let result = display_tree(&test_dir, true, None, "", 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_tree_max_depth() {
        let (test_dir, _temp) = create_test_dir();

        // Create nested directories
        let subdir1 = test_dir.join("level1");
        let subdir2 = subdir1.join("level2");
        let subdir3 = subdir2.join("level3");

        fs::create_dir(&subdir1).expect("Failed to create dir");
        fs::create_dir(&subdir2).expect("Failed to create dir");
        fs::create_dir(&subdir3).expect("Failed to create dir");

        // Test with depth limit
        let result = display_tree(&test_dir, false, Some(2), "", 0);
        assert!(result.is_ok());

        let result = display_tree(&test_dir, false, Some(0), "", 0);
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_tree_nonexistent_directory() {
        let nonexistent = PathBuf::from("/path/that/does/not/exist/directory");
        let result = display_tree(&nonexistent, false, None, "", 0);
        // For non-directory paths, display_tree returns Ok since it just checks is_dir()
        // which returns false for nonexistent paths without erroring
        assert!(result.is_ok());
    }

    #[test]
    fn test_colorize_filename_basic() {
        // Test that colorize_filename doesn't panic on basic inputs
        let temp_file = tempfile::NamedTempFile::new().expect("Failed to create temp file");
        let path = temp_file.path();

        // Disable colors for consistent testing
        colored::control::set_override(false);

        let result = colorize_filename("test.txt", path);
        assert_eq!(result.to_string(), "test.txt");

        // Re-enable colors
        colored::control::unset_override();
    }

    #[test]
    fn test_colorize_filename_archive_extensions() {
        let (test_dir, _temp) = create_test_dir();

        // Test various archive extensions
        let archives = vec!["test.zip", "test.tar", "test.gz", "test.7z"];

        for archive in archives {
            let file_path = test_dir.join(archive);
            File::create(&file_path).expect("Failed to create file");

            // Just verify it doesn't panic
            let _result = colorize_filename(archive, &file_path);
        }
    }

    #[test]
    fn test_colorize_filename_image_extensions() {
        let (test_dir, _temp) = create_test_dir();

        // Test various image extensions
        let images = vec!["test.png", "test.jpg", "test.jpeg", "test.gif"];

        for image in images {
            let file_path = test_dir.join(image);
            File::create(&file_path).expect("Failed to create file");

            // Just verify it doesn't panic
            let _result = colorize_filename(image, &file_path);
        }
    }

    #[test]
    fn test_colorize_filename_media_extensions() {
        let (test_dir, _temp) = create_test_dir();

        // Test various media extensions
        let media = vec!["test.mp3", "test.mp4", "test.avi", "test.mkv"];

        for media_file in media {
            let file_path = test_dir.join(media_file);
            File::create(&file_path).expect("Failed to create file");

            // Just verify it doesn't panic
            let _result = colorize_filename(media_file, &file_path);
        }
    }

    #[test]
    fn test_colorize_filename_directory() {
        let (test_dir, _temp) = create_test_dir();
        let subdir = test_dir.join("testdir");
        fs::create_dir(&subdir).expect("Failed to create directory");

        // Just verify it doesn't panic
        let _result = colorize_filename("testdir/", &subdir);
    }

    #[cfg(unix)]
    #[test]
    fn test_permission_constants() {
        // Verify our constants are correct
        assert_eq!(MODE_STICKY_BIT, 0o1000);
        assert_eq!(MODE_SETUID, 0o4000);
        assert_eq!(MODE_SETGID, 0o2000);
        assert_eq!(MODE_EXECUTABLE, 0o111);
        assert_eq!(MODE_WORLD_WRITABLE, 0o002);
    }
}
