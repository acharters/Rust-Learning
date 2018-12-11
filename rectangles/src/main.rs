
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
    println!("Rect1: {:#?}\nRect2: {:?}", rect1, rect2);
    println!("Hold value: {}", rect1.can_hold(&rect2));
}


