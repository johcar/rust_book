extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    
    let secret = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess." );
        
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        println!("Your guess: {}", guess);   
        let guess: u32 =  match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, 
        };

        match guess.cmp(&secret) {
            Ordering::Less => println!("Guess was less then secret"),
            Ordering::Equal =>  {
                println!("Guess was correct");
                break;
            }
            Ordering::Greater => println!("Guess was greater then secret"),
        }
    }

}
