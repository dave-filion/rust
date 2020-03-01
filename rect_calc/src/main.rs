#[derive(Debug)]
struct Rectangle {
    w: u32,
    h: u32,
}

impl Rectangle {
    // methods
    fn area(&self) -> u32 {
        self.w * self.h
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.w > other.w && self.h > other.h
    }

    // associated methods
    fn square(size: u32) -> Rectangle {
        Rectangle{w: size, h: size}
    }
}

fn main() {
    let rect1 = Rectangle{w: 52, h: 10};

    println!("rect is {:?}", rect1);
    println!("rect is {:#?}", rect1);

    println!("the area of the rectange is {} square pix",
    area(&rect1));

    println!("the area of the rectange is {} square pix",
             rect1.area());

    let sq1 = Rectangle::square(10);

    println!("square: {:#?}", sq1);
}

fn area(rect: &Rectangle) -> u32 {
    rect.w * rect.h
}
