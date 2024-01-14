/*
    第1章 函数式编程
    闭包
    迭代器

*/

// use std::{vec, path::Iter};



fn main() {
    // println!("Hello, world!");

    /*
    fn sum(x:i32,y:i32) -> i32 {
        x + y
    }
    let sum_c = |x,y| x+y;
    let i =sum(1, 3);
    println!("{}", i);
    let i = sum_c(1,3);
    println!("{}", i);
    */

    // 迭代器
    let vec = Counter(0);
    // for i in vec {
    //     println!("{}", i)
    // }
    vec.map(|v| v+1).for_each(|v| print!("{}", v));

}

struct Counter(usize);
impl Iterator for Counter {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.0 += 1;
        if self.0 < 6 {
            Some(self.0)
        }else {
            None
        }
    }
}
