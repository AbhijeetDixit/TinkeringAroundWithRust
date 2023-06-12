use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the Number Game");
    println!("Enter your guess");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failure to read");

        println!("You guessed {guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too less"),
            Ordering::Greater => println!("Too much"),
            Ordering::Equal => {
                println!("Correct!");
                break;
            }
        }
    }
}
