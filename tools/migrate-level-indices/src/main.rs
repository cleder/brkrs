use ron::de::from_str;
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize, Serialize, Debug)]
struct LevelDefinition {
    number: u32,
    #[serde(default)]
    matrix: Vec<Vec<u8>>,
}

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: migrate-level-indices [--backup] --from <N> --to <N> <files...>");
    std::process::exit(2);
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        print_usage_and_exit();
    }

    let mut backup = false;
    let mut from_val: Option<u8> = None;
    let mut to_val: Option<u8> = None;

    // Simple arg parsing
    while !args.is_empty() {
        match args[0].as_str() {
            "--backup" => {
                backup = true;
                args.remove(0);
            }
            "--from" => {
                args.remove(0);
                if args.is_empty() {
                    print_usage_and_exit();
                }
                from_val = Some(args.remove(0).parse::<u8>().unwrap());
            }
            "--to" => {
                args.remove(0);
                if args.is_empty() {
                    print_usage_and_exit();
                }
                to_val = Some(args.remove(0).parse::<u8>().unwrap());
            }
            _ => break,
        }
    }

    let from = match from_val {
        Some(v) => v,
        None => {
            eprintln!("--from is required");
            print_usage_and_exit();
        }
    };

    let to = match to_val {
        Some(v) => v,
        None => {
            eprintln!("--to is required");
            print_usage_and_exit();
        }
    };

    if args.is_empty() {
        eprintln!("No files provided");
        print_usage_and_exit();
    }

    let pretty = PrettyConfig::new().separate_tuple_members(true);

    for path in args.iter() {
        let p = Path::new(path);
        if !p.exists() {
            eprintln!("Skipping {}: file not found", path);
            continue;
        }

        let content = match fs::read_to_string(p) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed reading {}: {}", path, e);
                continue;
            }
        };

        // Try parsing as LevelDefinition RON
        match from_str::<LevelDefinition>(&content) {
            Ok(mut def) => {
                let mut changed = false;
                for row in def.matrix.iter_mut() {
                    for v in row.iter_mut() {
                        if *v == from {
                            *v = to;
                            changed = true;
                        }
                    }
                }
                if changed {
                    if backup {
                        let backup_path = format!("{}.bak", path);
                        if let Err(e) = fs::copy(p, backup_path) {
                            eprintln!("Warning: failed to create backup for {}: {}", path, e);
                        }
                    }
                    match to_string_pretty(&def, pretty.clone()) {
                        Ok(out) => match fs::write(p, out) {
                            Ok(_) => println!("Migrated {}: replaced {} -> {}", path, from, to),
                            Err(e) => eprintln!("Failed writing {}: {}", path, e),
                        },
                        Err(e) => eprintln!("Failed to serialize {}: {}", path, e),
                    }
                } else {
                    println!("No changes needed for {}", path);
                }
            }
            Err(_) => {
                // Not a LevelDefinition RON we can parse safely — skip
                println!("Skipping {}: not a LevelDefinition (could not parse)", path);
            }
        }
    }
}
