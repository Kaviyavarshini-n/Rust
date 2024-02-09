fn is_palindrome(input: &str) -> bool {
    let input = input.to_lowercase();
    let reversed = input.chars().rev().collect::<String>();

    input == reversed
}

fn main() {
    let input1 = "radar";
    let input2 = "hello";

    println!("Is '{}' a palindrome? {}", input1, is_palindrome(input1));
    println!("Is '{}' a palindrome? {}", input2, is_palindrome(input2));
}
