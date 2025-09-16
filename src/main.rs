use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Bulls and Cows");
    let secret_number = rand::thread_rng().gen_range(1..11);

    let mut attempts = 0;
    loop {
        println!("Please input a number: ");
        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Oops! Something goes wrong");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please input valid number");
                continue;
            }
        };
        attempts += 1;
    }
}
