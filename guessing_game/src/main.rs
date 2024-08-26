use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Game:: Guess The Number:");
    let mut chance = String::new();
    print!("Chance = ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut chance)
        .expect("failed to read line");
    let chance:u8 = chance.trim().parse().expect("Please type a number"); 

    let mut range = String::new();
    print!("Range 1 t0 ");
    io::stdout().flush().unwrap();
    io::stdin()
        .read_line(&mut range)
        .expect("failed to read line");
    let int_range:u32 = range.trim().parse().expect("please type a number");

    println!("<----------------------------------------------->");


    let secret_number = rand::thread_rng().gen_range(1..=int_range);
    // println!("{secret_number}");
    for i in 1..=chance {
        print!("Input Your Guessing Number = ");
        io::stdout().flush().unwrap();
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        let guess: u32 = guess.trim().parse().expect("Please type a number");
        println!("Left Chance = {}",chance-i);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You Guessed = {guess} 😊");
                println!("Congratulations, You win😊😊! ");
                break;
            }
        }

        if i == chance {
            println!("---------------------------------------------");
            println!("You Failed🥴😂😂😂")
        }
        println!("---------------------------------------------");
    }
}
