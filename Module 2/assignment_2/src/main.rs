fn most_frequent_word(text: &str) -> (String, usize) {
    let max_word = "a".to_string();
    let max_count = 0;
    (max_word, max_count) // return tuple
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog the quick brown fox";
    let (word, count) = most_frequent_word(text);
    println!("Most frequent word: \"{}\" ({} times)", word, count);
}