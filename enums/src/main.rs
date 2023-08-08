#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

#[derive(Debug)]
enum US_State {
    Alaska,
    Colorado
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(US_State),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("US State quarter from {:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1),
    }
}


fn main() {
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    println!("home is {:?}", home);
    println!("loopback is {:?}", loopback);
    
    let some_number = Some(5);

    let absent_number: Option<i32> = None;


    let total_value = value_in_cents(Coin::Quarter(US_State::Colorado)) + value_in_cents(Coin::Dime) + value_in_cents(Coin::Nickel) + value_in_cents(Coin::Penny);
    println!("total value is {}", total_value);

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("five is {:?}", five);
    println!("six is {:?}", six);
    println!("none is {:?}", none);


    let dice_roll = 5;

    match dice_roll {
        _ => println!("You rolled a {}", dice_roll),
    }


    let config_max = Some(4u8);

    if let Some(max) = config_max {
        println!("Max is {}", max);
    }

    let mut count = 0;
    let coin = Coin::Quarter(US_State::Alaska);
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}", state);
    } else {
        count += 1;
    }
}
