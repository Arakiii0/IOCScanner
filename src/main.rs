use clap::Parser;
use std::path::{Path, PathBuf};
use std::{fs, io};
use regex::Regex;
use walkdir::WalkDir;
use serde::Serialize;



#[derive(Parser, Debug)]
#[command(version = "1.0", about = "IOC Scanner")]
struct Args {
    #[arg(short = 'p', long)]
    path: Option<PathBuf>,

    #[arg(short = 'e', long)]
    ext: Option<String>,
}

#[derive(Serialize)]
struct MatchRecord {
    file: String,
    ioc_type: String,
    value: String,
}



fn find_ipv4(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b(?:(?:25[0-5]|2[0-4]\d|1?\d{1,2})\.){3}(?:25[0-5]|2[0-4]\d|1?\d{1,2})\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn find_domain(text: &str) -> Vec<String> {
    let re = Regex::new(
        r"\b(?:[a-zA-Z0-9-]+\.)+(?:com|net|org|info|io|gov|edu|biz|co)\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn find_hash(text: &str) -> Vec<String> {
    let re = Regex::new(
        r"\b[a-fA-F0-9]{32}\b|\b[a-fA-F0-9]{40}\b|\b[a-fA-F0-9]{64}\b").unwrap();
    re.find_iter(text)
        .map(|m| m.as_str().to_string())
        .collect()
}

fn gather_files(path: &Path, exts: Option<&[&str]>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    if path.is_file() {
        out.push(path.to_path_buf());
        return out
    }

    for entry in WalkDir::new(path)
                        .into_iter()
                        .filter_map(|e| e.ok()) {
        let p = entry.path();
        if p.is_file() {
            if let Some(exts) = exts {
                if let Some(ext) = p.extension().and_then(|s| s.to_str()) {
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

fn write_reports(results: &[MatchRecord]) -> io::Result<()> {
    use std::fs::File;

    // JSON output
    let json_file = File::create("scan_results.json")?;
    serde_json::to_writer_pretty(json_file, results)
        .expect("Failed to write JSON");

    println!("📁 Reports saved: scan_results.json");
    Ok(())
}



fn main() -> io::Result<()> {
    let args: Args = Args::parse();

    if let Some(path) = args.path {
        println!("🧭 Scanning Path: {}\n", path.display());

        let exts: Option<Vec<&str>> = args.ext.as_ref().map(|s| vec![s.as_str()]);
        let files = gather_files(&path, exts.as_deref());
        let mut results: Vec<MatchRecord> = Vec::new();

        for file in files {
            println!("📄 Scanning: {}", file.display());

            match fs::read_to_string(&file) {
                Ok(content) => {
                    let ipv4s = find_ipv4(&content);
                    let domains = find_domain(&content);
                    let hashes = find_hash(&content);

                    for v in ipv4s {
                        results.push(MatchRecord {
                            file: file.display().to_string(),
                            ioc_type: "IPv4".into(),
                            value: v,
                        });
                    }

                    for v in domains {
                        results.push(MatchRecord {
                            file: file.display().to_string(),
                            ioc_type: "Domain".into(),
                            value: v,
                        });
                    }
                    
                    for v in hashes {
                        results.push(MatchRecord {
                            file: file.display().to_string(),
                            ioc_type: "Hash".into(),
                            value: v,
                        });
                    }
                }
                Err(e) => eprintln!("  ⚠️ Error reading {}: {}\n", file.display(), e),
            }
        }
        write_reports(&results)?;
        println!("\n✅ Scan complete! Found {} IOCs.", results.len());
    } else {
        println!("⚠️  No path provided. Use: IOCScanner.exe -p <path>");
    }
    Ok(())
}
