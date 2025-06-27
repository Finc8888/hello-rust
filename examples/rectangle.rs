fn main() {
    let rectangle = Rectangle {
        width: 12,
        height: 5,
    };
    let square = Rectangle::square(4);
    println!(
        "Area for rectangle with width = {} and height = {} is {} square pixel",
        &rectangle.width,
        &rectangle.height,
        rectangle.calc_area()
    );
    println!("{:#?}", rectangle);

    println!(
        "Area for square with width = {} and height = {} is {} square pixel",
        &square.width,
        &square.height,
        square.calc_area()
    );
    println!("{:#?}", square);
    println!(
        "Rectangle [{:?}] can hold square [{:?}] is {}",
        rectangle,
        square,
        rectangle.can_hold(&square)
    );
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    fn calc_area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
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

    #[test]
    fn test_can_hold() {
        let rectangle1 = Rectangle {
            width: 12,
            height: 5,
        };
        let rectangle2 = Rectangle {
            width: 1,
            height: 1,
        };
        assert_eq!(rectangle1.can_hold(&rectangle2), true);
        assert_eq!(rectangle2.can_hold(&rectangle1), false);
    }

    #[test]
    fn test_check_square_build() {
        let square = Rectangle::square(5);
        assert_eq!(square.width, 5);
        assert_eq!(square.height, 5);
    }
}
