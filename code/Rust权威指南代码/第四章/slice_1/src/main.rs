fn main() {
    println!("Hello, world!");

    let mut s = String::from("hello world");

    let hello = &s[0..5];
    // let hello = &s[..5]; // 该写法也是正确的
    let world = &s[6..11];
    // let world = &s[6..]; // 该写法也是正确的
    
    let hello_world = &s[0..11];
    // let hello_world = &s[..]; // 该写法也是正确的

    // 可以使用一个由中括号中的 [starting_index..ending_index] 指定的 range 创建一个 slice，
    // 其中 starting_index 是 slice 的第一个位置，ending_index 则是 slice 最后一个位置的后一个值。
    // 在其内部，slice 的数据结构存储了 slice 的开始位置和长度，
    // 长度对应于 ending_index 减去 starting_index 的值。

    let word = first_word(&s);

    // s.clear() // 该写法是错误的
                 // 当拥有某值的不可变引用时，就不能再获取一个可变引用。
                 // 因为 clear 需要清空 String，它尝试获取一个可变引用。
                 // Rust不允许这样做，因而编译失败。

    println!("the first word is: {}", word);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}


