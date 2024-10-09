fn sum_with_step(total: &mut i32, low: i32, high: i32, step: i32) {
    
    // we will be adding both ends of the ranges anyway, i find it easier to just add them now
    *total += low + high;
    
    //we start at our first step
    let mut current = low + step;
    
    
    loop {
        // if we ever equal or go beyond we stop
        if current >= high {
            break;
        }
        
        *total += current;
        current += step;
    }
}

fn main() {
    let mut result = 0;
    sum_with_step(&mut result, 0, 100, 1);
    println!("Sum 0 to 100, step 1: {}", result);

    result = 0;
    sum_with_step(&mut result, 0, 10, 2);
    println!("Sum 0 to 10, step 2: {}", result);

    result = 0;
    sum_with_step(&mut result, 5, 15, 3);
    println!("Sum 5 to 15, step 3: {}", result);
}