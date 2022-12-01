use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Welcome to Hell Bitch!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {

        println!("Guess a number, you r-word");

        let mut guess = String::new();
  
        io::stdin()
            .read_line(&mut guess)
            .expect("You absolute moron");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Who would guess {guess} Idiot");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("That number is too small, like your dick!"),
            Ordering::Greater => println!("That number is too big, like your forehead!"),
            Ordering::Equal => {
                println!("Oh wait, I guess that number is right...");
                println!("I suppose you win, this time");
                break;
            }
        }
    }
}
