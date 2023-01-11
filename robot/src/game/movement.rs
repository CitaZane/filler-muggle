#[derive(Clone)]
pub struct Move{
    pub row:usize,
    pub col:usize,
    pub value:f32,
    pub distance: f32,
    pub edge:i32,
}

impl Move {
    pub fn new(row:usize, col:usize) -> Self{
        Self{
            row, col, value:0.,distance:f32::MAX,edge:0,
        }
    }
    pub fn calc_distance(&mut self, field_row:usize, field_col:usize){
        let new_dist = dist_between_points(self.row, self.col, field_row, field_col);
        if new_dist< self.distance{
            self.distance = new_dist;
        }
    }
    pub fn calc_value(&mut self) -> f32{
        self.value = (self.distance*3.) / self.edge as f32;
        self.value
    }
    pub fn register_edge(&mut self, edge: i32){
        self.edge = edge
    }
}

fn dist_between_points(x1:usize, y1 :usize, x2: usize, y2:usize) -> f32{
    let dx = f32::abs(x2 as f32 - x1 as f32);
    let dy = f32::abs(y2 as f32 - y1 as f32);

    (dx*dx + dy*dy).sqrt()
}