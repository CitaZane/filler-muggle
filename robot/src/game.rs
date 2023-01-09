mod anfield;
mod piece;
mod parser;

pub use parser::*;
use anfield::*;
use piece::*;

pub struct Game{
    pub anfield:Anfield,
    pub piece:Piece,
    pub player: usize,
}

impl Game{
    pub fn new()->Self{
        Self{
            anfield:Anfield::new(0,0),
            piece:Piece::new(0,0),
            player:0,
        }
    }
    #[allow(dead_code)]
    pub fn place_piece(&self){

        for row in 0..=self.anfield.height()-self.piece.height(){
            for col in 0..=self.anfield.width()-self.piece.width(){
                let solved = self.placement_valid(row, col);
                if solved{
                    println!("{} {}", col, row);
                    return
                }
            }
        }
        println!("0 0");

    }
    fn placement_valid(&self, row:usize, col:usize) -> bool{
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
    // #[allow(dead_code)]
    // fn try_to_place_piece(&self, row:usize, col:usize) -> bool{
    //     for (i, _piece_row) in self.piece.0.iter().enumerate(){
    //         for (j, tile) in self.piece.0[i].iter().enumerate(){
    //             if tile.to_owned()  == Tile::Empty{continue}
    //             let found = self.check_valid_placement(row,col, i,j);
    //             if found {return true}
    //         }
    //     }
    //     return false
    // }
    // #[allow(dead_code)]
    // fn check_valid_placement(&self, field_row:usize, field_col:usize, piece_row:usize, piece_col:usize) -> bool{
    //     let mut  valid = true;
    //     // check if out of field
    //     if field_row.checked_sub(piece_row).is_none(){return false}
    //     if field_row + self.piece.height()-1 - piece_row > self.anfield.height(){return  false}
    //     if field_col.checked_sub(piece_col).is_none(){return  false}
    //     if field_col + self.piece.width()-1- piece_col > self.anfield.width(){return  false}

    //     let base_row = field_row-piece_row;
    //     let base_col = field_col - piece_col;

    //     for (i, _check_row) in self.piece.0.iter().enumerate(){
    //         for (j, tile) in self.piece.0[i].iter().enumerate(){
    //             if tile.to_owned() == Tile::Empty{continue}
    //             if i == piece_row && i == piece_col {continue}
    //             if self.anfield.0[base_row+i][base_col+j] != Cell::Empty{
    //                 valid = false;
    //                 break;
    //             }
    //         }
    //     }

    //     if valid{
    //         println!("{} {}", base_col, base_row);
    //         return true
    //     }
    //     return false
    // }
}