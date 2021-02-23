use rand::Rng;
use std::cmp::Ordering;
use std::io;
const MIN: u32 = 1;
const MAX: u32 = 1000;

pub fn guess() {
    println!("Guess a number!");
    let secret = rand::thread_rng().gen_range(MIN, MAX);

    loop {
        println!("Number? ");
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Could not read number from standard input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Your guess: {}", guess);
        match guess.cmp(&secret) {
            Ordering::Less => println!("Number is too small"),
            Ordering::Greater => println!("Number is too large"),
            Ordering::Equal => {
                println!("Won!");
                break;
            }
        }
    }
}
