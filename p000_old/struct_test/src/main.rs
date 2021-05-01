#[derive(Debug)]
struct Position {
    x: u32,
    y: u32,
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
    pos: Position,
}

impl Rectangle {
    fn area(&self) -> u32 { // method
        self.length * self.width
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }

    fn new(l: u32, w: u32, x: u32, y: u32) -> Rectangle {
       Rectangle {length: l, width: w, pos: Position {x, y}}
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30, pos: Position {x: 2, y: 4}};
    // println!("rect1 is {:#?}", rect1);
    println!("rect1 is {:?}\nrect1's area = {}", rect1, area(&rect1));
    println!("rect1 is {:?}\nrect1's area = {}", rect1, rect1.area()); // method

    let rect2 = Rectangle { length: 40, width: 10, pos: Position {x: 2, y: 4}};
    println!("rect1 can hold rect2?: {}", rect1.can_hold(&rect2));

    let rect3 = Rectangle::new(25, 35, 5, 2);
    println!("rect3 is {:#?}", rect3);
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.length * rectangle.width
}
