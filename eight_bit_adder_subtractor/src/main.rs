#[macro_use] extern crate text_io;

fn main() {
    println!("Input: ");
    let s: String;
    scan!("{}\n", s);
    let f = binary_analysis(s);
}

fn binary_analysis(formula: String) {// -> (String, char, String) {
    for (i, c) in formula.chars().enumerate() {
        if c == '+' || c == '-' {
            let a = String::from(&formula[..i]).parse::<i32>().unwrap();
            let b = String::from(&formula[i+1..]).parse::<i32>().unwrap();
            println!("symbol {}, {}", a, b);
        }
    }
}
