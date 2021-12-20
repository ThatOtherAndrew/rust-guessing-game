use std::cmp::Ordering;
use std::io;
use std::process::exit;
use rand::Rng;


fn main() {
    let answer = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Your guess:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        let message = guess.trim().to_lowercase();
        if message == "exit" || message == "quit" {
            println!("Exiting program");
            exit(0);
        }

        let guess: u32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Invalid guess!");
                continue;
            }
        };

        println!("You guessed {}...", guess);

        match guess.cmp(&answer) {
            Ordering::Less => println!("but it was too small!"),
            Ordering::Greater => println!("but it was too big!"),
            Ordering::Equal => {
                println!("and you were correct!");
                break;
            }
        }
    }
}
