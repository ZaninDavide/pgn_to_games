mod square;
mod piece;
mod moves;
mod parser;

use parser::parse;

fn main() {
    let pgn = std::fs::read_to_string("test.pgn").expect("Something went wrong reading the file");

    let res = parse(&pgn).unwrap();
    println!("Headers \n\n{:?}\n\n", res.tags);
    println!("Game\n\n{:}\n\n", res.root.graph());
    println!("Lines\n\n{:?}", res.root.lines(true));
    println!("JSON\n\n{}", res.root.json());
}