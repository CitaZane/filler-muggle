mod game;

use game::{Game, Parser};
use std::io;

fn main() {
    let mut game = Game::new();
    let mut parser = Parser::new();

    for line in io::stdin().lines(){
        let line = line.unwrap();
        let new_piece = parser.parse_input(line.as_str());
        if new_piece{
            game.anfield = parser.anfield.as_ref().unwrap().clone();
            game.piece = parser.piece.as_ref().unwrap().clone();
            game.player = parser.current_player;

            game.place_piece();
        }  
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_move_test() {
        let mut parser = Parser::new();
        let input = "$$$ exec p1 : [robots/muggle]
Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .........@..........
003 ....................
004 ....................
005 ....................
006 ....................
007 ....................
008 ....................
009 ....................
010 ....................
011 ....................
012 .........$..........
013 ....................
014 ....................
Piece 4 1:
.OO.";
let mut game = Game::new();
        let iter: Vec<&str> = input.split("\n").collect();
        for line in iter{
            let new_piece = parser.parse_input(line);
            if new_piece{
                game.anfield = parser.anfield.as_ref().unwrap().clone();
                game.piece = parser.piece.as_ref().unwrap().clone();
                game.player = parser.current_player;
    
                game.place_piece();
            } 
        }

        assert_eq!(2, 1);
    }
}