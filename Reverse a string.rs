fn reverse_string(s: &str) -> String {
    s.chars().rev().collect::<String>()
}

fn main() {
    let s = "Hello, Welcome!";
    let reversed = reverse_string(s);
    println!("Reversed string: {}", reversed);
}