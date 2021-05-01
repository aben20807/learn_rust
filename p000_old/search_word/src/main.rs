#[macro_use] extern crate text_io;

fn search(s: &String, n: u32) -> (bool, &str) {
    let bytes = s.as_bytes();
    let (mut start, mut word_count): (usize, u32) = (0, 1);
    let mut litem = b'0';
    for (i, &item) in bytes.iter().enumerate() {
        if word_count == n && item == b' ' {
            return (true, &s[start..i]);
        }
        if item != b' ' && litem == b' ' {
            start = i;
            word_count += 1;
        }
        litem = item;
    }
    if word_count == n {
        return (true, &s[start..]);
    }
    (false, &s[..]) // not exist
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
        let (is_found, word) = search(&s, a);
        if is_found == true {
            println!("The {} word is '{}'", a, word);
        }
        else {
            println!("Word not found");
        }
    }
}
