#[macro_use] extern crate text_io;

fn main() {
    println!("Input: ");
    let s: String;
    scan!("{}\n", s);
    let (a, symbol, b) = statement_analysis(s);
    if symbol == '+' {
        plus(&dec_to_bin(a), &dec_to_bin(b), true);
    }
    else if symbol == '-' {
        minus(&dec_to_bin(a), &dec_to_bin(b), true);
    }
    else {
        println!("Wrong symbol!");
    }
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

fn dec_to_bin(n: i8) -> String {
    let mut dec:i32 = n as i32; // Because -128 * (-1) will overflow
    let mut ans = String::from("");
    let mut count = 0;
    let mut is_neg = false;
    if dec < 0 {
        dec = dec * (-1);
        is_neg = true;
    }
    while dec > 0 {
        ans.push_str(&(dec % 2).to_string()[..]);
        dec = dec / 2;
        count = count + 1;
    }
    while 8 - count > 0 {
        ans.push('0');
        count = count + 1;
    }
    ans = ans.chars().rev().collect::<String>();
    if is_neg {
        two_s_complement(&ans, false)
    }
    else {
        ans
    }
}

fn one_s_complement(input: &String) -> String {
    let mut ans = String::from("");
    for i in 0..8 {
        let old_bit = String::from(&input[i..i+1]).parse::<i8>().unwrap();
        let bit = (old_bit + 1) % 2;
        ans.push_str(&bit.to_string()[..]);
    }
    ans
}

fn two_s_complement(input: &String, can_display_process: bool) -> String {
    let ans = plus(&one_s_complement(input), &String::from("00000001"), false);
    if can_display_process {
        println!("\n2's)\t{}", input);
        println!("─────────────────");
        println!("\t{}\n", ans);
    }
    ans
}

fn plus(a: &String, b: &String, can_display_process: bool) -> String {
    if can_display_process {
        println!("\n\t{}\n+)\t{}", a, b);
    }
    let mut ans = String::from("");
    let mut carry = 0i8;
    let mut bit_s: i8;
    for i in (0..8).rev() {
        let bit_a = String::from(&a[i..i+1]).parse::<i8>().unwrap();
        let bit_b = String::from(&b[i..i+1]).parse::<i8>().unwrap();
        let bit_add = bit_a + bit_b + carry;
        if  bit_add > 1 {
            bit_s = if bit_add == 2 {0} else {1};
            carry = 1;
        }
        else {
            bit_s = bit_add;
            carry = 0;
        }
        ans.push_str(&bit_s.to_string()[..]);
    }
    ans = ans.chars().rev().collect::<String>();
    if can_display_process {
        println!("─────────────────");
        println!("\t{} (carry: {})\n", ans, carry);
    }
    ans
}

fn minus(a: &String, b: &String, can_display_process: bool) -> String {
    if can_display_process {
        println!("\n\t{}\n-)\t{}", a, b);
        println!("─────────────────");
    }
    let b_complement = two_s_complement(&b, false);
    let ans = plus(&a, &b_complement, false);
    if can_display_process {
        println!("\t{}\n+)\t{} (2's)", a, b_complement);
        println!("─────────────────");
        println!("\t{} (carry: )\n", ans);
    }
    ans
}
