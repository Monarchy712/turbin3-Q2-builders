use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;

fn main() {
    
    let secret_number = rand::rng().random_range(1..=100);
    //println!("The random number is {}", secret_number);
    let mut counter: u32 = 0;
    loop {
        let mut guess = String::new();
        println!("Enter your number : ");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too small!!".red()),
            Ordering::Greater => println!("{}","Too big!!".red()),
            Ordering::Equal => {
                println!("You took {} guesses !", counter);
                println!("{}","You win!".green());
                break;
            }
        };
        counter+=1;
    }
}
