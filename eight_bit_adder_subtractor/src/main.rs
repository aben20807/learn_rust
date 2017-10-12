#[macro_use] extern crate text_io;

fn main() {
    println!("Input: ");
    let s: String;
    scan!("{}\n", s);
    let (a, symbol, b) = statement_analysis(s);
    if symbol == '+' {
        is_overflow(&plus(&dec_to_bin(a), &dec_to_bin(b), true));
    }
    else if symbol == '-' {
        is_overflow(&minus(&dec_to_bin(a), &dec_to_bin(b), true));
    }
    else {
        println!("Wrong symbol!");
    }
}

fn statement_analysis(statement: String) -> (i8, char, i8) {
    for (i, c) in statement.chars().enumerate() {
        if i != 0 && (c == '+' || c == '-') {
            let a = statement[..i].parse::<i8>().unwrap();
            let b = statement[i+1..].parse::<i8>().unwrap();
            return (a, c, b)
        }
    }
    (0, '=', 0)
}

fn dec_to_bin(input: i8) -> String {
    let mut dec:i32 = input as i32; // Because -128 * (-1) will overflow
    let mut ans = String::new();
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

fn bin_to_dec(input: &String) -> i8 {
    let mut ans = 0i8;
    let base = 2i8;
    if &input[..1] == "0" {
        for i in 0..7 {
            ans = ans + base.pow(i as u32)*(input[7-i..8-i].parse::<i8>().unwrap());
        }
    }
    else {
        let tmp = two_s_complement(&input, false);
        for i in 0..7 {
            ans = ans + base.pow(i as u32)*(tmp[7-i..8-i].parse::<i8>().unwrap());
        }
        ans = ans * (-1);
        if ans == 0 && input[..1].parse::<i8>().unwrap() == 1 {
            ans = -128;
        }
    }
    ans
}

fn one_s_complement(input: &String) -> String {
    let mut ans = String::new();
    for i in 0..8 {
        let old_bit = input[i..i+1].parse::<i8>().unwrap();
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
        println!("\t{}\n", ans.0); // ans.1 is carry
    }
    ans.0
}

fn plus(a: &String, b: &String, can_display_process: bool) -> (String, i8, bool) {
    if can_display_process {
        println!("\n\t{} ({})\n+)\t{} ({})", a, bin_to_dec(&a), b, bin_to_dec(&b));
    }
    let mut ans = String::new();
    let mut carry = 0i8;
    let mut bit_s: i8;
    for i in (0..8).rev() {
        let bit_a = a[i..i+1].parse::<i8>().unwrap();
        let bit_b = b[i..i+1].parse::<i8>().unwrap();
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
        println!("\t{} ({})\t[carry: {}]", ans, bin_to_dec(&ans), carry);
    }
    let maybe_overflow =
        (a[..1].parse::<i8>().unwrap() == 0 &&
         b[..1].parse::<i8>().unwrap() == 0) ||
        (a[..1].parse::<i8>().unwrap() == 1 &&
         b[..1].parse::<i8>().unwrap() == 1);
    (ans, carry, maybe_overflow)
}

fn minus(a: &String, b: &String, can_display_process: bool) -> (String, i8, bool) {
    if can_display_process {
        println!("\n\t{} ({})\n-)\t{} ({})", a, bin_to_dec(&a), b, bin_to_dec(&b));
        println!("─────────────────");
    }
    let b_complement = two_s_complement(&b, false);
    let ans = plus(&a, &b_complement, false);
    let maybe_overflow = &b[..] != "00000000" && ( // -0 never overflow
        (a[..1].parse::<i8>().unwrap() == 0 &&
         b[..1].parse::<i8>().unwrap() == 1) ||
        (a[..1].parse::<i8>().unwrap() == 1 &&
         b[..1].parse::<i8>().unwrap() == 0));
    if can_display_process {
        println!("\t{} ({})\n+)\t{} ({})\t[2's]", a, bin_to_dec(&a), b_complement, bin_to_dec(&b_complement));
        println!("─────────────────");
        println!("\t{} ({})\t[carry: {}]", ans.0, bin_to_dec(&ans.0), ans.1);
    }
    (ans.0, ans.1, maybe_overflow)
}

#[allow(unused)]
fn is_overflow(input: &(String, i8, bool)) -> bool {
    if input.2 && input.0[..1].parse::<i8>().unwrap() != input.1 {
        println!("\t\t\t[overflow: {}]", true);
        return true;
    }
    println!("\t\t\t[overflow: {}]", false);
    false
}

#[allow(unused)]
fn overflow_check_all() {
    for i in -128..-126 {
        for j in 0..1 {
            let ans = plus(&dec_to_bin(i as i8), &dec_to_bin(j as i8), false);
            if (i+j != bin_to_dec(&ans.0) as i32) != is_overflow(&ans) {
                print!("({}+{})", i, j);
            }
        }
    }
    for i in -128..128 {
        for j in -128..128 {
            let ans = minus(&dec_to_bin(i as i8), &dec_to_bin(j as i8), false);
            if (i-j != bin_to_dec(&ans.0) as i32) != is_overflow(&ans) {
                print!("({}+{})", i, j);
            }
        }
    }
    println!("\ndone");
}
