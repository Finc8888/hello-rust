use std::str::FromStr;

fn main() {
    println!("=== Examples of working with parse::<T>() ===\n");

    // Base example of number parsing
    basic_parsing_examples();

    // Different cases of type specification
    type_specification_examples();

    // Error handling
    error_handling_examples();

    // Parsing other types
    other_types_examples();

    // Comparison of approaches
    comparison_examples();
}

fn basic_parsing_examples() {
    println!("1. Basic parsing examples:");

    // Парсинг в разные числовые типы
    let text = "42";

    let as_i32 = text.parse::<i32>().unwrap();
    let as_u32 = text.parse::<u32>().unwrap();
    let as_f64 = text.parse::<f64>().unwrap();
    let as_i64 = text.parse::<i64>().unwrap();

    println!("  '{}' as i32: {}", text, as_i32);
    println!("  '{}' as u32: {}", text, as_u32);
    println!("  '{}' as f64: {}", text, as_f64);
    println!("  '{}' as i64: {}", text, as_i64);
    println!();
}

fn type_specification_examples() {
    println!("2. Ways to specify types:");

    let text = "123";

    // Method 1: Turbofish ::<>
    let num1 = text.parse::<i32>().unwrap();
    println!("  Turbofish: {}", num1);

    // Method 2: Variable type annotation
    let num2: i32 = text.parse().unwrap();
    println!("  Variable annotation: {}", num2);

    // Method 3: Context inference
    let num3: i32 = text.parse().unwrap(); // Type specified explicitly
    let doubled: i32 = num3 * 2; // Compiler infers type from usage
    println!("  Context inference: {}", doubled / 2);
    println!();
}

fn error_handling_examples() {
    println!("3. Error handling:");

    let valid_number = "456";
    let invalid_number = "abc";
    let empty_string = "";
    let overflow_number = "999999999999999999999999999999";

    // Successful parsing
    match valid_number.parse::<i32>() {
        Ok(num) => println!("  '{}' successfully parsed: {}", valid_number, num),
        Err(e) => println!("  Parse error for '{}': {}", valid_number, e),
    }

    // Invalid format
    match invalid_number.parse::<i32>() {
        Ok(num) => println!("  '{}' successfully parsed: {}", invalid_number, num),
        Err(e) => println!("  Parse error for '{}': {}", invalid_number, e),
    }

    // Empty string
    match empty_string.parse::<i32>() {
        Ok(num) => println!("  '{}' successfully parsed: {}", empty_string, num),
        Err(e) => println!("  Parse error for empty string: {}", e),
    }

    // Overflow for i32
    match overflow_number.parse::<i32>() {
        Ok(num) => println!("  '{}' successfully parsed: {}", overflow_number, num),
        Err(e) => println!("  Overflow error: {}", e),
    }
    println!();
}

fn other_types_examples() {
    println!("4. Parsing other types:");

    // Boolean
    let bool_true = "true".parse::<bool>().unwrap();
    let bool_false = "false".parse::<bool>().unwrap();
    println!("  Boolean: {} and {}", bool_true, bool_false);

    // Character (only one character)
    let character = "A".parse::<char>().unwrap();
    println!("  Character: {}", character);

    // Floating point with decimal part
    let float_num = "3.14159".parse::<f64>().unwrap();
    println!("  Float: {}", float_num);

    // Negative numbers
    let negative = "-42".parse::<i32>().unwrap();
    println!("  Negative number: {}", negative);

    // Numbers with spaces (trim needed)
    let with_spaces = "  789  ";
    let trimmed_num = with_spaces.trim().parse::<i32>().unwrap();
    println!("  With spaces after trim: {}", trimmed_num);
    println!();
}

fn comparison_examples() {
    println!("5. Comparison of approaches:");

    let input = "100";

    // unwrap() - panic on error
    println!("  unwrap(): {}", input.parse::<i32>().unwrap());

    // expect() - custom message on panic
    println!(
        "  expect(): {}",
        input.parse::<i32>().expect("Failed to parse number")
    );

    // unwrap_or() - default value on error
    let bad_input = "xyz";
    println!("  unwrap_or(): {}", bad_input.parse::<i32>().unwrap_or(0));

    // unwrap_or_else() - function to generate default value
    println!(
        "  unwrap_or_else(): {}",
        bad_input.parse::<i32>().unwrap_or_else(|_| -1)
    );

    // match for full control
    match input.parse::<i32>() {
        Ok(num) => println!("  match Ok: {}", num),
        Err(e) => println!("  match Err: {}", e),
    }

    // if let for brevity
    if let Ok(num) = input.parse::<i32>() {
        println!("  if let: {}", num);
    }
    println!();
}

// Additional function to demonstrate generics
fn parse_to_type<T: FromStr>(input: &str) -> Result<T, T::Err> {
    input.parse::<T>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_parsing() {
        assert_eq!("42".parse::<i32>().unwrap(), 42);
        assert_eq!("3.14".parse::<f64>().unwrap(), 3.14);
        assert_eq!("true".parse::<bool>().unwrap(), true);
    }

    #[test]
    fn test_error_cases() {
        assert!("abc".parse::<i32>().is_err());
        assert!("".parse::<i32>().is_err());
        assert!("999999999999999999999999999999".parse::<i32>().is_err());
    }
}
