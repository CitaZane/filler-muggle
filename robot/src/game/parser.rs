use crate::game::anfield::*;
use crate::game::piece::*;

#[derive( Debug, PartialEq)]
pub enum Mode{
    Anfield,
    Piece,
    None
}
pub struct Parser{
    pub mode:Mode,
    pub anfield: Option<Anfield>,
    initial:bool,
    pub piece: Option<Piece>,
    current_row:usize,
    pub current_player:usize,
    pub opponent_stuck:bool,
}
impl Parser{
    pub fn new()-> Self{
        Self{
            mode: Mode::None,
            anfield:None,
            piece: None,
            current_player:0,
            current_row: 0,
            opponent_stuck:false,
            initial:true,
        }
    }
    pub fn parse_input(&mut self, input:&str) -> bool{
        if input.starts_with("$$$"){
            self.register_current_player(input);
        }else if input.contains("Anfield"){
            self.start_anfield_mode(input);
        }else if input.contains("Piece"){
            self.start_piece_mode(input)
        }else if self.mode == Mode::Anfield{
            if self.initial{
                self.register_field_occupancy(input);
            }else{
                self.update_anfield(input);
            }
        }else if self.mode == Mode::Piece{
            self.register_piece_occupancy(input);
            if self.current_row == self.piece.as_ref().unwrap().height{
                self.mode = Mode::None;
                return true
            }
        }
        return false
    }
    fn register_current_player(&mut self, input:&str){
        if input.contains("p1")&& input.contains("muggle"){
            self.current_player = 1;
        }else if input.contains("p2")&& input.contains("muggle"){
            self.current_player = 2;
        }
    }
    fn start_anfield_mode(&mut self, input:&str){
        self.mode = Mode::Anfield;
        if self.anfield.is_none(){
            let (width, height) = parse_two_numbers(input);
            self.anfield = Some(Anfield::new(width, height));
        }else{
            self.initial = false;
        }
        self.current_row = 0;
        self.opponent_stuck = true;
    }
    fn start_piece_mode(&mut self, input:&str){
        self.mode = Mode::Piece;
        let (width, height) = parse_two_numbers(input);
        self.piece = Some(Piece::new(width, height));
        self.current_row = 0;
    }
    fn register_field_occupancy(&mut self, input : &str){
        if input.starts_with("    "){return}
        let mut input = input.split_whitespace().skip(1);
        for (i,cell) in input.next().unwrap().chars().enumerate(){
            match cell{
                'a' =>{
                    if self.current_player != 1{self.opponent_stuck = false};
                    match self.anfield.as_mut(){
                        Some(anfield)=> anfield.0[self.current_row][i] = Cell::Player1(0),
                        _=> {}
                    }
                    // Cell::Player1(1)
                },
                '@'=>{
                    match self.anfield.as_mut(){
                        Some(anfield)=> anfield.0[self.current_row][i] = Cell::Player1(0),
                        _=> {}
                    }
                },
                's' =>{
                    if self.current_player != 2{self.opponent_stuck = false};
                    match self.anfield.as_mut(){
                        Some(anfield)=> anfield.0[self.current_row][i] = Cell::Player2(0),
                        _=> {}
                    }
                },
                '$'=>{
                    match self.anfield.as_mut(){
                        Some(anfield)=> anfield.0[self.current_row][i] = Cell::Player2(0),
                        _=> {}
                    }
                },
                _=> {}
            };
        }
        self.current_row +=1;
    }
    fn update_anfield(&mut self, input : &str){
        if input.starts_with("    "){return}
        let mut input = input.split_whitespace().skip(1);
        for (i,cell) in input.next().unwrap().chars().enumerate(){
            match cell{
                'a' =>{
                    if self.current_player != 1{self.opponent_stuck = false};
                    match self.anfield.as_mut(){
                        Some(anfield)=> anfield.0[self.current_row][i] = Cell::Player1(0),
                        _=> {}
                    }
                },
                's' =>{
                    if self.current_player != 2{self.opponent_stuck = false};
                    match self.anfield.as_mut(){
                        Some(anfield)=> anfield.0[self.current_row][i] = Cell::Player2(0),
                        _=> {}
                    }
                },
                _ => {},
            };
        }
        self.current_row +=1;
    }

    fn register_piece_occupancy(&mut self, input : &str){
        for (i,cell) in input.chars().enumerate(){
            if cell != '.'{
                self.piece.as_mut().unwrap().add_tile(self.current_row, i);
            }
        }
        self.current_row +=1;
    }
}

fn parse_two_numbers(input:&str)-> (usize, usize){
    let input = input.replace(":", "");
    let mut data = input.split_whitespace().skip(1).map(|s| s.parse::<usize>());
    let num1 = data.next().unwrap().unwrap();
    let num2 = data.next().unwrap().unwrap();
    (num1, num2)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_test() {
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
        let iter: Vec<&str> = input.split("\n").collect();
        for line in iter{
            println!("{}", line);
            parser.parse_input(line);
        }
        
        assert_eq!(parser.current_player, 1);
        assert_eq!(parser.anfield.is_some(), true);
        assert_eq!(parser.piece.is_some(), true);
        assert_eq!(parser.anfield.as_ref().unwrap().0[2][9], Cell::Player1(0));
        assert_eq!(parser.anfield.as_ref().unwrap().0[12][9], Cell::Player2(0));
        assert_eq!(parser.anfield.as_ref().unwrap().0[0][0], Cell::Empty);
        assert_eq!(parser.piece.as_ref().unwrap().tiles.len(), 2);
    }

    #[test]
    fn second_input_test() {
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
let iter: Vec<&str> = input.split("\n").collect();
        for line in iter{
            parser.parse_input(line);
        }
        let second_input = "Anfield 20 15:
    01234567890123456789
000 ....................
001 ....................
002 .......aa@..........
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
013 .........s..........
014 ....................
Piece 5 1:
.OOO.";
        let iter: Vec<&str> = second_input.split("\n").collect();
        for line in iter{
            parser.parse_input(line);
        }
        
        assert_eq!(parser.current_player, 1);
        assert_eq!(parser.anfield.is_some(), true);
        assert_eq!(parser.piece.is_some(), true);
        assert_eq!(parser.anfield.as_ref().unwrap().0[2][9], Cell::Player1(0));
        assert_eq!(parser.anfield.as_ref().unwrap().0[2][8], Cell::Player1(1));
        assert_eq!(parser.anfield.as_ref().unwrap().0[12][9], Cell::Player2(0));
        assert_eq!(parser.anfield.as_ref().unwrap().0[13][9], Cell::Player2(1));
        assert_eq!(parser.anfield.as_ref().unwrap().0[0][0], Cell::Empty);
    }
}