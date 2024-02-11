use win_fast_rm::{delete_path, resolve_path};

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

    let start = std::time::Instant::now();
    let full_path = resolve_path(path);

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
        "Deleted \"{}\" successfully in: {}ms",
        resolve_path(path).display(),
        duration.as_micros() as f64 / 1000.0
    );

    std::process::exit(0);
}
