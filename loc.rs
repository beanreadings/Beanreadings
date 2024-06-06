// Lines of code metric
// This is compiled using Rustc not Cargo

fn analyze_dir(dir: &std::path::Path, ignored_filenames: &[&str], ignored_dirs: &[&str]) -> u32 {
    let mut total_lines = 0;

    for file in std::fs::read_dir(dir).unwrap() {
        let file = file.unwrap();
        let path = file.path();

        if path.is_dir() {
            if ignored_dirs.contains(&path.file_name().unwrap().to_str().unwrap()) {
                continue;
            }

            total_lines += analyze_dir(&path, ignored_filenames, ignored_dirs);
        } else {
            if ignored_filenames.contains(&path.file_name().unwrap().to_str().unwrap()) {
                continue;
            }

            total_lines += count_lines(&path);
        }
    }

    total_lines
}

fn count_lines(path: &std::path::PathBuf) -> u32 {
    let mut total_lines = 0;

    let file = std::fs::read_to_string(path);

    if let Ok(file) = file {
        for line in file.lines() {
            if line.trim().is_empty() {
                continue;
            }

            total_lines += 1;
        }
    }

    total_lines
}

fn main() {
    let ignored_filenames = ["Cargo.lock"];
    let ignored_dirs = [".git", "target"];

    let chdir = std::env::current_dir().unwrap();

    // now here we count

    let total_lines = analyze_dir(&chdir, &ignored_filenames, &ignored_dirs);

    println!("Total lines of code: {}", total_lines);
}
