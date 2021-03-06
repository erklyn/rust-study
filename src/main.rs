use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Please input your guess.");
    loop {
        
        let mut guess = String::new();
        
        io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
        
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input a number, try again!");
                continue;
            }
        };
        
        println!("You guessed: {}" , guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small , try again"),
            Ordering::Greater => println!("Too big , try again"),
            Ordering::Equal => {
                println!("You win");
                break;
            }
        }
    }
}