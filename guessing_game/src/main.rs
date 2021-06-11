use std::io;
//use是显式导入， rust默认使用prelude导入，若不在内，则需要显式导入

use rand::Rng;
use std::cmp::Ordering; //trait trait表示定义了一堆方法,l类似java的interface

fn main() {
    println!("Hello, world!");

    //不显示声明的话默认是i32
    let secret_number = rand::thread_rng().gen_range(1, 101); // 从这个范围[1,101)获取随机数

    println!("神秘数字是:{}", secret_number);

    loop {
        //mut 可修改
        let mut guess = String::new();

        //读取一行 赋值给guess
        io::stdin().read_line(&mut guess).expect("无法读取行");

        println!("你猜测的数是：{}", guess);
        

        let found: bool = false;

        //这里变量重名了 shadow ，将覆盖掉之前的guess：String
        //这里显示声明了guess是u32类型，那后面的parse()就会解析成u32
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //这里把guess和secret_number比较，因为guess显示声明为u32
        //而secret_number是隐式声明，编译器会自动把secret_number改为u32
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("小了，继续猜，请输入："),
            Ordering::Greater => println!("大了，继续猜，请输入："),
            Ordering::Equal => {
                println!("对了");
                break;
            }
        }

        // if(int(guess) < secret_number) {
        //     io::stdin().read_line(&mut guess)
        //     .expect("无法读取行");
        // } else if(int(guess) > secret_number) {
        //     io::stdin().read_line(&mut guess)
        //     .expect("无法读取行");
        // } else {
        //     found = true;
        // }
    }
}
