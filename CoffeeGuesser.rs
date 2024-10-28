use std::io;

fn main() {
    let jim = "French Vanilla";
    let mut guess = String::new();
    
    println!("Input Jim's favorite flavor:");

    loop {
        guess.clear();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let trimmed_guess = guess.trim();
        println!("You guessed: {}", trimmed_guess);

        if trimmed_guess == jim {
            println!("You guessed right!");
            break; 
        } else if trimmed_guess == "cheese" {
            println!("You're right, but that's not coffee.");
            println!("Try guessing the correct coffee.");
        } else {
            println!("You guessed wrong.");
            println!("Try again.");
        }
    }
}
