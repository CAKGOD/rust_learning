fn main() {
    println!("Hello, world!");

    // let x = 5;  // 如果不添加mut关键字，之后的操作不会成功
    let mut x = 5;
    println!("The value of x is: {}.", x);

    x = 6;
    println!("The value of x is: {}.", x);

    const MAX_POINTS: u32 = 100_000; // 约定俗成：以下划线分隔的全大写字母命名一个常量，数值中加入下划线提高可读性。

    let y = 5;

    let y = y + 1;

    let y = y * 2; // 第一个变量被第二个变量隐藏（shadow）了

    println!("The value of y is: {}.", y);
    
    let spaces = " ";

    let spaces = space.len(); // 隐藏机制和mut的另一个区别在于：由于重复使用let关键字会创造出新变量，因此可以在复用同名变量的时候改变其类型。

    // 标量类型（scalar）
    // 四种：整数、浮点数、布尔值和字符
    //
    // Rust中的整数类型：
    //      长度          有符号          无符号
    //     8-bit            i8              u8
    //     16-bit           i16             u16
    //     32-bit           i32             u32
    //     64-bit           i64             u64
    //     arch             isize           usize
    // 对于一个位数为n的有符号的整数类型，可以存储-2^(n-1) ~ 2^(n-1)范围内的所有整数
    // 对于一个位数为n的无符号的整数类型，可以存储0 ~ 2^n-1范围内的所有整数
    // isize和usize两种特殊的整数类型的长度取决于程序运行的目标平台，在64位架构上就是64位，在32位架构上就是32位的。
    // 除了Byte，其余所有的字面量都可以使用类型后缀
    

    // 复合类型（compound）
}
