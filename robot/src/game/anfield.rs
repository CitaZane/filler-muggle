
#[derive( Debug, PartialEq, Clone)]
pub struct Anfield (pub Vec<Vec<Cell>>);

impl Anfield{
    pub fn new(width:usize, height :usize)->Self{
        Self(vec![vec![Cell::Empty; width]; height])
    }
    #[allow(dead_code)]
    pub fn print(&self){
        for (i,_row) in self.0.iter().enumerate(){
            for cell in self.0[i].iter(){
                if cell.to_owned() == Cell::Empty {
                    print!(".");
                }else if cell.to_owned() == Cell::Player1(0) {
                    print!("x");
                }else{
                    print!("o");
                }
            }
            println!();
        }
    }
    #[allow(dead_code)]
    pub fn height(&self) -> usize{
        self.0.len()
    }
    #[allow(dead_code)]
    pub fn width(&self) -> usize{
        self.0[0].len()
    }

}

#[derive( Debug, Clone, PartialEq)]
    pub enum Cell{
        Empty,
        Player1(i32),
        Player2(i32)
    }
    // Player pieces hold 0 if regular piece
    // hold 1 if the last placed piece
