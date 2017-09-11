#[macro_use] extern crate text_io;

fn search(s: &String, n: u32) -> &str{
    let bytes = s.as_bytes();
    let (mut start, mut word_count): (usize, u32) = (0, 1);
    let mut litem = b'0';
    for (i, &item) in bytes.iter().enumerate() {
        if word_count == n && item == b' ' {
            return &s[start..i];
        }
        if item != b' ' && litem == b' ' {
            start = i;
            word_count += 1;
        }
        litem = item;
    }
    &s[..] // not exist
}

fn main() {
    loop {
        println!("input string:");
        let s: String;
        scan!("{}\n", s);
        if s == "0" {
            break;
        }
        let a: u32;
        println!("input number:");
        scan!("{}", a);
        let word = search(&s, a);
        println!("The {} word is '{}'", a, word);
    }
}
