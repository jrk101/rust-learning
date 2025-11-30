use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let rand_num = rand::thread_rng().gen_range(1..=100);
    println!("Guess a number and enter it");
    let mut guess= String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read");
    println!("You guessed {guess}");
    let guess: u32 = guess.trim().parse().expect("Enter a number");
    match guess.cmp(&rand_num){
        Ordering::Less => println!("Less"),
        Ordering::Equal => println!("You win"),
        Ordering::Greater => println!("Bigger"),
    }
    
}