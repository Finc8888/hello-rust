use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("Secret number is {}", secret_num);
    loop {
        println!("Type your guessing:");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("String can't be read");

        let guess: i32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please type only numbers");
                continue;
            }
        };
        println!("You typed {}", guess);
        match &guess.cmp(&secret_num) {
            Ordering::Greater => println!("Your number is too big"),
            Ordering::Less => println!("Your number is too less"),
            Ordering::Equal => {
                println!("Congratulation!!! You guess number!");
                break;
            }
        }
    }
}
