use message_event_lint::analyze_file;
use std::{env, fs, path::PathBuf};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: message_event_lint <file.rs> [<file2.rs> ...]");
        std::process::exit(2);
    }
    let mut total = 0;
    for f in &args[1..] {
        let p = PathBuf::from(f);
        if let Ok(src) = fs::read_to_string(&p) {
            let findings = analyze_file(&p, &src);
            for f in findings {
                println!(
                    "{}: function `{}` likely misuses MessageWriter with an immediate side-effect",
                    f.file, f.fn_name
                );
                total += 1;
            }
        }
    }
    if total > 0 {
        std::process::exit(2);
    }
}
