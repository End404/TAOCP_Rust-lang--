/*
第3章 高级trait和类型

*/
 
fn main(){
    /*
    //  高级trait
    let mut test = Test {};
    test.next1();
     */
    
    // 类型
    // type NewFnType = ;
    type Ne = Box<dyn Fn() + Send + 'static'>;
    let f = Box::new(|| printintn!("hi"));
    fn takes_long_type(f:Ne){
        f();
    }
    takes_long_type(f);
    
}

/*
// 高级trait
struct Test{

}
pub trait Iterator1 {
    type Item = usize;
    fn next1(&mut self) -> Option<Self::Item>;
}
impl Iterator1 for Test {
    type Item =();

    fn next1(&mut self) -> Option<Self::Item>{
        Some(1)
    }
}

pub trait Iterator2<T> {
    fn next2(&mut self) -> Option<T>;
}
impl Iterator2<T> for Test {
    fn next2(&mut self) -> Option<T> {
        Some()
    }
}
 */
