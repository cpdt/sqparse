use sqparse::{parse, tokenize, Flavor};

fn main() {
    let source = include_str!("print_parser_error_script.nut");
    let tokens = tokenize(source, Flavor::SquirrelRespawn).unwrap();
    let parse_err = parse(&tokens).unwrap_err();

    println!("{}", parse_err.display(source, &tokens));
}
