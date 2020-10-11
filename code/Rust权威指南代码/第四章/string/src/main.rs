fn main() {
    println!("Hello, world!");

    let s = String::from("hello");

    // String类型被分配到堆上，所以能够存储在编译时未知大小的文本。
    // 可以使用from函数基于字符串字面值来创建 String
    // 这两个冒号（::）是运算符，允许将特定的 from 函数置于 String 类型的命名空间（namespace）下，而不需要使用类似 string_from 这样的名字。
    
    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str() 在字符串后追加字面值

    println!("{}", s); // 将打印 `hello, world!`

    // 在rust中，内存在拥有它的变量离开作用域后就被自动释放。
    // let s1 = String::from("hello");
    // let s2 = s1;
    // println!("{}, world!", s1);
    // 此时会报错，只是将s1移动到s2，并不是拷贝数据，只是移动了指针、长度和容量，因为此时s1已经被销毁。
    
    // 确实需要深度复制 String 中堆上的数据，而不仅仅是栈上的数据，可以使用一个叫做 clone的通用函数。
    let s1 = String::from("hello");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);

    // Rust 有一个叫做 Copy trait 的特殊注解，可以用在类似整型这样的存储在栈上的类型上（第十章详细讲解 trait）。
    // 如果一个类型拥有 Copy trait，一个旧的变量在将其赋值给其他变量后仍然可用。
    // Rust 不允许自身或其任何部分实现了 Drop trait 的类型使用 Copy trait。
    // 如下是一些 Copy 的类型：
    // 所有整数类型，比如 u32。
    // 布尔类型，bool，它的值是 true 和 false。
    // 所有浮点数类型，比如 f64。
    // 字符类型，char。
    // 元组，当且仅当其包含的类型也都是 Copy 的时候。比如，(i32, i32) 是 Copy 的，但 (i32, String) 就不是。
    
    let s = String::from("hello");  // s 进入作用域

    takes_ownership(s);             // s 的值移动到函数里 ...
                                    // ... 所以到这里不再有效

    let x = 5;                      // x 进入作用域

    makes_copy(x);                  // x 应该移动函数里，
                                    // 但 i32 是 Copy 的，所以在后面可继续使用 x 

}   // 注意，在}后，变量离开作用域，自动调用drop函数，释放内存。
    // 这里, x 先移出了作用域，然后是 s。但因为 s 的值已被移走，
    // 所以不会有特殊操作

fn takes_ownership(some_string: String) { // some_string 进入作用域
    println!("{}", some_string);
} // 这里，some_string 移出作用域并调用 `drop` 方法。占用的内存被释放

fn makes_copy(some_integer: i32) { // some_integer 进入作用域
    println!("{}", some_integer);
} // 这里，some_integer 移出作用域。不会有特殊操作


