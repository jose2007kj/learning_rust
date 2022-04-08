use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("wlecome to guessing game");
    loop {
        println!("Enter a number:");
        let mut guess = String::new();
        let secret_number = rand::thread_rng().gen_range(1, 101);
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read input");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("lesser number you entered"),
            Ordering::Greater => println!(" too large"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
        println!(
            "you have entered {} and secret number is {}",
            guess, secret_number
        );
    }
}
