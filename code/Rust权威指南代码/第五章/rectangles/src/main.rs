struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    // way 1
    let width1 = 30;
    let height1 = 50;
    
    println!(
        "The area of the rectangle is {} square pixels.",
        area1(width1, height1)
    );

    // way 2
    let rect1 = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect1)
    );

    // way 3
    let rect2 = Rectangle { width: 30, height: 50 };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect2)
    );
}

fn area1(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}


