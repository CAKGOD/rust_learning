fn main() {
    println!("Hello, world!");

    // 在具有指针的语言中，很容易通过释放内存时保留指向它的指针而错误地生成一个 悬垂指针（dangling pointer），
    // 所谓悬垂指针是其指向的内存可能已经被分配给其它持有者。
    // 相比之下，在 Rust 中编译器确保引用永远也不会变成悬垂状态：
    // 当你拥有一些数据的引用，编译器确保数据不会在其引用之前离开作用域。
    
    // let reference_to_nothing = dangle();
    let reference_to_nothing = no_dangle();
}

// fn dangle() -> &String { // dangle 返回一个字符串的引用
//
//     let s = String::from("hello"); // s 是一个新字符串
// 
//     &s // 返回字符串 s 的引用
// } // 这里 s 离开作用域并被丢弃。其内存被释放。
     // 危险！

fn no_dangle() -> String {
    let s = String::from("hello");

    s
}

// conclusion
// 在任意给定时间，要么 只能有一个可变引用，要么 只能有多个不可变引用。
// 引用必须总是有效的。
     
