/*
第2章 Rust基本概念
    变量
    函数
    基本数据类型
 */

// mod chapter_3;
mod chapter_4;

fn print_sum(a:i8, b:i8){
    println!("is:{}",a+b);
}
fn cal_sum(a:i8, b:i8) -> i8 {
    return a+b;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s:&mut String){
    s.push_str(", world");
}

fn main() {
    // println!("Hello, world!");

    // 变量
    let  c;
    let a:bool = true;
    let b: bool = true;
    c=5;

    static N: i32 = 5;

    const num: i32 = 5;

    // 基础数据类型
    let a_array:[i32;3] = [1,2,3];
    let mut mut_array:[i32;3] = [1,2,3];
    mut_array[1] = 2;

    let a_tuple:(i32,f64,&str) = (1,2.1,"s");
    let mut mut_tuple:(i32,f64,&str) = (1,1.5,"ss");
    mut_tuple.0 = 2;

    let a_slice:&[i32] = &a_array[0..2];
    println!("{:?}", a_slice);

    let a_str:&str = "hello world";
    let a_string:String = a_str.to_string();
    let b_string:String = String::from(a_str);

    // 操作符
    let d:f32 = (c as f32) / 2.0;
    println!("{}",d);

    let x:i32 = 5;
    let y:i32 = x;
    println!("x:{}, y{}",x,y);

    let mut s1:String = String::from("hello");
    let s2:String = s1.clone();
    println!("s1:{}, s2:{}", s1,s2);

    let length1:usize = calculate_length(&s1);
    println!("The长度: {}, is{}", s1,length1);

    change(&mut s1);
    println!("s1 is: {}", s1);
}
