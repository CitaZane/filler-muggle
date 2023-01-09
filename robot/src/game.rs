mod anfield;
mod piece;
mod movement;
mod parser;

pub use parser::*;
use anfield::*;
use piece::*;
use movement::*;

pub struct Game{
    pub anfield:Anfield,
    pub piece:Piece,
    pub player: usize,
    moves :Vec<Move>,
}


impl Game{
    pub fn new()->Self{
        Self{
            anfield:Anfield::new(0,0),
            piece:Piece::new(0,0),
            player:0,
            moves:vec![],
        }
    }
    pub fn place_piece(&mut self){
        self.find_all_valid_spaces();
        if self.moves.len() == 0 {
            println!("0 0");
            return
        }
        // self.calc_move_value()
        // Sort by value
        self.moves.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        println!("{} {}", self.moves[0].col, self.moves[0].row);

    }
    fn find_all_valid_spaces(&mut self){
        self.moves = vec![];
        for row in 0..=self.anfield.height()-self.piece.height(){
            for col in 0..=self.anfield.width()-self.piece.width(){
                let valid = self.piece_placement_valid(row, col);
                if valid{
                    self.moves.push(Move::new(row, col));
                }
            }
        }
    }
    fn piece_placement_valid(&self, row:usize, col:usize) -> bool{
        let mut overlap = 0;
        for piece_row in 0..self.piece.height(){
            for piece_col in 0..self.piece.width(){
                if self.piece.0[piece_row][piece_col] == Tile::Empty{continue}
                let cell = self.anfield.0[piece_row+row][piece_col+col].to_owned();
                match cell {
                    Cell::Empty => {},
                    Cell::Player1(_n) => {
                        if self.player != 1{return false}
                        overlap +=1;
                    },
                    Cell::Player2(_n) => {
                        if self.player != 2{return false}
                        overlap +=1;
                    },
                }
                if overlap >=2 {
                    return false
                }

            }
        }
        if overlap == 1 {
            return true
        }
        false
    }
}