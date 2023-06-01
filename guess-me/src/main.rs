use colored::*;
use rand::Rng;
use std::cmp;
use std::io;

fn main() {
    println!("Guess the Number!!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    loop {
        println!("Please Input Your Number?");
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readline");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            cmp::Ordering::Less => println!("{}", "go bigger".red()),
            cmp::Ordering::Greater => println!("{}", "go smaller".red()),
            cmp::Ordering::Equal => {
                println!("{}", "You won".green());
                break;
            }
        }
    }
}
