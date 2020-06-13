use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!!");

    let secret_num = rand::thread_rng().gen_range(1, 101);

    println!("The secret numb is: {}", secret_num);
    loop {
        println!("Input ur guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!(">> Input only accept number!");
                continue;
            }
        };
        println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!(">> Too small!"),
            Ordering::Greater => println!(">> Too big!"),
            Ordering::Equal => {
                println!(">> Nyou win!");
                break;
            }
        }
    }
}
