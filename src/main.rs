use rand::Rng;

fn main() {
    println!("Welcome to Bulls and Cows");
    let secret_number = rand::thread_rng().gen_range(1..11);

    let mut attempts = 0;
}
