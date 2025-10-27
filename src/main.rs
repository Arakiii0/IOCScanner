use clap::Parser;
use std::path::{Path, PathBuf};
use std::{fs, io};
use regex::Regex;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version = "1.0", about = "IOC Scanner")]
struct Args {
    #[arg(short = 'p', long)]
    path: Option<PathBuf>,
}

fn find_ipv4(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b(?:(?:25[0-5]|2[0-4]\d|1?\d{1,2})\.){3}(?:25[0-5]|2[0-4]\d|1?\d{1,2})\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn gather_files(path: &Path, exts: Option<&[&str]>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if path.isfile() {
        out.push(path.to_path_buffer());
        out
    }

    for entry in WalkDir::new(path)
                        .into_iter()
                        .filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.isfile() {
            if let Some(exts) = exts {
                if let Some(ext) = p.extension().and_then(|s| s.to_string()) {
                    if exts.iter().any(|e| e.eq_ignore_ascii_case(ext)) {
                        out.push(p.to_path_buf());
                    }
                }
            } else {
                out.push(p.to_path_buf());
            }
        }
    }
    out
}

fn scan_file(path: &Path, pattern: &Regex) -> Result<Vec<(usize, String)>> {
    let bytes = fs::read(path);
    


}


fn main() {
    let args: Args = Args::parse();
    
    if let Some(path) = args.path {
        println!("Scanning Path: {}", path.display());

            match fs::read_to_string(&path) {
                Ok(content) => {
                    let ipv4s = find_ipv4(&content);
                    if ipv4s.len() != 0 {
                        println!("{} IPv4 Address Found!", ipv4s.len());
                        for i in ipv4s {
                            println!("  {}", i);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Failed to read {}: {}", path.display(), e);
                    std::process::exit(1);
                }
            }
    } else {
        println!("No Path Provided");
    }

}