use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    

    println!("Please enter the number: ");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("Secret number is {}", secret_number);
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("U  haved guessed {}", guess);
    match guess.cmp(&secret_number){
        Ordering::Less => println!("Guess is too small");
        Ordering::Greater => println!("Guess is too big");
        Ordering::Equal => println!("You win");


}
