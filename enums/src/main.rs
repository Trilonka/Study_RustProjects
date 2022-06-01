use std::any::type_name;

#[derive(Debug)]
enum IpAddr {
    V4(u32, u32, u32, u32),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        println!("call method");
        // method body would be defines here
    }
}

// Standar enum, замена для Null в других языках. Нет возможности работать с Null значениями, используя интерфейс Option<T>
// enum Option<T> {
//     None,
//     Some(T),
// }

fn main() {
    let v4 = IpAddr::V4(127, 0, 0, 1);
    let v6 = IpAddr::V6(String::from("::1"));
    println!("{:#?}, {:#?}", v4, v6);

    let m = Message::Write(String::from("hello"));
    m.call();

    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;

    let x: i8 = 5;
    let y: Option<i8> = Some(x);
    // let sum = x+y; // error, because we can't sum i8 and Option<i8>
}
