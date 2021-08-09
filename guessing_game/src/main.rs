use std::io;


fn main() {
    println!("Welcome to the game");
    println!("Input guess :");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess);

    println!("You guessed {}",guess);
}