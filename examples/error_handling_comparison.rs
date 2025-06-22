use std::num::ParseIntError;

fn main() {
    println!("=== Error Handling Methods Comparison ===\n");

    // Demonstrate different error handling approaches
    demonstrate_unwrap();
    demonstrate_expect();
    demonstrate_unwrap_or();
    demonstrate_unwrap_or_else();
    demonstrate_match();
    demonstrate_if_let();
    demonstrate_question_mark();

    // Real-world examples
    println!("=== Real-world Examples ===");
    user_input_example();
    config_parsing_example();
}

fn demonstrate_unwrap() {
    println!("1. unwrap() - Panics on error, returns value on success");

    let good_input = "42";
    let result = good_input.parse::<i32>().unwrap();
    println!("  ✅ Success: {} parsed to {}", good_input, result);

    // This would panic - commented out for demo
    // let bad_input = "abc";
    // let result = bad_input.parse::<i32>().unwrap(); // 💥 PANIC! thread 'main' panicked

    println!("  ❌ With bad input: Would panic with 'called `Result::unwrap()` on an `Err` value'");
    println!("  📝 Use when: You're 100% sure the operation will succeed");
    println!("  ⚠️  Warning: Program will crash on error\n");
}

fn demonstrate_expect() {
    println!("2. expect() - Panics with custom message on error");

    let good_input = "123";
    let result = good_input.parse::<i32>().expect("This should never fail");
    println!("  ✅ Success: {} parsed to {}", good_input, result);

    // This would panic with custom message - commented out
    // let bad_input = "xyz";
    // let result = bad_input.parse::<i32>().expect("Failed to parse user input as number");

    println!("  ❌ With bad input: Would panic with custom message");
    println!("  📝 Use when: You want a meaningful error message for debugging");
    println!("  ⚠️  Warning: Program will crash on error, but with better context\n");
}

fn demonstrate_unwrap_or() {
    println!("3. unwrap_or() - Returns default value on error");

    let good_input = "456";
    let bad_input = "not_a_number";

    let result_good = good_input.parse::<i32>().unwrap_or(0);
    let result_bad = bad_input.parse::<i32>().unwrap_or(0);

    println!("  ✅ Success: '{}' -> {}", good_input, result_good);
    println!(
        "  🔄 Error handled: '{}' -> {} (default)",
        bad_input, result_bad
    );
    println!("  📝 Use when: You have a sensible default value");
    println!("  ✅ Safe: Never panics\n");
}

fn demonstrate_unwrap_or_else() {
    println!("4. unwrap_or_else() - Calls function to generate default on error");

    let inputs = ["789", "invalid", ""];

    for input in inputs.iter() {
        let result = input.parse::<i32>().unwrap_or_else(|error| {
            println!("    Error parsing '{}': {}", input, error);
            -1 // Default value
        });
        println!("  Input: '{}' -> {}", input, result);
    }

    println!("  📝 Use when: You need to compute the default or log the error");
    println!("  ✅ Safe: Never panics\n");
}

fn demonstrate_match() {
    println!("5. match - Full control over error handling");

    let inputs = ["42", "not_a_number", "999999999999999999999"];

    for input in inputs.iter() {
        match input.parse::<i32>() {
            Ok(number) => println!("  ✅ Parsed '{}' successfully: {}", input, number),
            Err(ParseIntError { .. }) => {
                println!("  ❌ Failed to parse '{}' as integer", input);
                // Could handle different error types here
            }
        }
    }

    println!("  📝 Use when: You need different behavior for different errors");
    println!("  ✅ Safe: Never panics, explicit error handling");
    println!("  🔧 Flexible: Can handle different error types differently\n");
}

fn demonstrate_if_let() {
    println!("6. if let - Concise success handling");

    let inputs = ["100", "abc", "200"];

    for input in inputs.iter() {
        if let Ok(number) = input.parse::<i32>() {
            println!("  ✅ Successfully parsed '{}': {}", input, number);
        } else {
            println!("  ❌ Could not parse '{}'", input);
        }
    }

    println!("  📝 Use when: You only care about the success case");
    println!("  ✅ Safe: Never panics\n");
}

