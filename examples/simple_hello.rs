fn main() {
    println!("=== Simple Hello Example ===");

    // Basic greeting
    simple_hello();

    // Personalized greeting
    greet_person("Alice");
    greet_person("Bob");

    // Multiple languages
    multilingual_hello();

    // Formatted output
    fancy_greeting();
}

fn simple_hello() {
    println!("Hello from simple example!");
}

fn greet_person(name: &str) {
    println!("Hello, {}! Nice to meet you.", name);
}

fn multilingual_hello() {
    let greetings = vec![
        ("English", "Hello!"),
        ("Spanish", "¡Hola!"),
        ("French", "Bonjour!"),
        ("German", "Hallo!"),
        ("Japanese", "こんにちは!"),
    ];

    println!("\nMultilingual greetings:");
    for (language, greeting) in greetings {
        println!("{}: {}", language, greeting);
    }
}

fn fancy_greeting() {
    let message = "Welcome to Rust Examples!";
    let border = "=".repeat(message.len());

    println!("\n{}", border);
    println!("{}", message);
    println!("{}", border);
}
