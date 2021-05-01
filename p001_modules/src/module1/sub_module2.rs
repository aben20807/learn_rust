pub struct Sub2 {
    y: i32,
}

impl Sub2 {
    pub fn new(i: i32) -> Sub2 {
        Sub2 { y: i }
    }
    pub fn get_y(&self) -> i32 {
        self.y
    }
}
