use sqparse::{parse, tokenize, Flavor};

fn main() {
    let source = include_str!("print_parser_error_script.nut");
    let tokens = tokenize(source, Flavor::SquirrelRespawn).unwrap();
    let parse_err = parse(&tokens, Flavor::SquirrelRespawn).unwrap_err();

    println!(
        "{}",
        parse_err.display(source, &tokens, Some("print_parser_error_script.nut"))
    );
}
