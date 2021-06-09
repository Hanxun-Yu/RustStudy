use std::io;
 //use是显式导入， rust默认使用prelude导入，若不在内，则需要显式导入

fn main() {
    println!("Hello, world!");


    //mut 可修改
    let mut guess = String::new();

    //读取一行 赋值给guess
    io::stdin().read_line(&mut guess)
    .expect("无法读取行");

    println!("你猜测的数是：{}",guess);
}
