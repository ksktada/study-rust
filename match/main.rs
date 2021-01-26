enum UsState {
    Alabama,
    Alaska,
}

enum Coin {
    Penny,
    Nikel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny -> 1,
        Coin::Nickel -> 5,
        Coin::Dime -> 10,
        Coin::Quarter(UsState) => {
            println("State quarter from {:?}", state);
            25
        }
    }

    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1
    }
}

fn if_let() {
    let some_u8_value = Some(0u8)
    match some_u8_value {
        Some(3) => println!("three"),
        _ => (),
    }

    if let Some(3) = some_u8_value {
        println!("three")
    }
}