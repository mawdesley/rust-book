use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let n = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("Guess a number, punk...");

        
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");
        
        match guess.cmp(&n) {
            Ordering::Less => println!("too smol"),
            Ordering::Greater => println!("too chonki"),
            Ordering::Equal => {
                println!("jus right");
                break;
            }
        }
    }
}
