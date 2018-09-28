use std::io;

mod game;

fn main() {

   loop{

       let mut x = String::new();
    
       println!("chose diffiuculty enter number");


    
           io::stdin().read_line(&mut x)
              .expect("Failed to read line WHAT DID YOU DO");

           let x: u32 = match x.trim().parse(){
               Ok(num) => num,
               Err(_) => continue,
           };

           if x > 1{
               println!("Guess the number!");

               game::start_game(x);

               break
           } else{
               continue
           }

}   
} 
