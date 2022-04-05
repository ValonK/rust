use std::{io, cmp::Ordering}; // brining multiple traits from std scope (importing io & cmp::Ordering) Ordering is an enum 
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let random_number = generate_random_number(1,101);

    println!("The secret number is: {}", random_number);
    
    loop {
        let user_guess = get_user_guess();
        match user_guess.cmp(&random_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal =>{
                println!();
                println!("You win! The number was {}", random_number);
                println!();
                break;
            }         
        }        
    }
}

fn get_user_guess() -> u32 {
    let mut guess = String::new();
    loop {
        println!("Enter the number");
    
        match io::stdin()
            .read_line(&mut guess){
                Ok(guess) => guess,
                Err(_) => continue
            };
    
        let guess : u32 = match guess.trim()
            .parse(){
                Ok(guess) => guess,
                Err(_) => continue,
            };
        return guess;
    }
}

fn generate_random_number(min: u32, max: u32) -> u32 {
    return rand::thread_rng().gen_range(min..max);
}
