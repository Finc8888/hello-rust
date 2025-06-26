fn main() {
    let original = "Twelve drummers drumming";
    let result = first_char_to_lowercase(original);

    println!("Original: {}", original);
    println!("Result: {}", result);

    // Test with other examples
    let examples = vec![
        "Hello World",
        "RUST Programming",
        "a already lowercase",
        "",
        "Ü Unicode test",
        "123 numbers first",
    ];

    println!("\nTesting with various examples:");
    for example in examples {
        println!("'{}' -> '{}'", example, first_char_to_lowercase(example));
    }
}

fn first_char_to_lowercase(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    let mut chars = s.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_lowercase().collect::<String>() + chars.as_str(),
    }
}

// Alternative implementation using split_at
fn first_char_to_lowercase_v2(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }

    // Find the byte boundary of the first character
    let mut indices = s.char_indices();
    match indices.next() {
        None => String::new(),
        Some((_, first_char)) => {
            let first_char_end = indices.next().map(|(i, _)| i).unwrap_or(s.len());
            let (first, rest) = s.split_at(first_char_end);
            format!("{}{}", first.to_lowercase(), rest)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_conversion() {
        assert_eq!(first_char_to_lowercase("Hello"), "hello");
        assert_eq!(first_char_to_lowercase("WORLD"), "wORLD");
    }

    #[test]
    fn test_edge_cases() {
        assert_eq!(first_char_to_lowercase(""), "");
        assert_eq!(first_char_to_lowercase("a"), "a");
        assert_eq!(first_char_to_lowercase("A"), "a");
    }

    #[test]
    fn test_unicode() {
        assert_eq!(first_char_to_lowercase("Ü test"), "ü test");
        assert_eq!(first_char_to_lowercase("Ñoël"), "ñoël");
    }

    #[test]
    fn test_numbers_and_symbols() {
        assert_eq!(first_char_to_lowercase("123abc"), "123abc");
        assert_eq!(first_char_to_lowercase("!Hello"), "!Hello");
    }
}
