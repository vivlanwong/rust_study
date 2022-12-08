#[derive(Debug)]
struct Rectangle {
    width: u32,
    length: u32,
}
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.length
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.width > rect.width && self.length > rect.length
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            length: size,
        }
    }
}

fn main() {
    let rect = Rectangle {
        width: 30,
        length: 30,
    };

    let s = Rectangle::square(20);
    println!("{:?}", rect);
    let area = rect.area();
    println!("area:{}", area);
    let hold = rect.can_hold(&s);
    println!("hold:{}", hold)
}
