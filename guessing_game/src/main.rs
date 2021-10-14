use rand::Rng;
use std::io;
use std::process;


fn main() {
    println!("Guess The Number!");
    

    loop {
        println!("please input the number between 0 and 10: ");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess).expect("failed to read");
        let guessed = guess
            .trim()
            .parse::<u32>()
            .expect("error, should type a number.");

        let n1 = rand::thread_rng().gen_range(0..10);

        if guessed == n1 {
            println!("you won, the secret number was {} and you got it right", n1);
        } else {
            println!("you lost, the secret number was {}", n1);
        }

        println!("do you want to play again? (Y/N)");
        
        
    }
}
