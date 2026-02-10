use clap::Parser;
use std::fs;
use std::path::{Path, PathBuf};

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
}

fn main() {
    let cli = Cli::parse();

    match display_tree(&cli.path, cli.all, cli.depth, "", 0) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    }
}

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
        println!("{}", name);
    }

    // Check if it's a directory
    if path.is_dir() {
        // Read directory entries
        let mut entries: Vec<_> = fs::read_dir(path)?
            .filter_map(Result::ok)
            .filter(|entry| {
                // Filter hidden files if needed
                if !show_hidden
                    && let Some(name) = entry.file_name().to_str()
                {
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

            println!("{}{}{}", prefix, connector, display_name);

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
