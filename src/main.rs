use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("kamil torrent v0.0.1 - CLI mode");

    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: {} <folder-torrent> <folder-output>", args[0]);
        return;
    }

    let torrent_folder = Path::new(&args[1]);
    let output_folder = Path::new(&args[2]);

    if !torrent_folder.is_dir() {
        eprintln!("Error: '{}' bukan folder yang valid.", torrent_folder.display());
        return;
    }

    if !output_folder.exists() {
        println!("Folder output '{}' tidak ada, membuat folder...", output_folder.display());
        if let Err(e) = fs::create_dir_all(output_folder) {
            eprintln!("Gagal membuat folder output: {}", e);
            return;
        }
    }

    match fs::read_dir(torrent_folder) {
        Ok(entries) => {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if path.extension().map_or(false, |ext| ext == "torrent") {
                        println!("Mulai download torrent: {}", path.display());

                        let output_path = output_folder.to_str().unwrap();

                        // Jalankan aria2c dengan argumen
                        let status = Command::new("aria2c")
                            .arg("-d")
                            .arg(output_path)
                            .arg(path.to_str().unwrap())
                            .status();

                        match status {
                            Ok(s) if s.success() => {
                                println!("Selesai download {}", path.display());
                            }
                            Ok(s) => {
                                eprintln!("aria2c gagal dengan code {}", s);
                            }
                            Err(e) => {
                                eprintln!("Gagal menjalankan aria2c: {}", e);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("Gagal baca folder torrent: {}", e);
        }
    }

    println!("Selesai scan dan proses folder torrent.");
}
