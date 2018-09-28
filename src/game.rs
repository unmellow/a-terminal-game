extern crate rand;

use std::io;
use std::cmp::Ordering;
use self::rand::Rng;

     pub fn start_game(difficulty: u32){
       let secret_number = rand::thread_rng().gen_range(1, difficulty+1);
       loop{ 
           println!("guess a number between 1 and {}.",difficulty);

           let mut guess = String::new();
    
           io::stdin().read_line(&mut guess)
              .expect("Failed to read line WHAT DID YOU DO");

           let guess: u32 = match guess.trim().parse(){
               Ok(num) => num,
               Err(_) => continue,
           };

               println!("You guessed: {}", guess);

               match guess.cmp(&secret_number) {
                   Ordering::Less => println!("{} is To small",guess),
                   Ordering::Greater => println!("{} is too big",guess),
                   Ordering::Equal => {
                       println!("YOU DID IT THAT'S JUST!! RIGHT");
                       break;
                   }
               }
       }
   }

