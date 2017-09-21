fn take_slice_return_slice(s: &str) -> &str {
    s
}

fn take_string_return_slice(s: &String) -> &str {
    &s[..]
}

fn main() {
    let s = String::from("Hello, world!");
    println!("{}", s);

    let slice = &s[..];
    println!("{}", slice);

    let slice2 = take_slice_return_slice(slice);
    println!("{}", slice2);

    let slice3 = take_string_return_slice(&s);
    println!("{}", slice3);
}
