fn check_guess(guess: i32, secret: i32) -> i32 {
    
    if guess == secret {
        return 0
    }
    
    if guess > secret {
        return 1
    }
    
    -1
}

fn main() {
    
    let mut secret = 32;
    let mut count = 0;
    let mut guess = 0;
    
    // main loop
    loop {
    
        //increment our attempt count
        count += 1;
        
        // set the guess variable
        guess = (rand::random::<f64>() * 50.0) as i32;
        
        // call our check_guess function
        let var = check_guess(guess, secret);
        
        // print how our guess was
        match var {
            
            1 => println!("Incorrect Guess: {}, Too high", guess),
            -1 => println!("Incorrect Guess: {}, Too low", guess),
            0 => {
                println!("Correct Guess: {}", guess);
                // if correct, game is over
                break
            },
            _ => {
                println!("Error, this statement should not have been reached");
                break
            }
        }

    }

    // print our result
    println!("Game Finished, took: {} attempts.", count);

}