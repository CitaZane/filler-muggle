
pub struct Move{
    pub row:usize,
    pub col:usize,
    pub value:i32,
}

impl Move {
    pub fn new(row:usize, col:usize) -> Self{
        Self{
            row, col, value:0,
        }
    }
}