
#[derive( Debug, PartialEq, Clone)]
pub struct Anfield (pub Vec<Vec<Cell>>);

impl Anfield{
    pub fn new(width:usize, height :usize)->Self{
        Self(vec![vec![Cell::Empty; width]; height])
    }
    pub fn height(&self) -> usize{
        self.0.len()
    }
    pub fn width(&self) -> usize{
        self.0[0].len()
    }

}

#[derive( Debug, Clone, PartialEq)]
    pub enum Cell{
        Empty,
        Player1,
        Player2
    }
