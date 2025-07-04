use rand::Rng;
use std::cmp::Ordering;
use std::io;
fn main() {
    println!("GUESS A NUMBER: ");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    loop {
        println!("PLEASE INPUT YOUR GUESS: ");

        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("PLEASE ENTER VALID INPUT");
                continue;
            }
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("SMALL"),
            Ordering::Equal => {
                println!("YOU WON");
                break;
            }
            Ordering::Greater => println!("BIG"),
        }
    }
}
