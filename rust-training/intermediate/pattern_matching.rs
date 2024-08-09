enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("O valor da moeda é: {} centavos", value_in_cents(coin));

    let some_option = Some(5);
    if let Some(value) = some_option {
        println!("O valor é: {}", value);
    } else {
        println!("Não há valor");
    }
}
