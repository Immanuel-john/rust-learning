use std::io;
use rand::Rng;
fn main() {
    let secret_number= rand::thread_rng().gen_range(1..101);

    println!("Secret number is  {} ",secret_number);
    println!("Input your guess");

    let mut guess = String::new();


    io::stdin()
        .read_line(&mut guess)
        .expect("Expected your guessing input");

    println!("You guessed {}", guess);
}
