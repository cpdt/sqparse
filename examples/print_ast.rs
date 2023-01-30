use sqparse::{parse, tokenize, Flavor};

fn main() {
    let source = include_str!("print_ast_script.nut");
    let tokens = tokenize(source, Flavor::SquirrelRespawn).unwrap();
    let ast = parse(&tokens).unwrap();

    println!("{ast:#?}");
}
