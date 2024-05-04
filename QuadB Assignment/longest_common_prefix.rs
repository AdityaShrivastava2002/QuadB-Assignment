use std::io;

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let mut prefix = strings[0].clone();
    
    for string in strings.iter().skip(1) {
        let mut i = 0;
        while i < prefix.len() && i < string.len() && prefix.as_bytes()[i] == string.as_bytes()[i] {
            i += 1;
        }
        prefix.truncate(i);
    }

    prefix
}

fn main() {
    println!("Enter a set of strings separated by spaces:");
    let mut input = String::new();

    io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input_strings: Vec<String> = input.split_whitespace()
        .map(|s| s.to_string())
        .collect();

    let common_prefix = longest_common_prefix(&input_strings);
    println!("The longest common prefix is: {}", common_prefix);
}