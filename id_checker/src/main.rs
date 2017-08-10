use std::io;

fn input() {
    println!("input id: ");
    let mut id = String::new();
    io::stdin().read_line(&mut id)
        .expect("Error");
    println!("id: {}", id);
}

fn main() {
    println!("Hello, world!");
    input();
}
