
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

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn create_new_piece() {
        let piece = Piece::new(2,3);
        assert_eq!(piece.width, 2);
        assert_eq!(piece.height, 3);
    }
    #[test]
    fn add_tile_to_piece() {
        let mut piece = Piece::new(2,3);
        piece.add_tile(1, 2);
        assert_eq!(piece.tiles.len(), 1);
    }
}
