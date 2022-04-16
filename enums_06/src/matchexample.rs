#[derive(Debug)] // so we can inspect the states
pub enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

pub enum UsCoins{
    Penny,
    Nickel, 
    Dime, 
    Quarter(UsState),
}

pub fn value_in_cents_us(coin: UsCoins) -> u8 {
    match coin {
        UsCoins::Penny => 1,
        UsCoins::Nickel => 5,
        UsCoins::Dime => 10,
        UsCoins::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }       
    }
}

fn plus_one(x: Option<i32>) -> Option<i32>{
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn ignore_val(x: u32) -> u32{
    match x {
        1 => 1,
        2 => 2,
        _ => 3
    }
}

fn plus_one_call(){
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}