use colored::Colorize;
use rand::Rng;
use std::{cmp::Ordering, io};
fn main() {
    println!("Please Enter the inputs");

    let target: u32 = rand::thread_rng().gen_range(0..101);
    println!("{}", target);

    loop {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read input");

        println!("{} {}", "you guess".blue(), guess);
        let guess: u32 = guess
            .trim()
            .parse()
            .expect("Invalid number, please enter again");

        match guess.cmp(&target) {
            Ordering::Less => println!("{}", "Too Small".red()),
            Ordering::Equal => {
                println!("{}", "Right".green());
                break;
            }
            Ordering::Greater => println!("{}", "Too Big".red()),
        }
    }
}
