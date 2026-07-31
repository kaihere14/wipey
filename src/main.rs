use clap::Parser;
use inquire::MultiSelect;
use std::ffi::OsStr;
use std::io::{IsTerminal, Write};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Root directory to start the search from
    #[arg(short = 'r', long)]
    root: PathBuf,

    /// Folder name(s) to delete. Repeat the flag or separate with commas:
    /// `-t node_modules -t target` or `-t node_modules,target`
    #[arg(
        short = 't',
        long = "target",
        required = true,
        num_args = 1..,
        value_delimiter = ','
    )]
    target: Vec<String>,

    /// Print what would be deleted, then exit without touching anything
    #[arg(short = 'd', long)]
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

const FRAMES: [char; 4] = ['|', '/', '-', '\\'];

/// A background spinner for the slow filesystem passes.
///
/// Draws to stderr so piping stdout to a file stays clean, and stays silent
/// entirely when stderr isn't a terminal. Stops itself on drop.
struct Spinner {
    stop: Arc<AtomicBool>,
    handle: Option<thread::JoinHandle<()>>,
}

impl Spinner {
    /// `counter` is polled by the spinner thread so the label shows live
    /// progress while the main thread keeps working.
    fn start(label: &'static str, noun: &'static str, counter: Arc<AtomicUsize>) -> Self {
        if !std::io::stderr().is_terminal() {
            return Spinner {
                stop: Arc::new(AtomicBool::new(true)),
                handle: None,
            };
        }

        let stop = Arc::new(AtomicBool::new(false));
        let thread_stop = Arc::clone(&stop);

        let handle = thread::spawn(move || {
            let mut frame = 0usize;
            while !thread_stop.load(Ordering::Relaxed) {
                let n = counter.load(Ordering::Relaxed);
                // \r returns to column 0, \x1b[2K wipes the line so shorter
                // labels don't leave debris from longer ones.
                eprint!(
                    "\r\x1b[2K{} {}... ({} {})",
                    FRAMES[frame % FRAMES.len()],
                    label,
                    n,
                    noun
                );
                let _ = std::io::stderr().flush();
                frame += 1;
                thread::sleep(Duration::from_millis(80));
            }
        });

        Spinner {
            stop,
            handle: Some(handle),
        }
    }
}

impl Drop for Spinner {
    fn drop(&mut self) {
        self.stop.store(true, Ordering::Relaxed);
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
            eprint!("\r\x1b[2K");
            let _ = std::io::stderr().flush();
        }
    }
}

fn find_matches(root: &PathBuf, targets: &[String], found: &AtomicUsize) -> Vec<PathBuf> {
    let mut matches = Vec::new();
    let mut it = WalkDir::new(root).into_iter();

    while let Some(entry) = it.next() {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let name = entry.file_name();
        if entry.file_type().is_dir() && targets.iter().any(|t| name == OsStr::new(t)) {
            matches.push(entry.path().to_path_buf());
            found.fetch_add(1, Ordering::Relaxed);
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

    let found = Arc::new(AtomicUsize::new(0));
    let spinner = Spinner::start("Scanning", "found", Arc::clone(&found));
    let matches = find_matches(&args.root, &args.target, &found);
    drop(spinner);

    if matches.is_empty() {
        println!("No folders named {} found.", quoted_list(&args.target));
        return;
    }

    // Sizing walks every match a second time, so it gets a spinner too.
    let sized = Arc::new(AtomicUsize::new(0));
    let spinner = Spinner::start("Measuring", "sized", Arc::clone(&sized));
    let mut entries: Vec<FolderEntry> = matches
        .into_iter()
        .map(|path| {
            let size = folder_size(&path);
            sized.fetch_add(1, Ordering::Relaxed);
            FolderEntry { path, size }
        })
        .collect();
    drop(spinner);

    // Biggest first — the whole point is reclaiming space.
    entries.sort_by_key(|e| std::cmp::Reverse(e.size));

    let total: u64 = entries.iter().map(|e| e.size).sum();
    println!(
        "Found {} folder(s) matching {} — {} total.",
        entries.len(),
        quoted_list(&args.target),
        human_size(total)
    );

    let selected = MultiSelect::new("Select folders to delete:", entries)
        .prompt()
        .unwrap_or_default();

    if selected.is_empty() {
        println!("Nothing selected.");
        return;
    }

    let reclaimed: u64 = selected.iter().map(|e| e.size).sum();

    if args.dry_run {
        println!("\n[DRY RUN] The following would be deleted:");
        for entry in &selected {
            println!("  {}  ({})", entry.path.display(), human_size(entry.size));
        }
        println!("[DRY RUN] Would free {}.", human_size(reclaimed));
        return;
    }

    println!("\nDeleting {} folder(s)...", selected.len());
    let mut freed = 0u64;
    for entry in &selected {
        match std::fs::remove_dir_all(&entry.path) {
            Ok(()) => {
                freed += entry.size;
                println!("  Deleted: {}", entry.path.display());
            }
            Err(e) => println!("  Failed to delete {}: {}", entry.path.display(), e),
        }
    }
    println!("Freed {}.", human_size(freed));
}

/// Renders the target list for messages: `"node_modules"` or
/// `"node_modules", "target"`.
fn quoted_list(items: &[String]) -> String {
    items
        .iter()
        .map(|s| format!("\"{s}\""))
        .collect::<Vec<_>>()
        .join(", ")
}
