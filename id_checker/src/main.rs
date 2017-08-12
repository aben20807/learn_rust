extern crate regex;
use std::io::{self, BufRead};
use regex::Regex;

fn input() {
    println!("input id: ");
    let mut id = String::new();
    id.clear();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut id)
        .expect("input Error");
    let id = id.trim();
    check(&id)
}

fn check(id: &str) {
    let reg = Regex::new(r"^[a-zA-Z][12]\d{8}$").unwrap();
    if reg.is_match(id) {
        println!("Y");
    }
    else {
        println!("input id wrong format");
    }
}

fn main() {
    println!("Hello, world!");
    input();
}
