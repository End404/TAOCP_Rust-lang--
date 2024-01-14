/*
第3章 高级trait和类型

*/

fn main(){
    pub trait Iterator {
        type Item;
        fn next(&mut self) -> Option<Self::Item>;
    }
    pub trait Iterator<T> {
        fn next(&mut self) -> Option<T>;
    }
}