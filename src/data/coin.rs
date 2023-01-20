enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => {
            println!("Lucky with Penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime =>10,
        Coin::Quarter => 25,
    }
}

pub fn show_coins () {
    let nickel: Coin = Coin::Nickel;
    let penny: Coin = Coin::Penny;
    let quarter: Coin = Coin::Quarter;
    let dime: Coin = Coin::Dime;

    let penny_result = value_in_cents(penny);
    let quarter_result = value_in_cents(quarter);
    let dime_result = value_in_cents(dime);
    let nickel_value = value_in_cents(nickel);

    println!("Nickel is {}", nickel_value);
    println!("Penny is {}",penny_result);
    println!("Quarter is {}", quarter_result);
    println!("Dime is {}", dime_result);

}