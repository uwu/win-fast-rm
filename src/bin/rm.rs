use std::path::PathBuf;

use win_fast_rm::{delete_path, resolve_path};

fn humanize_size(size: u64) -> String {
    // Convert to nearest unit iteratively (KB, MB, GB, TB)
    let mut size = size as f64;
    for unit in [" bytes", " KB", " MB", " GB", " TB"].iter() {
        if size < 1024.0 {
            return format!("{:.2}{}", size, unit);
        }
        size /= 1024.0;
    }

    format!("{:.2}{}", size, " TB")
}

fn get_directory_size(path: &PathBuf) -> u64 {
    let mut size = 0;

    if !path.exists() {
        return 0;
    } else if path.is_file() {
        return path.metadata().unwrap().len();
    }

    for entry in path.read_dir().unwrap() {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();
        if metadata.is_dir() {
            size += get_directory_size(&entry.path());
        } else {
            size += metadata.len();
        }
    }
    size
}

fn main() {
    #[cfg(not(windows))]
    {
        eprintln!("This program is only intended for Windows.");
        std::process::exit(1);
    }
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: rm <file>");
        std::process::exit(1);
    }

    let path = std::ffi::OsStr::new(&args[1]);

    let full_path = resolve_path(path);
    let size = humanize_size(get_directory_size(&full_path));

    let start = std::time::Instant::now();

    if let Err(e) = delete_path(&full_path) {
        let error_code = e.code().0;
        match e.code().0 as u32 {
            0x80070002 => eprintln!("Invalid path. Check if the path is correct and try again.\nAttempted to delete directory or file at \"{}\"", resolve_path(path).display()),
            0x80070005 => eprintln!("Access denied. Try running as administrator, or check if the directory or file is already in use."),
            _ => eprintln!("Unknown error (exit code {}): {}", e.code(), e.message()),
        }
        std::process::exit(error_code);
    }

    let duration = start.elapsed();

    println!(
        "Deleted \"{}\"({}) successfully in: {}ms",
        resolve_path(path).display(),
        size,
        duration.as_micros() as f64 / 1000.0
    );

    std::process::exit(0);
}
