use std::io;

fn shortest_word(sentence: &str) -> Option<&str> {
    sentence.split_whitespace().min_by_key(|word| word.len())
}

fn main() {
    println!("Enter a string of words:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input_string = input.trim();

    if let Some(shortest) = shortest_word(input_string) {
        println!("The shortest word is: {}", shortest);
    } else {
        println!("No words found in the input string");
    }
}