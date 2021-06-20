use crate::arinc424::parser::Parser;

mod arinc424;

fn main() {
    println!("reading cifp...");
    let data = std::fs::read_to_string("FAACIFP18").unwrap();

    println!("parsing cifp...");
    let mut parser = Parser::new(data);
    parser.parse().unwrap();

    println!("done!");
}
