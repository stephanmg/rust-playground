// make guess module's function play available
mod guess;

fn main() {
    // Say hello
    println!("Hello, world!");

    // Play guessing game
    guess::start(String::from("Console"));
}
