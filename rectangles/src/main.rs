fn main() {
    let widht1 = 60;
    let height1 = 100;

    println!(
        "The area of rectangle is {} square pixels",
        area(widht1, height1)
    );
}

fn area(width: u32, height: u32) -> u32 {
    return width * height;
}
