use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    

    let secret_number = rand::thread_rng().gen_range(1..=100);
    

    loop {
        println!("Please enter your guess");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the input");
        let mut guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
            
        };

        println!("You have guessed {}", guess);

        match guess.cmp(&secret_number) {

            Ordering::Less => println!("The number is too small"),
            Ordering::Greater => println!("The number is too big"),
            Ordering::Equal => {
                println!("BINGOOOO!!! "); 
                break;
            }
        }
    }
}
