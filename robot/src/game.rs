mod anfield;
mod piece;
mod movement;
mod parser;


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
    pub opponent_stuck:bool,
}


impl<'a> Game<'a>{
    pub fn new(piece:&'a Piece, anfield:&'a Anfield, player: usize)->Self{
        Self{
            piece,
            anfield,
            player,
            moves:vec![],
            best_move:Move::new(0,0),
            opponent_stuck:false,
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
                if !valid{continue}
                self.moves.push(Move::new(row, col));
                if self.opponent_stuck{return}
            }
        }
    }
    fn calc_move_stats(&mut self){
        for i in 0..self.moves.len(){

            let edge = self.distance_to_edge(i);
            self.moves[i].register_edge(edge); 
            self.distance_to_opponent(i);
            self.moves[i].calc_value();
            if i == 0 {
                self.best_move = self.moves[0].clone()
            }else{
                if self.moves[i].value < self.best_move.value{
                    self.best_move = self.moves[i].clone()
                }
            }
            if self.moves[i].value == 0. {return}
        }
    }
    fn distance_to_opponent(&mut self, move_index: usize){
        for row in 0..self.anfield.height(){
            for col in 0..self.anfield.width(){
                if self.opponents_field(row, col){
                    self.moves[move_index].calc_distance(row, col);
                }
            }
        }
    }
    fn distance_to_edge(&self, move_index: usize)-> i32{
        let m = &self.moves[move_index];
        let right_edge = self.anfield.width() - m.col + self.piece.width;
        let down_edge = self.anfield.height() - m.row + self.piece.height;
        get_biggest_out_of_4(right_edge, down_edge, m.col, m.row)
    }
    fn piece_placement_valid(&self, row:usize, col:usize) -> bool{
        let mut overlap = 0;
        for tile in self.piece.tiles.iter(){
            let row = tile.0+row;
            let col = tile.1+col;
            if self.opponents_field(row, col){return false}
            if self.players_field(row, col){overlap +=1 }
            if overlap >=2 {
                return false
            }
        }
        overlap == 1
    }
    fn players_field(&self, row:usize, col:usize) -> bool{
        match self.anfield.0[row][col]{
            Cell::Player1 => self.player == 1,
            Cell::Player2 => self.player == 2,
            _ => false
        }
    }
    fn opponents_field(&self, row:usize, col:usize) -> bool{
        match self.anfield.0[row][col]{
            Cell::Player1 => self.player == 2,
            Cell::Player2 => self.player == 1,
            _ => false
        }
    }
}

fn get_biggest_out_of_4(one:usize, two:usize, three:usize, four:usize)->i32{
    let mut data = vec![one as i32, two as i32, three as i32, four as i32];
    data.sort();
    data[3]
}