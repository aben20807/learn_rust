pub struct Sub3 {
    z: i32,
}

impl Sub3 {
    pub fn new(i: i32) -> Sub3 {
        Sub3 { z: i }
    }
    pub fn get_z(&self) -> i32 {
        self.z
    }
}

pub struct Sub4 {
    w: i32,
}

impl Sub4 {
    pub fn new(i: i32) -> Sub4 {
        Sub4 { w: i }
    }
    pub fn get_w(&self) -> i32 {
        self.w
    }
}
