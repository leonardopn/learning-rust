use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn main() {
    loop {
        guessing_game();
        println!("Do you want to play again? (y/n)");

        let mut play_again = String::new();

        io::stdin()
            .read_line(&mut play_again)
            .expect("Failed to read line");

        play_again = play_again.trim().to_string();

        if play_again != "y" {
            break;
        }
    }
}
