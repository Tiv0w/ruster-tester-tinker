use crate::utils::choice;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other_rect: &Rectangle) -> bool {
        self.width > other_rect.width && self.height > other_rect.height
    }
}

pub fn rectangle_tester() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };
    println!("The area is: {}", rect1.area());
    println!("The rectangle is: {:#?}", rect1);
    println!("Can rect1 holds rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 holds rect3? {}", rect1.can_hold(&rect3));
}

#[allow(dead_code)]
pub fn _interactive_rectangle_tester() {
    println!("Width:");
    let width = choice::ask_u32_integer_choice();
    println!("Height:");
    let height = choice::ask_u32_integer_choice();
    let rect1 = Rectangle { width, height };
    let area = rect1.area();
    println!("The area is: {}", area);
    println!("The rectangle is: {:#?}", rect1);
}
