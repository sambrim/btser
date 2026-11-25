use regex::Regex;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

const VIDEO_EXTENSIONS: &[&str] = &[
    "mkv", "mp4", "avi", "mov", "wmv", "m4v", "ts", "m2ts", "webm", "flv",
];

fn get_patterns() -> &'static [Regex] {
    static PATTERNS: OnceLock<Vec<Regex>> = OnceLock::new();
    PATTERNS.get_or_init(|| {
        vec![
            // Standard: S01E10, s1e3, S01 E10
            Regex::new(r"(?i)\bS(\d{1,2})\s*E(\d{1,3})\b").unwrap(),
            // Odd formats with EP: S01 EP. 10, S01-EP10, S01 EP 10
            Regex::new(r"(?i)\bS(\d{1,2})\s*[-.]?\s*EP\.?\s*(\d{1,3})\b").unwrap(),
            // Written out: Season 1 Episode 10
            Regex::new(r"(?i)\bSeason\s*(\d{1,2})\s*Episode\s*(\d{1,3})\b").unwrap(),
            // Multiplier format: 1x10
            Regex::new(r"(?i)\b(\d{1,2})x(\d{1,3})\b").unwrap(),
            // Dash separated without E/EP: S01 - 10
            Regex::new(r"(?i)\bS(\d{1,2})\s*-\s*(\d{1,3})\b").unwrap(),
        ]
    })
}

fn get_season_episode(filename: &str) -> Option<(u32, u32)> {
    for re in get_patterns() {
        if let Some(caps) = re.captures(filename) {
            let season = caps.get(1)?.as_str().parse::<u32>().ok()?;
            let episode = caps.get(2)?.as_str().parse::<u32>().ok()?;
            return Some((season, episode));
        }
    }
    None
}

fn rename_videos() {
    let folder = Path::new(".");
    let entries = match fs::read_dir(folder) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("Failed to read current directory: {}", e);
            return;
        }
    };

    // Collect and sort paths for predictable execution order
    let mut files: Vec<PathBuf> = entries
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .collect();
    files.sort();

    for file in files {
        let ext = match file.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => continue,
        };

        if !VIDEO_EXTENSIONS.contains(&ext.as_str()) {
            continue;
        }

        let file_name = match file.file_name().and_then(|n| n.to_str()) {
            Some(name) => name,
            None => continue,
        };

        let (season, episode) = match get_season_episode(file_name) {
            Some(res) => res,
            None => {
                println!("Skipping (no episode number): {}", file_name);
                continue;
            }
        };

        let new_name = format!("s{:02}e{:02}.{}", season, episode, ext);
        let new_path = file.with_file_name(&new_name);

        if new_path == file {
            println!("Already correct: {}", file_name);
            continue;
        }

        if new_path.exists() {
            println!("Skipping (destination exists): {} -> {}", file_name, new_name);
            continue;
        }

        match fs::rename(&file, &new_path) {
            Ok(_) => println!("Renamed: {} -> {}", file_name, new_name),
            Err(e) => eprintln!("Error renaming {} -> {}: {}", file_name, new_name, e),
        }
    }
}

fn main() {
    rename_videos();
}
