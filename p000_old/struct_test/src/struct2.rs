struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    fn set_width(&mut self, width: u32) {
        self.width = width;
    }
}

fn main() {
    let rect1 = Rectangle { width: 30, height: 50 };
    println!("area is {}", rect1.area());

    let mut rect1 = rect1;
    rect1.set_width(40);
    println!("area is {}", rect1.area());
}
