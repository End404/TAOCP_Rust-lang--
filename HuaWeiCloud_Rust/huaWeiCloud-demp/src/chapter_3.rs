/*
第3章 其他数据结构
    结构体
    枚举
    泛型
 */

use std::collections::HashMap;
use std::ffi::CString;
use std::fmt::Display;

struct  User{
    username:String,
    email:String,
    sign_in_count:u32,
    active:bool
}
struct Rectangle{
    x:u32,
    y:u32
}

impl Rectangle {
    fn area(&self) -> u32{
        self.x * self.y
    }
}

enum IpAddrKind {    //定义枚举
    V4,
    V6,
}
fn route(ip_type:IpAddrKind) {}

struct IpAddr{
    kind:IpAddrKind,
    address:String
}
fn ipaddr_match(ip_type:IpAddrKind) -> u8 {
    match ip_type {
        IpAddrKind::V4 => 1,
        IpAddrKind::V6 => 2
    }
}

struct point<T>{
    x:T,
    y:T,
}

struct NewsArticle{
    headline:String,
    location:String,
    author:String,
    content:String,
}
struct Tweet{
    username:String,
    content:String,
    reply:bool,
    retweet:bool
}
trait Summary{
    fn summarize(&self) -> String;
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
impl Summary for Tweet{
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)    }
}

impl <T: Display + PartialOrd> point<T>{
    fn cmp_disply(&self){
        if self.x > self.y{
            println!("The largest number is x={}", self.x)
        }else { println!("The largest number is y={}", self.y) }
    }
}

// fn notify(item: impl Summary, item2: impl Summary){
//     println!("breaking new! {}", item.summarize());
// }
// fn notify<T: Summary>(itme1:T, item2:T){}
fn notify(item:impl Display+Summary){}


#[test]
fn test3(){
    let mut user1 = User{
        username: String::from("abc"),
        email: String::from("abc@examp"),
        sign_in_count: 1,
        active: true
    };
    let email:String = user1.email;
    let ret1 = Rectangle{x:1, y:2};
    let area:u32 = ret1.area();
    println!("面积：{}", area);

    // 使用枚举
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    let home = IpAddrKind {      //枚举成员关联数据
        kind:IpAddrKind::V4,
        address:String::from("127.0.0.1")
    };

    let integer = point{
        x:1,
        y:2,
    };
    let float =point{
        x:1.0,
        y:2.0,
    };

    let article = NewsArticle{
        headline:String::from("abbb"),
        location:String::from("usa"),
        author:String::from("aaa"),
        content:String::from("bbb")
    };
    notify(article);

    // Vec
    let v = vec![1,2,3];
    let item1 = &v[1];
    let item2=v.get(100);

    // HashMap
    let mut scores:HashMap<String, i32>=HashMap::new();
    scores.insert(String::from("bule"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    println!("scores: {:?}", scores);
}