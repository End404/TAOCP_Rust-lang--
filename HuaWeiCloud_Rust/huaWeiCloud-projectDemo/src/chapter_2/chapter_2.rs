/*
第2章 智能指针
    2.1 Box<T>
    2.2 Deref 以及 Drop trait
    2.3 引用计数 Rc<T> 和RefCell

*/


use std::cell::{Ref, RefCell};
use std::mem::size_of;
use std::ops::Deref;
use std::rc::Rc;


fn main() {
    /*
    // let b = Box::new(64);
    // println!("{}", size_of()::<Box<i32>>());
    // 2.2
    let a= MyBox::new(5);
    let a1 = MyBox::new(5);
    let c = a.deref() + a1.deref();
    println!("{:?}", c);
    */

    // 2.3
    // enum List {
    //     Cons(i32, Rc<List>),
    //     Nil,
    // }

    // let a =Rc::new(Test(1));
    let a =Rc::new(RefCell::new(Test(1)));
    {
        {
            let a1 = a.clone();
            // println!("test drop-1")
            println!("1 {}", a1.borrow().0);
        }
        let a2 = a.clone();
        println!("test drop-2")
    }
}

/*
// 2.2
struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
*/

// 2.3
struct Test(usize);
impl Drop for Test {
    fn drop(&mut self) {
        println!("test drop")
    }
}