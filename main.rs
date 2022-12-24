use std::io;
use std::cmp::Ordering;
use rand::Rng;


fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);


    loop{

    println!("Please enter your guess.");

    let mut guess = String::new();


    io::stdin()
    .read_line(&mut guess)
    .expect("Error to read input");


    let guess: u32 = match guess.trim().parse(){
        Ok(num) => num,
        Err(_) => continue,
    };

    

    println!("Your guessed number is {}.", guess);
    println!("Random Number is {}", secret_number);
    


    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Your Guess {} is less than {} random number.", guess, secret_number),
        Ordering::Greater => println!("Your Guess {} is Higher than {} random number.", guess, secret_number),
        Ordering::Equal => {println!("Your Guess {} matches random number {} .", guess, secret_number);
                                break;}
        }
    }
}


