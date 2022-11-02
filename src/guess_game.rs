use rand::Rng;
use std::cmp::Ordering;
use std::io;

pub fn guess_game() {
    println!("\n 🍔 Welcome to the guessing game!");
    println!(" 🥂 Enter your guess value");

    let random_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut user_guessed_value = String::new();
        io::stdin()
            .read_line(&mut user_guessed_value)
            .expect("Failed to read line");

        // let the user re-enter the guess value if it is not a number
        let user_guessed_value: u32 = match user_guessed_value.trim().parse() {
            Ok(num) => {
                if num > 100 {
                    println!(" 🧩 You can choose between 1 to 100 🙉");
                    continue;
                } else {
                    num
                }
            }
            Err(_) => {
                println!(" 🥊 Only numbers allowed");
                continue;
            }
        };

        match user_guessed_value.cmp(&random_number) {
            Ordering::Less => println!(" ⬇️ Too less!"),
            Ordering::Greater => println!(" ⬆️ Too big!"),
            Ordering::Equal => {
                println!("hurray! 🦄 that was a match 🌟");
                break;
            }
        }
    }
    // println!("You guessed :{user_guessed_value}, Actual :{random_number}")
}
