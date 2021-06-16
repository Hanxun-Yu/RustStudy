fn main() {
    println!("Hello, world!");

    // var();
    // type_();
    // another_function(3, 4);
    _loop();
}

fn type_() {
    /**
     * rust 是强类型语言，可在编译时进行推断
     */
    //使用parse()，接受返回值的变量必须显示声明类型
    let guess: u32 = "42".parse().expect("not a number");
    println!("{}", guess);

    //标量类型
    //整数类型
    //u8-128 u8-128
    //isize usize 由计算机架构决定，32位或64位 对应 i32 i64 u32 u64

    //浮点类型
    //f32 f64(default)
    let x = 2.0; //f64
    let y: f32 = 2.0; //f32

    //字符类型
    let c: char = 'a';
    let c = 'a';
    let c = '😂';

    //复合类型
    //元组tuple(内部可以给每个元素定义不同类型，可以理解为一个匿名临时结构体)
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    println!("{},{},{},", tup.0, tup.1, tup.2);

    //数组
    let array: [i32; 5] = [1, 2, 3, 4, 5]; //[<元素类型>;<元素个数>]
                                           // let array = [3; 5]; // = [<值>;<元素个数>] 相当于 let array = [3,3,3,3,3]
    println!("array[0]:{}", array[0])
}

fn var() {
    //变量-不可变
    let x = 5;
    println!("The value of x is {}", x);

    // x = 6;//会报错

    //变量-可变 加上mut，就可变
    let mut x = 6;
    x = 5;

    //常量
    const MAX_POINTS: u32 = 100_00;

    //Shadowing 重复声明，之前声明的将隐藏
    let y = 5;
    let y = y + 1; //之前y=5会被shadow
    let y = y * 1;

    //Shadowing 重复声明，可改变类型
    let spaces = "   ";
    let spaces = spaces.len();
}

//函数参数必须写明参数
fn another_function(x: i32, y: i32) {
    println!("the value of x is:{}", x);
    println!("the value of y is:{}", y);

    println!("five() result:{}", five());
}

fn five() -> i32 {
    // 5 //可以就写一个5，用表达式方式返回5，注意表达式方式，不加分号
    return 5; //也可以用return 关键字返回5
}

//表达式和语句
fn expression_statement() {
    let y = {
        let x = 1;
        x + 3 //表示现在整个{}闭包，是一个表达式，返回值是x+3，注意这里不能加分号;

        // x+3; //如果这里加分号;,这就变成了一个语句，语句是没有返回值的
    };

    println!("the value of y is:{}", y);
}

fn process_control() {
    let number = 5;

    if number < 5 {
        //
    } else {
        //
    }

    //match写法
    // match number.cmp(5) {

    // }

    //if是一个表达式，所以可以赋值给let
    let condition: bool = false;
    let number = if condition { 5 } else { 6 }; //这里2个{}必须类型相同
}

fn _loop() {
    loop {
        //

        break;
    }

    //loop 可以作为表达式
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; //使用break 返回值
        }
    };

    println!("The result is:{}", result);

    let mut index = 0;
    while index < 5 {
        break;
    }

    let a = [1, 2, 3, 4, 5];

    for element in a.iter() {
        println!("the element is :{}", element);
    }

    println!();
    //(1..4) 表示1到4，不包括4， rev()翻转
    for number in (1..4).rev() {
        println!("the element is :{}", number);
    }
}
