use std::io;
use std::cmp::Ordering; // Ordering也是枚举类型，有Less、Greater、Equal三个变体
use rand::Rng; // 这里的Rng是一个trait（特征），定义了随机数生成器需要实现的方法的集合

fn main() {
    println!("Guess the number");
    
    let secret_number = rand::thread_rng().gen_range(1, 101); // rand::thread_rng会返回一个特定的随机数生成器
                                                              // gen_range方法生成两者之间的随机数，包含下限不包含上限

    // println!("The secret number is: {}", secret_number);

    loop {
        // loop关键字会创建一个无限循环
        println!("Please input your guess.");
    
        let mut guess = String::new(); // let关键字创建了一个新变量
                                       // mut关键字说明guess变量是可变的，没有的话说明该变量是不可变的
                                       // //表示后面是注释，与c++相同
                                       // String是标准库中的一个字符串类型，使用了UTF-8格式编码并可以按照需求扩展自己的大小
                                       // ::说明new是String类型的一个关联函数（associated function），有时候也被称为静态方法
                                       // new创建了一个新的空白字符串，new是创建类型实例的惯用函数名称

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");  // mut说明guess参数是可变的
                                             // &说明后面的参数是一个引用
                                             // 引用和变量一样，默认不可变
                                             // read_line还会返回一个io::Result值，其中Result是一个枚举类型，有Ok和Err两个变体
                                             // 枚举类型由一系列固定的值组合成，这些值称为枚举的变体
                                             // 如果io::Result实例的值是Err，expect方法就会中断当前程序，并显示出传入的字符串参数
                                             // 如果io::Result实例的值是Ok，expect方法会提取出Ok中附带的值

        // let guess: u32 = guess.trim().parse()
        //    .expect("Please type a number~"); // Rust允许使用同名的新变量guess来隐藏（shadow）旧变量的值
                                              // trim方法会返回一个去除首尾所有空白字符的新字符串实例
                                              // parse方法会会尝试将当前字符串解析为某种数值

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,  
            Err(_) => continue,
        };  // Err(_)中的_是一个通配符，匹配所有可能的Err值
            // continue会直接跳至下一次循环

        println!("You guessed: {}", guess);  // {}是一个占位符，将后面的参数值插入预留的特定位置

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small~"),
            Ordering::Greater => println!("Too big~"),
            Ordering::Equal => {
                println!("You win~");
                break;
            } // break代表结束所有循环
        } // match由数个分支（arm）组成，每个arm都包含一个用于匹配的模式（pattern），以及匹配成功后要执行的代码
    }
}
