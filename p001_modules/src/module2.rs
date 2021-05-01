pub mod sub_module3;

use crate::module1::sub_module1::Sub1;

pub fn use_sub1_in_mod2() {
    let s1 = Sub1::new(100);
    println!("{}", s1.get_x());
}
