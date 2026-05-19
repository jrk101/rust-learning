use std::io;
use rand::Rng;
fn main() {
    println!("Enter a number:");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read line");
    println!("You entered: {}", guess);
    let secret_number= rand::thread_rng().gen_range(1..=100);
    
}
