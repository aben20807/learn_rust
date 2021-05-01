pub mod sub_module1;

mod sub_module2;
use sub_module2::Sub2;

pub struct Mod1 {}
impl Mod1 {
    pub fn call_sub2(i: i32) {
        let s2 = Sub2::new(i);
        println!("{}", s2.get_y());
    }
}
