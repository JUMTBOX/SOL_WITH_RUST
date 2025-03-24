use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main () {
    println!("guess the number");

    let secret_num = rand::rng().random_range(1..=100);

    // println!("the secret number is: {secret_num}");
    
    loop {
        println!("input your guess");

        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to readLine");
        
        // match 키워드가 (java, js)의 switch 키워드 같은 역할을 하는 듯
        let guess:u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
    
        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("you win!");
                break;
            },
        }
    
        println!("you guessed!: {guess}");   
    }
}