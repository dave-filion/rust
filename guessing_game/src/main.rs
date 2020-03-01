use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number");
    let secret_num = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input ur guess");
        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("thats not a number...");
                continue;
            },
        };

        println!("you guessed {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small!"),
            Ordering::Greater => println!("too large!"),
            Ordering::Equal => {
                println!("Just got it!");
                break;
            },
        }
    }

}
