#[macro_use] extern crate text_io;

fn search(s: &String, n: u32) -> &str{
    let bytes = s.as_bytes();
    let (mut start, mut n): (usize, u32) = (0, n);
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' && n == 1 {
            // println!("n == 1");
            return &s[start..i];
        }
        else if item == b' ' {
            start = i+1;
            n -= 1;
            // println!("{}", n);
        }
    }
    &s[start..]
}

fn main() {
    loop {
        println!("input string:");
        let s: String;
        scan!("{}\n", s);
        if s == "0" {
            break;
        }
        // println!("{}", s);
        let a: u32;
        println!("input number:");
        scan!("{}", a);
        // println!("{}", a);
        let word = search(&s, a);
        println!("The {} word is '{}'", a, word);
    }
}
