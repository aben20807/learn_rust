#[macro_use] extern crate text_io;

fn main() {
    println!("Input: ");
    let s: String;
    scan!("{}\n", s);
    let (a, symbol, b) = statement_analysis(s);
    // println!("{} {} {}", to_binary(a), symbol, to_binary(b));
    plus(to_binary(a), to_binary(b));
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

fn plus(a: String, b: String) {
    println!("\t{}\n+)\t{}", a, b);
    let mut ans = String::from("");
    let mut carry = 0i8;
    let mut bit_s = 0i8;
    for i in (0..8).rev() {
        let bit_a = String::from(&a[i..i+1]).parse::<i8>().unwrap();
        let bit_b = String::from(&b[i..i+1]).parse::<i8>().unwrap();
        let bit_add = bit_a + bit_b + carry;
        if  bit_add > 1 {
            bit_s = if bit_add == 2 {0} else {1};
            // println!("{}", bit_a);
            carry = 1;
        }
        else {
            bit_s = bit_add;
            carry = 0;
        }
        // println!("{}", bit_s);
        ans.push_str(&bit_s.to_string()[..]);
    }
    ans = ans.chars().rev().collect::<String>();
    println!("─────────────────");
    println!("\t{}\ncarry: {}", ans, carry);
}

fn minus(a: String, b: String) {

}
