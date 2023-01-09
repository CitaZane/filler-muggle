mod anfield;
mod piece;
mod movement;
mod parser;

use std::cmp;

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
        self.calc_move_values();
        // Sort by value
        self.moves.sort_by(|a, b| a.value.partial_cmp(&b.value).unwrap());
        let res = self.moves.len() -1;
        println!("{} {}", self.moves[res].col, self.moves[res].row);

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
    fn calc_move_values(&mut self){
        for i in 0..self.moves.len(){
            let edge = self.distance_to_edge(i);
            let oponent_distance = self.distance_to_opponent(i);
            self.moves[i].value = edge + oponent_distance;
        }
    }
    fn distance_to_opponent(&self, move_index: usize)-> i32{
        let mut  distance = self.anfield.height() as i32 * 2;
        let m = &self.moves[move_index];
        // returns distance to closest edge
        for row in 0..self.anfield.height(){
            for col in 0..self.anfield.width(){
                match self.anfield.0[row][col]{
                    Cell::Player1(_n)=>{
                        if self.player == 1{ continue}
                        let new_dist = calc_distance(row, col, m.row, m.col);
                        if new_dist < distance{
                            distance = new_dist
                        }
                    },
                    Cell::Player2(_n)=>{
                        if self.player == 2{continue}
                        let new_dist = calc_distance(row, col, m.row, m.col);
                        if new_dist < distance{
                            distance = new_dist
                        }
                    },
                    _=>{}
                }
            }
        }
        distance as i32

    }
    fn distance_to_edge(&self, move_index: usize)-> i32{
        // returns distance to closest opponent
        
        let m = &self.moves[move_index];
        let left_edge = m.col;
        let right_edge = self.anfield.width() - m.col + self.piece.width();
        cmp::min(left_edge, right_edge) as i32
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

fn calc_distance(x1:usize, y1 :usize, x2: usize, y2:usize) -> i32{
    let dx = i32::abs(x2 as i32 - x1 as i32);
    let dy = i32::abs(y2 as i32 - y1 as i32);

    let min = cmp::min(dx, dy);
    let max = cmp::max(dx, dy);

    let diognal_steps = min;
    let straight_steps = max-min;
    (1.4 * diognal_steps as f32 ) as i32 + straight_steps
}