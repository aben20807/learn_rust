pub struct Sub1 {
    x: i32,
}

impl Sub1 {
    pub fn new(i: i32) -> Sub1 {
        Sub1 { x: i }
    }
    pub fn get_x(&self) -> i32 {
        self.x
    }
}
