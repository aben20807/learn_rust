#[macro_use] extern crate text_io;

fn main() {
    println!("Input: ");
    let s: String;
    scan!("{}\n", s);
    let (minuend, symbol, subtrahend) = statement_analysis(s);
    println!("{}, {}, {}", to_binary(minuend), to_binary(subtrahend), symbol);
}

fn statement_analysis(statement: String) -> (i8, char, i8) {
    for (i, c) in statement.chars().enumerate() {
        if i != 0 && (c == '+' || c == '-') {
            let a = String::from(&statement[..i]).parse::<i8>().unwrap();
            let b = String::from(&statement[i+1..]).parse::<i8>().unwrap();
            return (a, c, b)
        }
    }
    (0, '=', 0)
}

fn to_binary(n: i8) -> String {
    format!("{:08b}", n)
}
