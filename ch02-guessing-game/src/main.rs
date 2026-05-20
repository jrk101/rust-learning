use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main(){
    println!("Enter a number:");
    let mut guess=String::new();
    io::stdin().read_line(&mut guess).expect("Failed to read input");
    let random_number=rand::thread_rng().gen_range(1..=100);
    println!("The random number is: {}", random_number);
    let guess:u32=guess.trim().parse().expect("Enter a number");
    match guess.cmp(&random_number){
        Ordering::Less => println!("Very less!"),
        Ordering::Greater => println!("Very Big!"),
        Ordering::Equal => println!("You Win!"),
    }
}