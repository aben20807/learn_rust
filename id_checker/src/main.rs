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
        let id = id.to_uppercase();
        let id0 = &id[..1];
        // String IdStr="";
        static COUNTRY_LIST: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
        let country_list_code: [i32; 26]=[10,11,12,13,14,15,16,17,34,18,19
            ,20,21,22,35,23,24,25,26,27,28,29,32,30,31,33];
        let country_index = COUNTRY_LIST.find(id0).unwrap();
        let country_code = country_list_code[country_index];
        let mut sum = 0;
        sum += (country_code / 10) + (country_code % 10 * 9);
        println!("{}", sum);
        for i in 1..(id.len()-1) {
        }
    }
    else {
        println!("input id wrong format!\n");
        input();
    }
}

fn main() {
    input();
}
