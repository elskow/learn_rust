use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let randint = rand::thread_rng().gen_range(1..101);
    println!("Guess The Number!");

    loop{
        println!("Please input your guess :");
        let mut num_guess = String::new();
        io::stdin()
            .read_line(&mut num_guess)
            .expect("Failed to read line");

        println!("You guessed : {}", num_guess); 

        let num_guess: u32 = match num_guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match num_guess.cmp(&randint){
            Ordering::Less => println!("Too small!\n"),
            Ordering::Greater => println!("Too big!\n"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}