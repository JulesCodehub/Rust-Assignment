fn main() {
    println!("Hello, world!");
}
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

        // set the guess variable
        
        // call our check_guess function
        
        // print how our guess was
        
        // if correct, break

    // print our result
    
    println!("Game Finished, took: {} attempts.", count);

}