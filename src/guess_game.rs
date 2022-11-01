use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
    println!("\nWelcome to the guessing game!");
    println!("Enter your guess value");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut user_guessed_value = String::new();
        io::stdin()
            .read_line(&mut user_guessed_value)
            .expect("Failed to read line");

        let user_guessed_value: u32 = match user_guessed_value.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match user_guessed_value.cmp(&random_number) {
            Ordering::Less => println!("Too less!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("hurray! 🦄 that was a match");
                break;
            }
        }
    }
    // println!("You guessed :{user_guessed_value}, Actual :{random_number}")
}
