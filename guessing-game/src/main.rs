use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // generate a random number between 1 and 100
    let secret_number = rand::thread_rng().gen_range(1..101);

   //println!("The secret number is: {}", secret_number);

    // loop forever
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        // io::stdin() is an instance of std::io::Stdin

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // convert the string to a number
        // trim() removes whitespace from the string
        // parse() converts the string to a number
        // expect() is like println!() but for errors

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        // match is like a switch statement in other languages
        // it compares the value of guess to the value of secret_number
        // and returns an Ordering enum
        // Ordering is an enum that has three variants: Less, Greater, and Equal

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
