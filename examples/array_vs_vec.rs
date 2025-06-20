fn main() {
    println!("=== ARRAYS vs VECTORS in Rust ===\n");

    // 1. ARRAYS - Fixed size, known at compile time
    println!("1. ARRAYS (Fixed size):");
    let array: [&str; 3] = ["German", "Japanese", "English"];
    println!("Array: {:?}", array);
    println!("Array size: {}", array.len());
    // array.push("French"); // ❌ This won't work - arrays are fixed size

    // 2. VECTORS - Dynamic size, can grow/shrink
    println!("\n2. VECTORS (Dynamic size):");
    let mut vector: Vec<&str> = vec!["German", "Japanese", "English"];
    println!("Vector: {:?}", vector);
    println!("Vector size: {}", vector.len());

    // Vectors can be modified
    vector.push("French");
    vector.push("Spanish");
    println!("After adding languages: {:?}", vector);
    println!("New vector size: {}", vector.len());

    // 3. MEMORY ALLOCATION
    println!("\n3. MEMORY ALLOCATION:");
    println!("Arrays: Stored on STACK (faster access)");
    println!("Vectors: Stored on HEAP (more flexible)");

    // 4. TYPE DIFFERENCES
    println!("\n4. TYPE SYSTEM:");
    let arr1: [i32; 3] = [1, 2, 3];
    let arr2: [i32; 4] = [1, 2, 3, 4];
    // arr1 and arr2 are DIFFERENT types because size is part of the type

    let vec1: Vec<i32> = vec![1, 2, 3];
    let vec2: Vec<i32> = vec![1, 2, 3, 4];
    // vec1 and vec2 are the SAME type

    println!("arr1 type: [i32; 3]");
    println!("arr2 type: [i32; 4] <- Different type!");
    println!("vec1 type: Vec<i32>");
    println!("vec2 type: Vec<i32> <- Same type!");

    // 5. PERFORMANCE COMPARISON
    println!("\n5. PERFORMANCE:");
    performance_demo();

    // 6. PRACTICAL EXAMPLES
    println!("\n6. WHEN TO USE WHAT:");
    println!("Use ARRAYS when:");
    println!("  - Size is known at compile time");
    println!("  - Small, fixed collections");
    println!("  - Maximum performance needed");
    println!("  - Stack allocation preferred");

    println!("\nUse VECTORS when:");
    println!("  - Size changes during runtime");
    println!("  - Unknown size at compile time");
    println!("  - Need to add/remove elements");
    println!("  - Working with user input or dynamic data");

    // 7. CONVERSION BETWEEN THEM
    println!("\n7. CONVERSION:");
    let original_array = [1, 2, 3, 4, 5];
    let array_to_vec: Vec<i32> = original_array.to_vec();
    println!("Array to Vec: {:?}", array_to_vec);

    let original_vec = vec![10, 20, 30];
    let vec_to_array: [i32; 3] = original_vec.try_into().unwrap();
    println!("Vec to Array: {:?}", vec_to_array);
}

fn performance_demo() {
    use std::time::Instant;

    // Array performance
    let start = Instant::now();
    let arr = [0; 10000];
    let sum: i32 = arr.iter().sum();
    let array_time = start.elapsed();

    // Vector performance
    let start = Instant::now();
    let vec = vec![0; 10000];
    let sum: i32 = vec.iter().sum();
    let vector_time = start.elapsed();

    println!("Array access time: {:?}", array_time);
    println!("Vector access time: {:?}", vector_time);
    println!("Arrays are typically faster for fixed-size operations");
}

// Function that demonstrates type constraints
fn process_fixed_data(data: [i32; 5]) -> i32 {
    // Can only accept arrays of exactly 5 elements
    data.iter().sum()
}

fn process_dynamic_data(data: Vec<i32>) -> i32 {
    // Can accept vectors of any size
    data.iter().sum()
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_array_characteristics() {
        let arr = [1, 2, 3];
        assert_eq!(arr.len(), 3);
        assert_eq!(std::mem::size_of_val(&arr), 12); // 3 * 4 bytes
    }

    #[test]
    fn test_vector_characteristics() {
        let mut vec = vec![1, 2, 3];
        assert_eq!(vec.len(), 3);

        vec.push(4);
        assert_eq!(vec.len(), 4);

        vec.pop();
        assert_eq!(vec.len(), 3);
    }
}
