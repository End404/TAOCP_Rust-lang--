/*
第4章 错误处理
    4.1 不可恢复错误panic
    4.2 可恢复错误result
 */



use std::fmt::Error;
use std::fs::File;
use std::io;
use std::io::Read;



fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    /* let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e)
    }; */

    let mut s = String::new();

    /* match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e)
    } */
    f.read_to_string(&mut s)?;
    Ok(s)
}


#[test]
fn test4() {
    // panic!("crush and burn");

    let username:Result<String, Error> = read_username_from_file();
    println!("username {:?}", username);

    let mut f = File::open("hello.txt");
    // let mut f=match f {
    //     Ok(file) => file,
    //     Err(e) => panic!("错误信息：{:?}", e)
    // };
    let f = match f {
        Ok(file) => file,
        Err(error)=> match File::create("hello.txt") {
            Ok(fc) => fc,
            Err(e) => panic!("创建文件失败：{:?}", e)
        },
        other_error=> panic!("打开文件失败：{:?}",other_error)
    };
}
