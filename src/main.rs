use std::{ cmp::Ordering, io};
use rand::Rng;
use colored::{self, Colorize};
 
fn main() {
    // println!("Hello, world!");
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("{} {}","The secret number is:".yellow(),secret_number.to_string().white());
    
    
    loop {
        let mut guess: String = String::new();
 
        println!("{}","-".bright_cyan().repeat(100));
        println!("{}","Please input your guess:".yellow());
        io::stdin()
        .read_line(&mut guess)
        .expect("Could not read line");
    
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("{}","Entered input is not a number".bright_red());
                continue;
            },
        }; //.expect("Please enter a number!");
    
        println!("You guessed: {}",guess);
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Number guessed is smaller!".red()),
            Ordering::Greater => println!("{}","Number guessed is greater!".red()),
            Ordering::Equal => {
                println!("{}","You win".green());
                break;
            },
        };
    }
 
}