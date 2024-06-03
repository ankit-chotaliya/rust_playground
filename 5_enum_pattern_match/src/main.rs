#[derive(Debug)]
enum InStats {
    Gujarat,
    Maharashtra,
    Rajasthan,
    Bihar,
    UttarPradesh,
    Uttarakhand,
}

#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(InStats),
}

fn value_in_rupees(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("{:?}", coin);
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("{:?}", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
    // OR
    // match x {
    //     Some(i) => Some(i + 1),
    //     _ => None,
    // }
}
fn main() {
    // Ex - 1
    let coin: Coin = Coin::Penny;
    println!("{:?}", coin);
    let state: InStats = InStats::Gujarat;
    println!("{:?}", state);
    value_in_rupees(Coin::Quarter(InStats::Gujarat));

    // Ex - 2
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    // Ex - 3
    let some_u8_value = Some(3);
    match some_u8_value {
        Some(1) => println!("one"),
        Some(3) => println!("three"),
        Some(5) => println!("five"),
        Some(7) => println!("seven"),
        _ => (),
    }

    // Ex - 5
    if let Some(3) = some_u8_value {
        println!("three");
    }
}
