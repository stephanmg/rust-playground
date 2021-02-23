// make guess module's function play available
mod guess;
use crate::guess::game::play;

fn main() {
    // Say hello
    println!("Hello, world!");

    // Play guessing game
    play();
}
