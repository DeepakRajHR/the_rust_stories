use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("This is a guessing game. I will choose a number in [1..100) and you guess it.");
    let rand_number = rand::thread_rng().gen_range(1..=100);

    loop {
        // Get input from the user as string
        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read the input");

        // Convert str input to unsigned input
        let user_input: u32 = match user_input.trim().parse() {
            Ok(number) => number,
            Err(_) => {
                println!("Not a number! Try again");
                continue;
            }
        };

        // Compare user input with choosen number
        match user_input.cmp(&rand_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too large!"),
            Ordering::Equal => {
                println!("Bingo! You win.");
                break;
            }
        }
    }

    println!("I choose {rand_number}");
}

// EOF
