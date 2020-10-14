fn main() {
    println!("Structure 0!");
    
    let mut user1 = User {
        email: String::from("17801030219@163.com"),
        username: String::from("chuankang"),
        sign_in_count: 1,
        active: true,
    };

    user1.email = String::from("cakgod9621@gmail.com");
    println!{"{}", user1.username};


    let email_1 = String::from("1@2.com");
    let username_1 = String::from("None");

    let mut user2 = build_user(email_1, username_1);
    println!("{}", user2.username);

    let user3 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: user1.active,
        sign_in_count: user1.sign_in_count,
    };

    let user4 = User {
        email: String::from("3@4.com"),
        ..user1 // 剩下的字段设置为跟user1相同
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_1(email: String, username: String) -> User {  // 此处参数名与结构体字段同名，可以简化写法
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
