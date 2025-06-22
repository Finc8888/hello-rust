use std::io;

fn main() {
    println!("=== Fibonacci Calculator (1-based indexing) ===");
    println!("This calculator uses 1-based indexing:");
    println!("F(1) = 0, F(2) = 1, F(3) = 1, F(4) = 2, F(5) = 3, F(6) = 5, ...");
    println!();

    loop {
        println!("Enter the position number for Fibonacci sequence (or 'quit' to exit):");
        let mut input = String::new();

        // Read user input
        match io::stdin().read_line(&mut input) {
            Ok(_) => {}
            Err(_) => {
                println!("Error reading input. Please try again.");
                continue;
            }
        }

        let input = input.trim();

        // Allow user to exit
        if input.eq_ignore_ascii_case("quit") || input.eq_ignore_ascii_case("q") {
            println!("Thank you for using Fibonacci calculator!");
            break;
        }

        // Parse the input number
        let n: u32 = match input.parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Please enter a positive number (starting from 1).");
                    continue;
                }
                num
            }
            Err(_) => {
                println!(
                    "Error: '{}' is not a valid number. Please enter a positive integer.",
                    input
                );
                continue;
            }
        };

        // Check for potential overflow (u32 max Fibonacci is around F(47))
        if n > 47 {
            println!("Warning: Large numbers may cause overflow. Maximum recommended value is 47.");
            println!("Do you want to continue? (y/n):");

            let mut confirm = String::new();
            if io::stdin().read_line(&mut confirm).is_ok() {
                if !confirm.trim().eq_ignore_ascii_case("y") {
                    continue;
                }
            }
        }

        // Calculate Fibonacci number
        let result = fibonacci_1based(n);

        // Display result with context
        println!("F({}) = {}", n, result);

        // Show sequence context for small numbers
        if n <= 10 {
            print!("Sequence: ");
            for i in 1..=n.min(10) {
                print!("F({})={}", i, fibonacci_1based(i));
                if i < n.min(10) {
                    print!(", ");
                }
            }
            println!();
        }

        println!();
    }
}

/// Calculates the nth Fibonacci number using 1-based indexing
///
/// In 1-based indexing:
/// - F(1) = 0 (first Fibonacci number)
/// - F(2) = 1 (second Fibonacci number)
/// - F(3) = 1 (third Fibonacci number)
/// - F(n) = F(n-1) + F(n-2) for n > 2
///
/// The sequence is: 0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, ...
///
/// # Arguments
/// * `n` - Position in the Fibonacci sequence (must be >= 1)
///
/// # Returns
/// * The nth Fibonacci number
///
/// # Algorithm
/// Uses an iterative approach with O(n) time complexity and O(1) space complexity.
/// Maintains two variables (prev, cur) to track consecutive Fibonacci numbers.
///
/// # Examples
/// ```
/// assert_eq!(fibonacci_1based(1), 0);  // F(1) = 0
/// assert_eq!(fibonacci_1based(2), 1);  // F(2) = 1
/// assert_eq!(fibonacci_1based(5), 3);  // F(5) = 3
/// ```
fn fibonacci_1based(n: u32) -> u32 {
    // Handle the base case F(1) = 0
    if n == 1 {
        return 0;
    }

    // For n >= 2, use iterative calculation
    let mut index: u32 = 1;
    let mut prev: u32 = 0; // F(1) = 0
    let mut cur: u32 = 1; // F(2) = 1

    // Calculate F(3) through F(n)
    while index < n {
        let sum: u32 = prev + cur; // F(i+1) = F(i) + F(i-1)
        prev = cur; // Move to next position
        cur = sum; // Update current value
        index += 1; // Move to next index
    }

    prev // Return F(n)
}

/// Alternative implementation with explicit base cases for clarity
#[allow(dead_code)]
fn fibonacci_1based_explicit(n: u32) -> u32 {
    match n {
        1 => 0,
        2 => 1,
        _ => {
            let mut prev = 0; // F(1)
            let mut cur = 1; // F(2)

            // Calculate F(3) through F(n)
            for _ in 3..=n {
                let next = prev + cur;
                prev = cur;
                cur = next;
            }

            cur
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_1based_basic_cases() {
        assert_eq!(fibonacci_1based(1), 0);
        assert_eq!(fibonacci_1based(2), 1);
        assert_eq!(fibonacci_1based(3), 1);
        assert_eq!(fibonacci_1based(4), 2);
        assert_eq!(fibonacci_1based(5), 3);
    }

    #[test]
    fn test_fibonacci_1based_sequence() {
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377];

        for (i, &expected_val) in expected.iter().enumerate() {
            let n = (i + 1) as u32; // Convert to 1-based index
            assert_eq!(
                fibonacci_1based(n),
                expected_val,
                "F({}) should be {}",
                n,
                expected_val
            );
        }
    }

    #[test]
    fn test_fibonacci_implementations_match() {
        for n in 1..=20 {
            assert_eq!(
                fibonacci_1based(n),
                fibonacci_1based_explicit(n),
                "Implementations should match for F({})",
                n
            );
        }
    }

    #[test]
    fn test_fibonacci_larger_values() {
        assert_eq!(fibonacci_1based(20), 4181);
        assert_eq!(fibonacci_1based(25), 46368);
        assert_eq!(fibonacci_1based(30), 514229);
    }
}
