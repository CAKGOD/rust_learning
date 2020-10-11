fn main() {
    println!("Hello, world!");

    // 可变引用
    let mut s = String::from("hello");

    change(&mut s);

    // 可变引用有一个很大的限制：在特定作用域中的特定数据只能有一个可变引用。
    // let r1 = &mut s;
    // let r2 = &mut s; 这种写法是错误的。
    // 如果添加了大括号就可以允许多个变量引用，不过不能同时拥有。
    // {
    // let r1 = &mut s;
    // } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用
    // let r2 = &mut s;
    
    
    // 注意不可变引用，多个不可变引用是可以的。
    // let mut s = String::from("hello");
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // let r3 = &mut s; // 大问题
    // println!("{}, {}, and {}", r1, r2, r3);
    // 不能在拥有不可变引用的同时拥有可变引用。
    
    
    // let mut s = String::from("hello");
    // let r1 = &s; // 没问题
    // let r2 = &s; // 没问题
    // println!("{} and {}", r1, r2);  // 此位置之后 r1 和 r2 不再使用
    // let r3 = &mut s; // 没问题
    // println!("{}", r3);
    // 不可变引用 r1 和 r2 的作用域在 println! 最后一次使用之后结束，这也是创建可变引用 r3 的地方。
    // 它们的作用域没有重叠，所以代码是可以编译的。
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
