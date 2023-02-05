use sqparse::{parse, tokenize, Flavor};
use std::path::{Path, PathBuf};
use std::time::Instant;

fn main() {
    let mut args = std::env::args();
    let exe = args.next().unwrap();

    let base_path = match args.next() {
        Some(arg) => PathBuf::from(arg),
        None => {
            eprintln!("Usage: {exe} [path]");
            eprintln!();
            eprintln!("Provide a path to a file to parse that file, or a path to a directory to");
            eprintln!("recursively parse all .nut and .gnut files in the directory");
            std::process::exit(1);
        }
    };

    let mut total_size_bytes = 0;
    let mut total_lex_secs = 0.;
    let mut total_parse_secs = 0.;

    visit(&base_path, &mut |path| {
        let extension = path.extension().and_then(|val| val.to_str());
        if !matches!(extension, Some("nut") | Some("gnut")) {
            return;
        }

        println!("{}", path.display());

        let file_text = match std::fs::read_to_string(path) {
            Ok(text) => text,
            Err(err) => {
                println!("  could not read: {err}");
                return;
            }
        };

        let lex_start = Instant::now();
        let tokens = match tokenize(&file_text, Flavor::SquirrelRespawn) {
            Ok(tokens) => tokens,
            Err(err) => {
                eprintln!("{}", err.display(&file_text, path.to_str()));
                std::process::exit(1);
            }
        };
        let lex_secs = lex_start.elapsed().as_secs_f64();
        println!("  tokenize: {lex_secs}s");

        let parse_start = Instant::now();
        if let Err(err) = parse(&tokens) {
            eprintln!("{}", err.display(&file_text, &tokens, path.to_str()));
            std::process::exit(1);
        }
        let parse_secs = parse_start.elapsed().as_secs_f64();
        println!("  parse: {parse_secs}s");

        total_size_bytes += file_text.bytes().len();
        total_lex_secs += lex_secs;
        total_parse_secs += parse_secs;
    });

    let total_mb = total_size_bytes as f64 / 1048576.;
    println!("Finished!");
    println!(
        "Tokenize: {:.4}s, {:.2} MB/s",
        total_lex_secs,
        total_mb / total_lex_secs
    );
    println!(
        "Parse: {:.4}s, {:.2} MB/s",
        total_parse_secs,
        total_mb / total_parse_secs
    );
}

fn visit<F: FnMut(&Path)>(path: &Path, cb: &mut F) {
    if path.is_file() {
        cb(path);
    } else if path.is_dir() {
        for entry in std::fs::read_dir(path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            visit(&path, cb);
        }
    }
}
