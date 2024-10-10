fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    
    //10 random numbers
    let my_ten_numbers: [i32; 10] = [ 16, 21, 24, 23, 16, 13, 10, 15, 2, 10 ];
    
    println!("{:?}", my_ten_numbers);

    // fizz buzz loop
    for idx in 0..my_ten_numbers.len() {
        let var = my_ten_numbers[idx];
        if var % 3 == 0 && var % 5 == 0 {
            println!("FizzBuzz");
        } else if var % 3 == 0 {
            println!("Fizz");
        } else if var % 5 == 0 {
            println!("Buzz");
        } else if is_even(var) {
            println!("Even");
        } else {
            println!("Odd");
        }
        
    }
    
    // summation loop
    let mut sum = 0;
    for idx in 0..my_ten_numbers.len() {
        sum += my_ten_numbers[idx];
    }
    println!("Total Sum is: {}", sum);
    
    
    // largest number loop
    let mut largest = my_ten_numbers[0];
    for idx in 1..my_ten_numbers.len() {
        if my_ten_numbers[idx] > largest {
            largest = my_ten_numbers[idx];
        }
    }
    println!("Largest number is: {}", largest);
    
}