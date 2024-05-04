fn is_palindrome(s: &str) -> bool {
    let chars = s.chars().collect::<Vec<_>>(); // Collect chars into a vector
    let mut left = 0;
    let mut right = chars.len() - 1;

    while left < right {
        if chars[left] != chars[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

fn main() {
    println!("Enter a string to check if it's a palindrome:");
    let mut input = String::new();

    std::io::stdin().read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim(); // Remove newline character

    println!("Is '{}' a palindrome? {}", input, is_palindrome(input));
}
