mod board;
mod defines;
use board::Board;
use defines::*;

fn main() {
    println!("Hello, world!");
    let board = Board::new();
    println!("{:?}", board);
}
