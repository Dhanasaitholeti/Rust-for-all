use std::io;

fn main(){

    println!("guess the number");

    println!("please enter your number");


    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("Failed to read the line");


    println!("you guessed the number {}",guess);

}