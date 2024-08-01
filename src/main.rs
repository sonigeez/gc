use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use ignore::WalkBuilder;
use cli_clipboard::{ClipboardContext, ClipboardProvider};
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(default_value = ".")]
    path: PathBuf,
    
    #[arg(short, long)]
    stdout: bool,

    #[arg(short = 'H', long)]
    include_hidden: bool,

    #[arg(short, long)]
    no_ignore: bool,
    
    #[arg(short, long, value_delimiter = ',')]
    folders: Vec<String>,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();

    let output = files_to_string(&cli.path, cli.include_hidden, !cli.no_ignore, &cli.folders)?;

    if cli.stdout {
        println!("{}", output);
    } else {
        let mut ctx = ClipboardContext::new().expect("Failed to initialize clipboard");
        ctx.set_contents(output).expect("Failed to copy to clipboard");
        println!("Selected file contents have been copied to the clipboard.");
    }

    Ok(())
}
fn files_to_string(folder_path: &Path, include_hidden: bool, use_gitignore: bool, specified_folders: &[String]) -> io::Result<String> {
    let mut result = String::new();
    
    println!("Searching in path: {:?}", folder_path);
    println!("Specified folders: {:?}", specified_folders);
    println!("Include hidden: {}", include_hidden);
    println!("Use gitignore: {}", use_gitignore);

    if !folder_path.is_dir() {
        return Err(io::Error::new(io::ErrorKind::NotFound, format!("Path is not a directory: {:?}", folder_path)));
    }

    let walker = WalkBuilder::new(folder_path)
        .hidden(!include_hidden)
        .git_ignore(use_gitignore)
        .build();

    for entry in walker {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                println!("Checking entry: {:?}", path);

                if path.is_file() {
                    let parent_folder_name = path.parent()
                        .and_then(|p| p.file_name())
                        .and_then(|name| name.to_str())
                        .unwrap_or("");

                    println!("Parent folder: {}", parent_folder_name);

                    if specified_folders.is_empty() || specified_folders.contains(&parent_folder_name.to_string()) {
                        println!("Processing file: {:?}", path);

                        let comment_style = get_comment_style(path);

                        result.push_str(&format!("\n{} {}\n", comment_style, path.display()));

                        match fs::read_to_string(path) {
                            Ok(contents) => {
                                result.push_str(&contents);
                                result.push_str("\n");
                            },
                            Err(e) => println!("Error reading file {:?}: {}", path, e),
                        }

                        result.push_str(".\n");
                        if comment_style == "<!--" {
                            result.push_str("-->\n");
                        } else if comment_style == "/*" {
                            result.push_str("*/\n");
                        }
                    } else {
                        println!("Skipping file (not in specified folders): {:?}", path);
                    }
                } else {
                    println!("Skipping non-file entry: {:?}", path);
                }
            },
            Err(e) => println!("Error accessing entry: {}", e),
        }
    }

    if result.is_empty() {
        println!("No files were processed. Result is empty.");
    } else {
        println!("Processed {} characters", result.len());
    }

    Ok(result.trim().to_string())
}

fn get_comment_style(file_path: &Path) -> &'static str {
    match file_path.extension().and_then(|s| s.to_str()) {
        Some("js") | Some("ts") | Some("java") => "//",
        Some("py") | Some("rb") | Some("sh") => "#",
        Some("html") => "<!--",
        Some("css") => "/*",
        _ => "//",
    }
}