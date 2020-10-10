fn main() {
    println!("Hello, world!");

    // Rust 代码中的函数和变量名使用 snake case 规范风格。
    // 在 snake case 中，所有字母都是小写并使用下划线分隔单词。
    another_function();
    // another_function 定义在 main 函数 之后；也可以定义在之前。
    
    // 函數參數
    // 函数也可以被定义为拥有参数（parameters），参数是特殊变量，是函数签名的一部分。
    // 当函数拥有参数（形参）时，可以为这些参数提供具体的值（实参）。
    // 技术上讲，这些具体值被称为参数（arguments），
    another_function_1(5);
    // 在函数签名中，必须声明每个参数的类型。 
    
    // 当一个函数有多个参数时，使用逗号分隔
    another_function_2(5, 6);

    // let x = (let y = 6); // 这种写法是错误的
    
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("The value of y is: {}", y); // 4

    // 具有返回值的函数
    // 函数可以向调用它的代码返回值。
    // Rust并不对返回值命名，但要在箭头（->）后声明它的类型。
    // 在 Rust 中，函数的返回值等同于函数体最后一个表达式的值。
    // 使用 return 关键字和指定值，可从函数中提前返回；
    // 但大部分函数隐式的返回最后的表达式。
    let x = five();

    println!("The value of x is: {}", x);

    let x = plus_one(5);

    println!("The value of x is: {}", x);


    
}

fn another_function() {
    println!("Another function.");
}

fn another_function_1(x: i32) {
    println!("Another function 1.");
    println!("The value of x is: {}", x);
}

fn another_function_2(x: i32, y: i32) {
    println!("Another function 2.");
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn five() -> i32 {
    5
}

fn plus_one(x: i32) -> i32 {
    x + 1
}

