use clap::Parser;
use inquire::MultiSelect;
use std::path::PathBuf;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    //for selection of root path
    #[arg(short = 'r', long)]
    root: PathBuf,

    //folder name that would get deleted
    #[arg(short = 't', long)]
    target: String,

    // A final confirmation before deleting slected folders
    #[arg(short='d', long)]
    dry_run: bool,
}

struct FolderEntry {
    path: PathBuf,
    size: u64,
}

impl std::fmt::Display for FolderEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}  ({})", self.path.display(), human_size(self.size))
    }
}

fn find_matches(root: &PathBuf, target: &str) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let mut it = WalkDir::new(root).into_iter();

    while let Some(entry) = it.next() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        if entry.file_type().is_dir() && entry.file_name() == target {
            matches.push(entry.path().to_path_buf());
            it.skip_current_dir(); //if it wilm found that folder it will save and tahn skip and move to next dir
        }
    }

    matches
}

fn folder_size(path: &PathBuf) -> u64 {
    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.metadata().map(|m| m.len()).unwrap_or(0))
        .sum()
}

fn human_size(bytes: u64) -> String {
    const UNITS: [&str; 5] = ["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit = 0;
    while size >= 1024.0 && unit < UNITS.len() - 1 {
        size /= 1024.0;
        unit += 1;
    }
    format!("{:.2} {}", size, UNITS[unit])
}

fn main() {
    let args = Args::parse();
    let matches = find_matches(&args.root, &args.target);

    if matches.is_empty() {
        println!("No matching folders found.");
        return;
    }

    let entries: Vec<FolderEntry> = matches
        .into_iter()
        .map(|path| {
            let size = folder_size(&path);
            FolderEntry { path, size }
        })
        .collect();

    let selected = MultiSelect::new("Select folders to delete:", entries)
        .prompt()
        .unwrap_or_default();

    if selected.is_empty() {
        println!("Nothing selected.");
        return;
    }

    if args.dry_run {
        println!("\n[DRY RUN] The following would be deleted:");
        for entry in &selected {
            println!("  {}  ({})", entry.path.display(), human_size(entry.size));
        }
        return;
    }

    println!("\nDeleting {} folder(s)...", selected.len());
    for entry in &selected {
        match std::fs::remove_dir_all(&entry.path) {
            Ok(()) => println!("  Deleted: {}", entry.path.display()),
            Err(e) => println!("  Failed to delete {}: {}", entry.path.display(), e),
        }
    }
}
