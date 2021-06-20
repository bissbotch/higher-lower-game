use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    
    run();
}

fn run() {
    println!("| *** Higher Lower *** |\n");

    let mut index = 0;

    while index < 1 {
        let secret_number = rand::thread_rng().gen_range(1..101);
        println!("Please type your guess");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Higher"),
        Ordering::Greater => println!("Lower"),
        Ordering::Equal => {
            println!("You win!");
            let mut answer = String::new();

            println!("Do you want to play again? y/n \n");
            io::stdin()
                .read_line(&mut answer)
                .expect("Failed to read line");

            if answer == "y" || answer == "Y" {
                continue;
            }

            else {
                index = 1;
            }
            }
        }
    }
}
