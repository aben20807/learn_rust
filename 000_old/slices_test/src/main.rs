#[allow(dead_code)]
fn take_slice_return_slice(s: &str) -> &str {
    &s[..]
}

fn take_string_return_slice(s: &String) -> &str {
    &s[..]
}

#[allow(dead_code)]
fn main() {
    let s = String::from("Hello World");
    println!("{}", s);

    let slice: &str = &s[..=4];
    println!("{}", slice);

    let slice: &str = &s[6..];
    println!("{}", slice);

    let slice_slice: &str = &slice[..];
    println!("{}", slice_slice);

    let slice2 = take_slice_return_slice(slice);
    println!("{}", slice2);

    let slice3 = take_string_return_slice(&s);
    println!("{}", slice3);

    // let mut s = "Hello world".to_string();
    // let first_index = first_word_x(&s);
    // s.clear();
    // println!("{}", first_index);
    // println!("{}", s.as_bytes()[first_index]);

    let mut s = "Hello world".to_string();
    let s1 = "Hello world";
    {
        let first_index = first_word(&s);
        println!("{}", first_index);
        let first_index = first_word(&s1);
        println!("{}", first_index);
    }
    s.clear();
}

#[allow(dead_code)]
fn first_word_x(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}

#[allow(dead_code)]
fn first_word_xx(s: &String) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}
