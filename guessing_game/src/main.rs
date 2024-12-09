use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // creates a new RNG thread and generates a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess:");

        // we define guess as a mutable variable, rust is type strict so this will be a growable string
        let mut guess = String::new();

        // read the user input and save it into guess
        // in the event of an exception prints "Failed to read line" and crashes
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Convert the guess from string to unsigned int 32 bit
        // ignore any errors due to non-numeric inputs
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // This method performs a comparison between the two values and returns an ordering,
        // which can be one of three variants: Ordering::Less, Ordering::Greater, or Ordering::Equal.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win! The random number was indeed {}", secret_number);
                break;
            }
        }
    }
}
