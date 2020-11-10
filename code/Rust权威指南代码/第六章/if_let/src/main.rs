#[derive(Debug)]
enum UsState {
   Alabama,
   Alaska,
}

enum Coin {
   Penny,
   Nickel,
   Dime,
   Quarter(UsState),
}

fn main() {
    //let some_u8_value = 1;
    //match some_u8_value {
    //    1 => println!("one"),
    //    3 => println!("three"),
    //    5 => println!("five"),
    //    7 => println!("seven"),
    //    _ => (),
    //}
    
    //let some_u8_value = Some(3);
    //match some_u8_value {
    //    Some(3) => println!("Three"),
    //    _ => (),
    //}
    
    //let some_u8_value = Some(3);
    //if let Some(3) = some_u8_value {
    //println!("three");
    //}
    
    let coin = Coin::Quarter(UsState::Alaska);
    let mut count = 0;
    //match coin {
    //    Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //    _ => count += 1,
    //}
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
}
