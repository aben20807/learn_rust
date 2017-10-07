#[macro_use] extern crate text_io;

fn main() {
    println!("Input: ");
    let s: String;
    scan!("{}\n", s);
    let (minuend, symbol, subtrahend) = statement_analysis(s);
    println!("{}, {}, {}", minuend, subtrahend, symbol);
}

fn statement_analysis(statement: String) -> (i32, char, i32) {
    for (i, c) in statement.chars().enumerate() {
        if i != 0 && (c == '+' || c == '-') {
            let a = String::from(&statement[..i]).parse::<i32>().unwrap();
            let b = String::from(&statement[i+1..]).parse::<i32>().unwrap();
            // println!("symbol {}, {}", a, b);
            return (a, c, b)
        }
    }
    (0, '=', 0)
}
