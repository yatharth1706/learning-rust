use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Guessing game!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Please input your guess.");

        let mut input = String::new();    

        // --snip--
        io::stdin().read_line(&mut input).expect("Failed to read input");

        let input: i32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        match input.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!"); 
                break;
            }
        }
    }
    
}
