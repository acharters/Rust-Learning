
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32
}

impl Rectangle {

    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        if self.width >= rect.width && self.height >= rect.height {
            true
        } else {
            false
        }
    }

    fn make_square(x: i32) -> Rectangle {
        Rectangle {
            width: x,
            height: x
        }
    }

}

fn main() {
    let rect1 = Rectangle {
        width: 50,
        height: 60
    };
    let rect2 = Rectangle {
        width: 70,
        ..rect1
    };
    let sq = Rectangle::make_square(5);
    println!("Rect1: {:#?}\nRect2: {:?}", rect1, rect2);
    println!("Hold value: {}", rect1.can_hold(&rect2));
    println!("Square: {:#?}", sq);
}


