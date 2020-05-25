use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = gen_num();
    println!("Guess the number!");
    let mut tries = 0;
    loop {
        let guess = get_guess();
        tries += 1;
        compare_guess(guess, secret_number);
        if guess == secret_number {
            congratulate(tries);
            break;
        }
    }
}

fn get_guess() -> i32{
    println!("Input your guess: ");
    let mut guess  = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    
    let guess = guess.trim().parse()
        .expect("Please enter a number..");
    println!("You guessed: {}", guess);
    return guess
}

fn gen_num() -> i32{
    let secret_number = rand::thread_rng().gen_range(1, 101);
    return secret_number
}

fn compare_guess(guess: i32, secret_number: i32){
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too Small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}

fn congratulate(tries: i32){
    println!("Congratulations!");
    println!("It only took you {} tries!", tries);
}
