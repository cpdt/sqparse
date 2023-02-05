use sqparse::{parse, tokenize, Flavor};

fn main() {
    let source = include_str!("print_ast_script.nut");

    let tokens = match tokenize(source, Flavor::SquirrelRespawn) {
        Ok(tokens) => tokens,
        Err(err) => {
            eprintln!("{}", err.display(source, Some("print_ast_script.nut")));
            return;
        }
    };

    let ast = match parse(&tokens) {
        Ok(ast) => ast,
        Err(err) => {
            eprintln!(
                "{}",
                err.display(source, &tokens, Some("print_ast_script.nut"))
            );
            return;
        }
    };

    println!("{ast:#?}");
}
