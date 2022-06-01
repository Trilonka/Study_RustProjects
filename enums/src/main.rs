<<<<<<< HEAD
enum IpAddr {
    V4(String),
=======
#[derive(Debug)]
enum IpAddr {
    V4(u32, u32, u32, u32),
>>>>>>> 5a968a86c12a9c1c39ee02953e75a06f81fee6f5
    V6(String),
}

fn main() {
<<<<<<< HEAD
    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));
=======
    let v4 = IpAddr::V4(127, 0, 0, 1);
    let v6 = IpAddr::V6(String::from("::1"));
    println!("{:#?}, {:#?}", v4, v6);
>>>>>>> 5a968a86c12a9c1c39ee02953e75a06f81fee6f5
}
