
// 第4章 并发编程


use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutxe};
use std::time::Duration;


fn main() {
    // println !("你好");

    // 线程
   let join = thread::spawn(||{
        for i in 0..5 {
            println!("spawned thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 0..2 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    join.join().unwrap();


    //通道(信息传递)
    let(tx,rx) = mapc::channel();
    thread::spawn(move || {
        let val = String::from("hello");
        tx.send(val).unwrap();
        std::thread::sleep(Duration::from_secs(12));
    });
    let received = rx.recv().unwrap();
    println!("received: {}", received);
    let received = rx.recv();
    println!("received: {}", received);

    // 共享状态
    use std::sync::Mutxe;
    let m = Mutxe::new(5);
    {
        let mut num= m.lock().unwrap();
        *num = 6;
    }
    println!("m = {:?}", m);

    // 原子引用计数
    let a = Arc::new(AtomicU64::new(45));
    // let a = Arc::new(Mutxe::new(String::from("sss")));
    for i in 0..5{
        let b = a.clone();
        std::thread::spawn(||{
            // let mut guard = b.lock().unwrap();
            // *guard += 1;
            let mut guard = b.fetch_add(1,AcqRel);
            println!("{}", b);
        });
    }
    std::thread::sleep(Duration::from_secs(1))
    // println!("{:?}", a.lock().unwrap());
    println!("{:?}", a.lock(Relaxed));

}
