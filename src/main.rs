mod square;
mod piece;
mod moves;
mod parser;

use parser::parse;

fn main() {
    let pgn = std::fs::read_to_string("test.pgn").expect("Something went wrong reading the file");

    let res = parse(&pgn).unwrap();
    println!("Headers: \n{:?}\n\nGame:\n{:}", res.tags, res.root.graph());
}