fn demonstrate_question_mark() {
    println!("7. ? operator - Early return on error");

    fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
        let number = input.parse::<i32>()?; // Returns error if parsing fails
        Ok(number * 2)
    }

    let inputs = ["21", "not_a_number"];

    for input in inputs.iter() {
        match parse_and_double(input) {
            Ok(result) => println!("  ✅ '{}' doubled: {}", input, result),
            Err(e) => println!("  ❌ Error with '{}': {}", input, e),
        }
    }

    println!("  📝 Use when: You want to propagate errors up the call stack");
    println!("  ✅ Safe: Never panics, returns Result\n");
}

fn user_input_example() {
    println!("\n🎯 Real Example: User Input Validation");

    fn get_user_age(input: &str) -> Option<u32> {
        match input.trim().parse::<u32>() {
            Ok(age) if age <= 150 => Some(age), // Reasonable age limit
            Ok(_) => None,                      // Age too high
            Err(_) => None,                     // Parse error
        }
    }

    let test_inputs = ["25", "abc", "200", "  30  ", "0"];

    for input in test_inputs.iter() {
        match get_user_age(input) {
            Some(age) => println!("  Valid age: {}", age),
            None => println!("  Invalid input: '{}'", input),
        }
    }
}

fn config_parsing_example() {
    println!("\n⚙️  Real Example: Configuration Parsing");

    #[derive(Debug)]
    struct Config {
        port: u16,
        max_connections: u32,
        timeout_seconds: u64,
    }

    impl Config {
        fn from_strings(port: &str, max_conn: &str, timeout: &str) -> Result<Self, String> {
            let port = port
                .parse::<u16>()
                .map_err(|_| format!("Invalid port: '{}'", port))?;

            let max_connections = max_conn
                .parse::<u32>()
                .map_err(|_| format!("Invalid max_connections: '{}'", max_conn))?;

            let timeout_seconds = timeout
                .parse::<u64>()
                .map_err(|_| format!("Invalid timeout: '{}'", timeout))?;

            Ok(Config {
                port,
                max_connections,
                timeout_seconds,
            })
        }
    }

    // Test with different inputs
    let test_cases = [
        ("8080", "100", "30"),
        ("invalid", "100", "30"),
        ("8080", "abc", "30"),
        ("8080", "100", "xyz"),
    ];

    for (port, max_conn, timeout) in test_cases.iter() {
        match Config::from_strings(port, max_conn, timeout) {
            Ok(config) => println!("  ✅ Config: {:?}", config),
            Err(e) => println!("  ❌ Config error: {}", e),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unwrap_or_behavior() {
        assert_eq!("42".parse::<i32>().unwrap_or(0), 42);
        assert_eq!("abc".parse::<i32>().unwrap_or(0), 0);
    }

    #[test]
    fn test_match_behavior() {
        let result = match "42".parse::<i32>() {
            Ok(n) => n,
            Err(_) => 0,
        };
        assert_eq!(result, 42);
    }

    #[test]
    #[should_panic(expected = "Test panic")]
    fn test_expect_panic() {
        "abc".parse::<i32>().expect("Test panic");
    }
}

/*
=== SUMMARY OF ERROR HANDLING METHODS ===

┌─────────────────┬────────────────┬─────────────────┬──────────────────┐
│ Method          │ Panics?        │ Returns         │ Best Used When   │
├─────────────────┼────────────────┼─────────────────┼──────────────────┤
│ unwrap()        │ Yes            │ T               │ 100% sure       │
│ expect()        │ Yes (custom)   │ T               │ Want context     │
│ unwrap_or()     │ No             │ T               │ Have default     │
│ unwrap_or_else()│ No             │ T               │ Compute default  │
│ match           │ No             │ Whatever you    │ Full control     │
│                 │                │ decide          │                  │
│ if let          │ No             │ ()              │ Only care about  │
│                 │                │                 │ success          │
│ ? operator      │ No             │ Result<T, E>    │ Propagate errors │
└─────────────────┴────────────────┴─────────────────┴──────────────────┘

🏆 RECOMMENDATIONS:
- Use `match` or `if let` for production code
- Use `unwrap_or()` when you have a sensible default
- Use `expect()` for prototyping with descriptive messages
- Avoid `unwrap()` in production unless you're 100% certain
- Use `?` operator for clean error propagation
*/
