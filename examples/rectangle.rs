fn main() {
    let rectangle = Rectangle {
        width: 12,
        height: 5,
    };
    println!(
        "Area for rectangle with widht = {} and height = {} is {} squre pixel",
        &rectangle.width,
        &rectangle.height,
        calc_area(&rectangle)
    );
}

struct Rectangle {
    width: u32,
    height: u32,
}

fn calc_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_area() {
        let rectangle1 = Rectangle {
            width: 12,
            height: 5,
        };
        let rectangle2 = Rectangle {
            width: 1,
            height: 1,
        };
        assert_eq!(calc_area(&rectangle1), 60);
        assert_eq!(calc_area(&rectangle2), 1);
    }
}
