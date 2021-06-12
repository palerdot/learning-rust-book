fn add_two(a: i32) -> i32 {
    a + 2
}

fn greet(name: &str) -> String {
    format!("porumai ... wait and hope !!! {} ", name)
    // String::from("porumai")
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value should be > 1. was {} ", value);
        }
        if value > 32 {
            panic!("Guess value should be < 32. was {} ", value);
        }
        Guess { value }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn rect_holder_check() {
        let large_rectangle = Rectangle {
            width: 32,
            height: 32,
        };

        let small_rectangle = Rectangle {
            width: 24,
            height: 24,
        };

        assert!(large_rectangle.can_hold(&small_rectangle));
        assert!(!small_rectangle.can_hold(&large_rectangle));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(32, add_two(30));
    }

    #[test]
    fn it_adds_two_result_version() -> Result<(), String> {
        if 30 + 2 == 32 {
            Ok(())
        } else {
            Err(String::from("porumai ... 30 + 2 should be 32"))
        }
    }

    #[test]
    fn greeting_contains_name() {
        let name = "Arun";
        let greeting = greet(name);

        assert!(
            greeting.contains(name),
            "Greeting does not contain name. value was {}",
            greeting
        );
    }

    #[test]
    #[should_panic(expected = "Guess value should be < 32")]
    fn greater_than_32() {
        Guess::new(111);
    }
}
