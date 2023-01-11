mod game;

use game::{Game, Parser};
use std::io;

fn main() {
    let mut parser = Parser::new();
    
    for line in io::stdin().lines(){
        let line = line.unwrap();
        let new_piece = parser.parse_input(line.as_str());
        if new_piece{
            let piece = parser.piece.as_ref().unwrap();
            let anfield = parser.anfield.as_ref().unwrap();
            let player = parser.current_player;
            
            let mut game = Game::new(piece, anfield, player);
            game.opponent_stuck = parser.opponent_stuck;

            game.place_piece();
        }  
    }
}
