use rand::Rng;
use std::io;

pub fn guess_number(){
    println!("   Guessing game");
    println!("?=================?");
    println!("Please enter your Guess(The Random Number range is upto 15): ");

    let mut random_thread = rand::thread_rng();
    let random_number = random_thread.gen_range(1..15);

    let mut user_input : String = String::new();
    io::stdin().read_line(&mut user_input).expect("Failed to Read user input");

    let user_input_number : i32 = user_input.trim().parse().expect("Failed to Parse it into a Number");

    let mut range = 5;
    let mut number_of_tries = 0;

    while range > 0{ 
        if random_number == user_input_number{
        println!("You have Guessed it Right!");
        println!("It only took you {} number of tries",number_of_tries);
        break;
    }
    if random_number != user_input_number{
        println!("Try Again!,the Random Number was {}",random_number);
        number_of_tries += 1;
        range -= 1;
    }
    
    }
    println!("Awww you have failed to guess the random_number,better luck next time");
    


}