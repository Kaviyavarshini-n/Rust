fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return String::new();
    }

    let first_string = &strings[0];
    let mut longest_prefix = String::new();

    'outer: for (index, char) in first_string.chars().enumerate() {
        for string in &strings[1..] {
            if let Some(c) = string.chars().nth(index) {
                if c != char {
                    break 'outer;
                }
            } else {
                break 'outer;
            }
        }
        longest_prefix.push(char);
    }

    longest_prefix
}

fn main() {
    let strings = vec![
        String::from("flower"),
        String::from("flow"),
        String::from("flight"),
    ];
    println!("Longest Common Prefix: {}", longest_common_prefix(&strings));

    let strings2 = vec![
        String::from("dog"),
        String::from("racecar"),
        String::from("car"),
    ];
    println!("Longest Common Prefix: {}", longest_common_prefix(&strings2));
}
