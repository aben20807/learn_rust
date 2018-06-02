fn main() {
    println!("{}", longest("a", "b"));
}
/*
fn t1() {
    let y;
    {
        let x = "OuO";
        y = &x;
    }
    println!("{}", y);
}

fn t2() {
    let y;
    let x = "OuO";
    y = &x;
    println!("{}", y);
}
*/
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}

fn l(x: &i32) -> &str {
    "o"
}
