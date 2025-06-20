fn main() {
    println!("Hello world!");
    greeting();
}

fn greeting() {
    let german = "Hallo, große Kinder";
    let japanese = "こんにちは、大きな子供たち。";
    let regions = [german, japanese];
    for region in regions.iter() {
        println!("{}", region);
    }
}
