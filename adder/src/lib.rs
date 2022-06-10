#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    // #[test]
    // fn another() {
    //     panic!("Fail this test"); // failed test
    // }

    #[test]
    fn larget_can_hold_smaller() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle { length: 8, width: 7 };
        let smaller = Rectangle { length: 5, width: 1 };
        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Karol");
        assert!(
            result.contains("Karol"),
            "Greeting didn't contains name, result is '{}'", result
        );
    }

    #[test]
    #[should_panic(expected = "Значение догадки должно быть меньше или равно 100")] // true when panic
    fn greater_than_100() {
        Guess::new(200);
    }

    #[test]
    fn it_works() -> Result<(), String> { // we can use Result
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two is four"))
        }
    }

    #[test]
    #[ignore]
    fn expensive_test() {
        panic!("Expensive test!"); // this test will be ignored
    }
}

#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.length > other.length && self.width > other.width
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn greeting(name: &str) -> String {
    format!("Greeting, {}!", name)
}

struct Guess {
    value: i32,
}

impl Guess {
    fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Значение догадки должно быть больше или равно 1, 
            получено {}.", value);
        } else if value > 100 {
            panic!("Значение догадки должно быть меньше или равно 100, 
            получено {}.", value);
        }
        Guess { value }
    }
}