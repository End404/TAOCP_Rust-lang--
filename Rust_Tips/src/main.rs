use std::convert::TryInto;

fn main() {
    let a: i32 = 10;
    let b: u16 = 100;
    
    if a<(b as i32) {
        println!("10 is less than 100. ")
    }
    
    let c: i32 = 1203414;
    println!("{}", c as i8);

// 方法2
    let b_ = b.try_into().unwrap();
    if a<b_ {
        println!("10(a) is less than 100(b_).")
    }
}
