use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let min = 1 ;
    let max = 100;

    let secret_nb = rand::thread_rng().gen_range(min..=max);

    println!("Input your guess :");
    let mut guess = String::new();
     std::io::stdin()
        .read_line(&mut guess)
        .expect("[ERROR] Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    
    println!("You guessed : {guess}");
    
    match guess.cmp(&secret_nb) {
        Ordering::Less => println!("too small!"),
        Ordering::Greater => println!("too BIG!"),
        Ordering::Equal => println!("YOU WIN !"),
    }
}