
#[derive( Debug, PartialEq, Clone)]
pub struct Piece {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<(usize, usize)>,
}

impl Piece{
    pub fn new(width:usize, height :usize)->Self{
        Self{width, height, tiles:vec![]}
    }
    pub fn add_tile(&mut self, row:usize, col:usize){
        self.tiles.push((row, col));
    }
}