use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The Number!");

    println!("please input the number between 1 and 100: ");
    let mut guess = String::new();
    let n1 = rand::thread_rng().gen_range(1..101);

    loop {
        guess.clear();
        io::stdin().read_line(&mut guess).expect("failed to read");
        let guessed: u32 = match guess.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a number.");
                continue;
            }
        };

        match n1.cmp(&guessed) {
            Ordering::Less => println!("lower!"),
            Ordering::Greater => println!("higher"),
            Ordering::Equal => {
                println!("great, you did it :D");
                break;
            }
        }
    }
}
