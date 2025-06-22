use std::io;

fn main() {
    loop {
        println!("Enter the index for Fibonacci number:");
        let mut n = String::new();
        io::stdin()
            .read_line(&mut n)
            .expect("Error during fibonacci number entering");
        let n: u64 = match n.trim().parse() {
            Ok(num) => {
                if num == 0 {
                    println!("Please enter a positive number (starting from 1).");
                    continue;
                }
                num
            }
            Err(_) => {
                println!("Please enter a positive number (starting from 1).");
                continue;
            }
        };
        println!("Calculating value for F({})...", n);
        let res: u64 = fibonacci(n);
        println!("F({}) = {}", n, res);
        break;
    }
}
/// Calculates nth Fibonacci number (1-based: F(1)=0, F(2)=1, F(3)=1, ...)
///
/// Uses 1-based indexing where:
/// - F(1) = 0 (first Fibonacci number)
/// - F(2) = 1 (second Fibonacci number)
/// - F(n) = F(n-1) + F(n-2) for n > 2
///
/// # Arguments
/// * `n` - Position in sequence (must be >= 1)
///
/// # Returns
/// * The nth Fibonacci number
fn fibonacci(n: u64) -> u64 {
    let mut index: u64 = 1;
    let mut prev: u64 = 0;
    let mut cur: u64 = 1;
    while index < n {
        let sum: u64 = prev + cur;
        prev = cur;
        cur = sum;
        index = index + 1;
    }
    prev
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_base_cases() {
        assert_eq!(fibonacci(1), 0); // F(1) = 0
        assert_eq!(fibonacci(2), 1); // F(2) = 1
        assert_eq!(fibonacci(3), 1); // F(3) = 1
    }

    #[test]
    fn test_fibonacci_sequence() {
        // Test first 10 numbers in 1-based Fibonacci sequence
        let expected = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

        for (i, &expected_val) in expected.iter().enumerate() {
            let n = (i + 1) as u64; // Convert to 1-based index
            assert_eq!(
                fibonacci(n),
                expected_val,
                "F({}) should be {}",
                n,
                expected_val
            );
        }
    }

    #[test]
    fn test_fibonacci_larger_numbers() {
        assert_eq!(fibonacci(15), 377);
        assert_eq!(fibonacci(20), 4181);
        assert_eq!(fibonacci(25), 46368);
    }

    #[test]
    fn test_fibonacci_properties() {
        // Test that F(n) = F(n-1) + F(n-2) for n > 2
        for n in 3..=15 {
            let fn_val = fibonacci(n);
            let fn_1 = fibonacci(n - 1);
            let fn_2 = fibonacci(n - 2);

            assert_eq!(
                fn_val,
                fn_1 + fn_2,
                "F({}) should equal F({}) + F({})",
                n,
                n - 1,
                n - 2
            );
        }
    }
}
