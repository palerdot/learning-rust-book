use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // println!("porumai ... secret number {} ", secret_number);
    loop {
        println!("porumai ... wait and hope ... guess a number ?");

        let mut guess = String::new();
        // get the number from user
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to get input");

        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("porumai ... number please !!! \n");
                continue;
            }
        };

        println!("porumai ... your guess: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("porumai ... small"),
            Ordering::Greater => println!("porumai ... big"),
            Ordering::Equal => {
                println!("porumai ... correct guess !!!");
                // exit the loop
                break;
            }
        }
    }
}
