#[derive(Debug)]

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn greet(prefix: &'static str) -> String {
        let mut greeting = String::from("");
        greeting.push_str("porumai ... wait and hope !!! ");
        greeting.push_str(prefix);

        greeting
    }
}

fn main() {
    let name = "Arun";
    let rectangle = Rectangle {
        width: 32,
        height: 32,
    };
    let other_rectangle = Rectangle {
        width: 24,
        height: 24,
    };

    println!(
        "Porumai ... area of rectangle {:#?} is {} square pixels !, {}, {} ",
        rectangle,
        rectangle.area(),
        rectangle.can_hold(&other_rectangle),
        Rectangle::greet(name)
    );
}
