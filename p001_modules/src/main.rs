mod module1;
use module1::sub_module1::Sub1;
// use module1::sub_module2::Sub2; // error because Sub2 not pub
use module1::*; // use Mod1
mod module2;
use module2::sub_module3::{Sub3, Sub4};
use module2::use_sub1_in_mod2;

fn main() {
    println!("Hello, world!");

    let s1 = Sub1::new(10);
    println!("{}", s1.get_x());

    // let s2 = Sub2::new(20);
    // println!("{}", s2.get_y());
    Mod1::call_sub2(20);

    let s3 = Sub3::new(30);
    println!("{}", s3.get_z());

    let s4 = Sub4::new(40);
    println!("{}", s4.get_w());

    use_sub1_in_mod2();
}
