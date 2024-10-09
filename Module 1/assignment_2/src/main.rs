fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    
    //10 random numbers
    let my_ten_numbers: [i32; 10] = [ 16, 21, 24, 23, 16, 13, 10, 18, 2, 10 ];
    
    //for idx in 0..my_ten_numbers.len() {
    //    println!("{}", is_even(my_ten_numbers[idx]));
    //}
    
    // fizz buzz loop
    
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