mod anfield;
mod piece;
mod movement;
mod parser;

use std::cmp;

pub use parser::*;
use anfield::*;
use piece::*;
use movement::*;

pub struct Game<'a>{
    pub anfield:&'a Anfield,
    pub piece:&'a Piece,
    pub player: usize,
    moves :Vec<Move>,
    best_move:Move,
}


impl<'a> Game<'a>{
    pub fn new(piece:&'a Piece, anfield:&'a Anfield, player: usize)->Self{
        Self{
            piece,
            anfield,
            player,
            moves:vec![],
            best_move:Move::new(0,0),
        }
    }
    pub fn place_piece(&mut self){
        self.find_all_valid_spaces();
        
        if self.moves.len() == 0 {
            println!("0 0");
            return
        }
        self.calc_move_stats();
        println!("{} {}", self.best_move.col, self.best_move.row);
    }
    fn find_all_valid_spaces(&mut self){
        self.moves = vec![];
        for row in 0..=self.anfield.height()-self.piece.height{
            for col in 0..=self.anfield.width()-self.piece.width{
                let valid = self.piece_placement_valid(row, col);
                if valid{
                    self.moves.push(Move::new(row, col));
                }
            }
        }
    }
    fn calc_move_stats(&mut self){
        for i in 0..self.moves.len(){
            let edge = self.distance_to_edge(i);
            self.moves[i].register_edge(edge); 
            self.distance_to_opponent(i);
            let value = self.moves[i].calc_value();
            if i == 0 {
                self.best_move = self.moves[0].clone()
            }else{
                if value < self.best_move.value{
                    self.best_move = self.moves[i].clone()
                }
            }
        }
    }
    fn distance_to_opponent(&mut self, move_index: usize){
        for row in 0..self.anfield.height(){
            for col in 0..self.anfield.width(){
                match self.anfield.0[row][col]{
                    Cell::Player1(_n)=>{
                        if self.player == 1{ continue}
                        self.moves[move_index].calc_distance(row, col);
                    },
                    Cell::Player2(_n)=>{
                        if self.player == 2{continue}
                        self.moves[move_index].calc_distance(row, col);
                    },
                    _=>{}
                }
                // if self.moves[move_index].distance < 1. {return}
            }
        }
    }
    fn distance_to_edge(&self, move_index: usize)-> i32{
        // returns distance to far edge
        let m = &self.moves[move_index];
        let left_edge = m.col;
        let up_edge = m.row;
        let right_edge = self.anfield.width() - m.col + self.piece.width;
        let down_edge = self.anfield.height() - m.row + self.piece.height;
        let first_winner = cmp::max(left_edge, right_edge) as i32;
        let second_winner=cmp::max(down_edge, up_edge) as i32;
        cmp::max(first_winner, second_winner)
    }
    fn piece_placement_valid(&self, row:usize, col:usize) -> bool{
        let mut overlap = 0;
        for tile in self.piece.tiles.iter(){
            let cell = &self.anfield.0[tile.0+row][tile.1+col];
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
        if overlap == 1 {
                return true
            }
        false
    }
}

