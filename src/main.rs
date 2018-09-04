extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop{
        println!("Please enter your guess.");

        let mut guess = String::new();
    
        io::stdin().read_line(&mut guess)
           .expect("Failed to read line WHAT DID YOU DO");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
            println!("You guessed: {}", guess);

            match guess.cmp(&secret_number) {
                Ordering::Less => println!("To small You Noob!"),
                Ordering::Greater => println!("Too Big! You dum dum"),
                Ordering::Equal => {
                    println!("YOU DID IT THAT'S JUST!! RIGHT");
                    break;
                }
            }
    }
}
