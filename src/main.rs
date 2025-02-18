#![allow(unused)] // suppress warnings for unused code (there is plenty when you start)

use board::Board;
use rand::{rng, Rng};

// declare other modules that are in other files and must be compiled
mod board;
fn main() {
    let mut moves_played = 0;
    let nb = 11;
    let mut initial_board = Board::new_empty();
    let mut board = initial_board.generate_random().unwrap().generate_random().unwrap();
    loop{
        println!("initial_board: \n{}", board);
        println!("\n!!Thinking!!\n");
        let bsmove = board.best_move(nb);
        println!("best move is : {}",bsmove );
        println!("playing best move : ");
        println!("\n!!working!!\n");
        board = board.play(bsmove).unwrap();
        moves_played+=1;
        println!("moves played : {}",moves_played );
    }
}
