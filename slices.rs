fn main() {
    let mut word = String::from("Hello world");
    let word2 = find_first_word(&word);  
    println!("{}", word);
    println!("{}", word2)
}

//Till now nothing special 
fn find_first_word(word: &String) -> &str {
    let mut index = 0;
    for (_, i) in word.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }

    return &word[0..index];
}