struct a {
    name: &String,
    age: i32
}

struct D (i32, String, u8);

fn main() {

    let b = a {
        name: &String::from("klq"),
        age: 26,
    };

    let c = a {
        name: &String::from("scq"),
        ..b

    };

    let e = D(1, String::from("cak"), 1);
}
