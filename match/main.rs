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
}