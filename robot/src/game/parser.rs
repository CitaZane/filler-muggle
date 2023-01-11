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
            self.register_field_occupancy(input);
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
                    self.overwrite_cell(1, i);
                },
                '@'=>if self.initial{self.overwrite_cell(1, i)},
                's' =>{
                    if self.current_player != 2{self.opponent_stuck = false};
                    self.overwrite_cell(2, i);
                },
                '$'=> if self.initial{self.overwrite_cell(2, i)},
                _=> {}
            };
        }
        self.current_row +=1;
    }
    fn overwrite_cell(&mut self, player: usize, col:usize){
        let player = if player == 1{Cell::Player1}else{Cell::Player2};
        match self.anfield.as_mut(){
            Some(anfield)=> anfield.0[self.current_row][col] = player,
            _=> {}
        }
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
    fn register_player_1() {
        let mut parser = Parser::new();
        let line = "$$$ exec p1 : [robots/muggle]";
        parser.parse_input(line);
        assert_eq!(parser.current_player, 1);
    }
    #[test]
    fn register_player_2() {
        let mut parser = Parser::new();
        let line = "$$$ exec p2 : [robots/muggle]";
        parser.parse_input(line);
        assert_eq!(parser.current_player, 2);
    }
    #[test]
    fn register_anfield() {
        let mut parser = Parser::new();
        let line = "Anfield 20 15";
        parser.parse_input(line);
        assert_eq!(parser.anfield.is_some(), true);
        assert_eq!(parser.anfield.unwrap().width(), 20);
    }
    #[test]
    fn write_anfield() {
        let mut parser = Parser::new();
        let line = "Anfield 3 3";
        parser.parse_input(line);
        let line = "000 $..";
        parser.parse_input(line);
        let line = "001 ...";
        parser.parse_input(line);
        let line = "002 ...";
        parser.parse_input(line);
        assert_eq!(parser.anfield.is_some(), true);
        assert_eq!(parser.anfield.unwrap().0[0][0], Cell::Player2);
    }
    #[test]
    fn register_piece() {
        let mut parser = Parser::new();
        let line = "Piece 5 1:";
        parser.parse_input(line);
        assert_eq!(parser.piece.is_some(), true);
        assert_eq!(parser.piece.unwrap().width, 5);
    }
    #[test]
    fn write_piece() {
        let mut parser = Parser::new();
        let line = "Piece 5 1:";
        parser.parse_input(line);
        let line = ".OOO.";
        parser.parse_input(line);
        assert_eq!(parser.piece.is_some(), true);
        assert_eq!(parser.piece.unwrap().tiles.len(), 3);
    }
}