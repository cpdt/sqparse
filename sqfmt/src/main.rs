use clap::Parser;
use std::time::Instant;
use sqparse::token::TokenType;

/// If no arguments are specified, it formats the code from standard input and writes the result
/// to standard output.
///
/// If files are given, it reformats the files. If -i is specified together with files, the
/// files are edited in-place. Otherwise, the result is written to the standard output.
#[derive(Parser, Debug)]
#[clap(author, version)]
struct Args {
    /// Inplace edit files, if specified.
    #[clap(short)]
    inplace_edit: bool,

    /// Path to a coding style configuration file, absolute or relative to the current working
    /// directory.
    #[clap(long)]
    style: Option<String>,

    files: Vec<String>,
}

fn test_file(path: &str) {
    let file_text = std::fs::read_to_string(path).unwrap();
    println!("Processing {}... ({} KiB)", path, file_text.as_bytes().len() / 1024);

    let lex_start = Instant::now();
    let maybe_tokens = sqparse::tokenize(&file_text, sqparse::Flavor::SquirrelRespawn);
    let lex_elapsed = lex_start.elapsed();
    println!("  tokenize: {}s", lex_elapsed.as_secs_f64());

    let tokens = match maybe_tokens {
        Ok(tokens) => tokens,
        Err(why) => {
            eprintln!("{}", why.display(&file_text));
            std::process::exit(1);
        }
    };

    /*for token in &tokens {
        match token.ty {
            TokenType::Empty => print!("<empty> "),
            TokenType::Terminal(terminal) => print!("{} ", terminal.as_str()),
            TokenType::Literal(lit) => print!("{:?} ", lit),
            TokenType::Identifier(id) => print!("[{}] ", id),
        }
    }
    println!();*/

    let parse_start = Instant::now();
    let maybe_ast = sqparse::parse(&tokens);
    let parse_elapsed = parse_start.elapsed();
    println!("  parse: {}s", parse_elapsed.as_secs_f64());

    match maybe_ast {
        Ok(ast) => {}, //println!("{:#?}", ast),
        Err(why) => {
            eprintln!("{}", why.display(&tokens, &file_text));
            std::process::exit(1);
        }
    }
}

fn main()  {
    let args = Args::parse();

    if args.files.is_empty() {
        for file in std::fs::read_dir("test_files_r2").unwrap() {
            let path = file.unwrap().path();
            test_file(path.to_str().unwrap());
        }
    } else {
        for file in &args.files {
            test_file(file);
        }
    }
}
