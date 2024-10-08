use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("Hello, world!");
    loop {
        println!("Guess the number!");
    let mut  secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);
    let mut guess =  String::new();

io::stdin().read_line(&mut guess)
        .expect("Failed to read line");  
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

       match guess.cmp(&secret_number){
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
            
        }

    }

}
