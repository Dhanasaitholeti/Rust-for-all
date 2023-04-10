use std::io;

fn main(){
    println!("Guessing game");

    println!("enter the number");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read line");

    println!("you guessed the number {}",guess);
}