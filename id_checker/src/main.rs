use std::io::{self, BufRead};

fn input() {
    println!("input id: ");
    let mut id = String::new();
    id.clear();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut id)
        .expect("input Error");
    let id = id.trim();
    // if let Some('\n')=id.chars().next_back() {
    //     id.pop();
    // }
    // if let Some('\r')=id.chars().next_back() {
    //     id.pop();
    // }
    check(&id)
}

fn check(id: &str) {
    // if id.eq("DD") {
    if id == "DD" {
        println!("Y");
    }
    else {
        println!("{}, {}", id.len(), "DD".to_owned().len());
        println!("{}, {}", id, "DD");
        println!("N");
    }
}

fn main() {
    println!("Hello, world!");
    input();
}
