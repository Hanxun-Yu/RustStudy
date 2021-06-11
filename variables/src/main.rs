fn main() {
    println!("Hello, world!");

    // var();
    type_();
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
    println!("array[0]:{}",array[0])
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
