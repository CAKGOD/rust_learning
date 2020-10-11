fn main() {
    println!("Hello, world!");

    // 编写一个函数，该函数接收一个字符串，并返回在该字符串中找到的第一个单词。
    // 如果函数在该字符串中并未找到空格，则整个字符串就是一个单词，所以应该返回整个字符串。
    let mut s = String::from("hello world");
    println!(s);
    let word = first_word(&s); // word 的值为 5
    println!(s);
    s.clear(); // 这清空了字符串，使其等于 ""

    // // word 在此处的值仍然是 5，
    // // 但是没有更多的字符串让我们可以有效地应用数值 5。word 的值现在完全无效！
}

fn first_word(s: &String) -> usize {
    
    let bytes = s.as_bytes(); // 用 as_bytes 方法将 String 转化为字节数组：

    for (i, &item) in bytes.iter().enumerate() { // 使用 iter 方法在字节数组上创建一个迭代器：
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


