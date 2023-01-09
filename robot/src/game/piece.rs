
#[derive( Debug, PartialEq, Clone)]
pub struct Piece (pub Vec<Vec<Tile>>);

impl Piece{
    pub fn new(width:usize, height :usize)->Self{
        Self(vec![vec![Tile::Empty; width]; height])
    }
    #[allow(dead_code)]
    pub fn print(&self){
        for row in self.0.iter(){
            println!("{:?}", row);
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

#[derive( Debug, Clone, PartialEq, Copy)]
pub enum Tile{
    Empty,
    Taken,
}