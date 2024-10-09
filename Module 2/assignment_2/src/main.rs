fn most_frequent_word(text: &str) -> (String, usize) {

    let words: Vec<&str> = text.split_whitespace().collect();
    
    //Rust does not allow arrays to have sized not defined at compile time
    //The next simpliest data structure we can use is a vector, but it will be used exactly like an array
    let mut count = vec![0 as i32; words.len()];
    
    for idx in 0..words.len(){
        
        // -1 will indicate that we have already looked at this word
        if count[idx] != -1 {
            
            // we increment the count after having encountered this word
            count[idx] += 1;
            
            // we compare this word with the remaining words in the list looking for any duplicates
            for idx2 in idx+1..words.len(){
            
                // when we encounter a duplicate
                if words[idx] == words[idx2]{
                
                    //we increment our count
                    count[idx] += 1;
                
                    // and mark that spot with -1
                    count[idx2] = -1;
                }
            }
        }
    }
    
    // we then iterate through our counts, looking for the highest
    let mut counter = 0;
    let mut idx = 0;
    for idx2 in 0..words.len(){
        if count[idx2] > counter {
            counter = count[idx2];
            idx = idx2;
        }
    }
    
    let max_count = counter as usize;
    let max_word = words[idx].to_string();
    
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}