use rand::prelude::*;

fn main() {
    println!("Hi! In this Game you have to guess a number between 0 and 5, you only have 3 chance to guess it.");
    
    let mut rng = thread_rng();
    let random_number = rng.gen_range(0..=5);

    let mut attempt = 0;

    loop {
        println!("Write a number: ");
        let mut number : String = String::new();
        std::io::stdin().read_line(&mut number).unwrap();
        let int_number : u8 = number.trim().parse().unwrap();

        if random_number == int_number {
            println!("Well Done! the secret number is {}.", random_number);
            break;
        }
        if attempt == 3 {
            println!("Ops! GAME OVER! the secret number is {}. Try Again.", random_number);
            break;
        }
        attempt = attempt + 1;
    }
}
