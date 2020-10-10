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

    let spaces = spaces.len(); // 隐藏机制和mut的另一个区别在于：由于重复使用let关键字会创造出新变量，因此可以在复用同名变量的时候改变其类型。

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
    
    // Rust也有两个原生的浮点数（floating-point numbers）类型，它们是带小数点的数字。Rust的浮点数类型是f32和f64，分别占32位和64位。默认类型是f64，因为在现代CPU中，它与f32速度几乎一样，不过精度更高。
    // f32是单精度浮点数，f64是双精度浮点数。
    let x = 2.0; // f64
    
    let y: f32 = 3.0; // f32
    
    // 数值运算
    // 加法
    let sum = 5 + 10;

    // 减法
    let difference = 95.5 - 4.3;

    // 乘法
    let product = 4 * 30;

    // 除法
    let quotient = 56.7 / 32.2;

    // 取余
    let remainder = 43 % 5;

    // Rust中的布尔类型有两个可能的值：true和false。Rust中的布尔类型使用bool表示。
    let t = true;

    let f: bool = false; // 显式指定类型注解

    // Rust的char类型是语言中最原生的字母类型。
    // Rust的char类型的大小为四个字节(four bytes)，并代表了一个Unicode标量值（Unicode Scalar Value），这意味着它可以比ASCII表示更多内容。
    // 在Rust中，拼音字母（Accented letters），中文、日文、韩文等字符，emoji（绘文字）以及零长度的空白字符都是有效的char值。Unicode标量值包含从U+0000到U+D7FF和U+E000到U+10FFFF在内的值。
    let c = 'z';

    let z = 'ℤ';

    let heart_eyed_cat = '😻';

    // 复合类型（compound）
    // Rust有两个原生的复合类型：元组（tuple）和数组（array）。
    
    // 元组类型
    // 元组是一个将多个其他类型的值组合进一个复合类型的主要方式。
    // 元组长度固定：一旦声明，其长度不会增大或缩小。
    // 使用包含在圆括号中的逗号分隔的值列表来创建一个元组。
    // 元组中的每一个位置都有一个类型，而且这些不同值的类型也不必是相同的。
    let tup: (i32, f64, u8) = (500, 6.4, 1);

    // tup 变量绑定到整个元组上，因为元组是一个单独的复合元素。
    // 为了从元组中获取单个值，可以使用模式匹配（pattern matching）来解构（destructure）元组值。
    let tup = (500, 6.4, 1);

    let (x, y, z) = tup;

    println!("The value of y is: {}", y);

    // 除了使用模式匹配解构外，也可以使用点号（.）后跟值的索引来直接访问它们。
    let x: (i32, f64, u8) = (500, 6.4, 1);

    let five_hundred = x.0;

    let six_point_four = x.1;

    let one = x.2;

    // 数组类型
    // 与元组不同，数组中的每个元素的类型必须相同。
    // Rust中的数组与一些其他语言中的数组不同，因为Rust中的数组是固定长度的：一旦声明，它们的长度不能增长或缩小。
    let a = [1, 2, 3, 4, 5];

    // vector类型是标准库提供的一个允许增长和缩小长度的类似数组的集合类型。
    // 当不确定是应该使用数组还是vector的时候，你可能应该使用vector。
    let months = ["January", "February", "March", "April", "May", "June", "July",
                  "August", "September", "October", "November", "December"];

    // 可以像这样编写数组的类型：
    // 在方括号中包含每个元素的类型，后跟分号，再后跟数组元素的数量。
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // 如果要为每个元素创建包含相同值的数组，可以指定初始值，后跟分号，然后在方括号中指定数组的长度.
    let a = [3; 5];
    // 这种写法与 let a = [3, 3, 3, 3, 3]; 效果相同，但更简洁。
    
    // 可以使用索引来访问数组的元素
    let a = [1, 2, 3, 4, 5];

    let first = a[0];
    let second = a[1];
}
