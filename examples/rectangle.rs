fn main() {
    let rectangle = Rectangle {
        width: 12,
        height: 5,
    };
    println!(
        "Area for rectangle with widht = {} and height = {} is {} squre pixel",
        &rectangle.width,
        &rectangle.height,
        rectangle.calc_area()
    );
    println!("{:#?}", rectangle);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }
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
        assert_eq!(rectangle1.calc_area(), 60);
        assert_eq!(rectangle2.calc_area(), 1);
    }
}
