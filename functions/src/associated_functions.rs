struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // Associated function to create a new Rectangle instance
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Associated function to calculate the area of a Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

pub fn main() {
    // Using the associated function `new` to create a Rectangle instance
    let rect = Rectangle::new(10, 20);

    // Accessing the instance method `area` on the Rectangle instance
    let area = rect.area();

    println!("Area: {}", area);
}
