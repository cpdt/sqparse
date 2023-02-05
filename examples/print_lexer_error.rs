use sqparse::{tokenize, Flavor};

fn main() {
    let source = include_str!("print_lexer_error_script.nut");
    let tokens_err = tokenize(source, Flavor::SquirrelRespawn).unwrap_err();

    println!(
        "{}",
        tokens_err.display(source, Some("print_lexer_error_script.nut"))
    );
}
