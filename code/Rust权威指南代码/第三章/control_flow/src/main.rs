fn main() {
    println!("Hello, world!");

    // Rust 代码中最常见的用来控制执行流的结构是 if 表达式和循环。
    // if 表达式允许根据条件执行不同的代码分支。你提供一个条件并表示 “如果条件满足，运行这段代码；如果条件不满足，不运行这段代码。”
    let number = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    // Rust中if期望一个 bool。
    // 不像 Ruby 或 JavaScript 这样的语言，Rust 并不会尝试自动地将非布尔值转换为布尔值。
    // 必须总是显式地使用布尔值作为 if 的条件。
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

    // 可以将 else if 表达式与 if 和 else 组合来实现多重条件。
    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    // 因为 if 是一个表达式，我们可以在 let 语句的右侧使用它
    let condition = true;
    let number = if condition {
        5
    } else {
        6
    };
    // if 的每个分支的可能的返回值都必须是相同类型
    println!("The value of number is: {}", number);

    // 一个循环执行循环体中的代码直到结尾并紧接着回到开头继续执行。
    // Rust 有三种循环：loop、while 和 for
    // loop {
    //     println!("again!");
    // }
    
    // loop 的一个用例是重试可能会失败的操作，比如检查线程是否完成了任务。
    // 然而你可能会需要将操作的结果传递给其它的代码。
    // 如果将返回值加入你用来停止循环的 break 表达式，它会被停止的循环返回
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result); // 20

    // while循环
    let mut number = 3;

    while number != 0 {
        println!("{}!", number);

        number = number - 1;
    }

    println!("LIFTOFF!!!");

    // for循环
    // 可以使用 while 结构来遍历集合中的元素，比如数组。
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index = index + 1;
    }

    // for循环遍历数组元素
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    // 使用for循环来倒计时
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
    
}
