fn main() {
    let x64 = 0.1;
    let y64 = 0.2;
    let z64 = 0.3;
    let sum64 = x64 + y64;
    let sub64 = z64 - x64;
    println!("sum:f64 is {}", sum64);
    println!("sub:f64 is {}", sub64);

    let x32: f32 = 0.1;
    let y32: f32 = 0.2;
    let z32: f32 = 0.3;
    let sum32 = x32 + y32;
    let sub32 = z32 - x32;
    println!("sum:f32 is {}", sum32);
    println!("sub:f32 is {}", sub32);
}
