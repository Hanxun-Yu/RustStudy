fn main() {
    let mut s = String::from("Hello"); //heap上申请
    s.push_str(" World!");
    println!("{}", s);

    //rust使用作用域判断，释放内存

    let _s2 = s; //这里没有重新分配String
                 //如果在上面这行代码后，继续使用s，编译将报错
                 //这是因为当2个变量指向同一片heap时，会引发2次释放
                 //所以当 _s2=s;后，会触发rust的move机制，s将失效，若使用s，下面这行编译会报错
                 // println!("{}",s);

    //深拷贝 (clone)
    let mut s = String::from("Hello"); //heap上申请
    let _s2 = s.clone();
    println!("s:{}, _s2:{}", s, _s2);

    test_send_arg_to_function();
}

fn test_send_arg_to_function() {
    let mut s = String::from("Hello World!");

    //这里给函数传String，会发生move
    //若在下一行继续使用这个String，编译会报错，因为当传入函数时s已经失效
    // take_ownership(s);
    // println!("{}", s);

    //使用函数返回值，使s继续有效
    let (s, s_len) = takes_and_gives_back(s);
    println!("\"{}\".len()={}", s, s_len);

    //使用引用（借用行为），使s继续有效(引用：允许你使用某些值，而不取得其所有权，类似指针)
    let s_len = use_reference(&s);
    println!("\"{}\".len()={}", s, s_len);

    //使用可变引用(注意：使用可变引用时，变量声明 和 传入时，都需要使用mut修饰)
    let mut s = String::from("Hello World!");
    let s_len = use_mutable_reference(&mut s);
    println!("\"{}\".len()={}", s, s_len);


    let num = 4;
    make_copy(num);
}

fn take_ownership(str: String) {
    println!("take_ownership str:{}", str);
}

fn make_copy(num: i32) {
    println!("make_copy num:{}", num);
}

fn takes_and_gives_back(str: String) -> (String, usize) {
    //...做一些处理
    let len = str.len();
    (str, len) //直接返回
}

fn use_reference(str: &String) -> usize {
    //这里若要修改引用参数str将报错，因为这里是借用的行为，默认不可以修改
    // str.push_str("hahah");
    str.len()
}

fn use_mutable_reference(str: &mut String) -> usize {
    //因为使用了mut修饰，所以引用str可被修改
    str.push_str("hahaha");
    str.len()
}
