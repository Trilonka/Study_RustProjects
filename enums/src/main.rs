#[derive(Debug)]
enum IpAddr {
    V4(u32, u32, u32, u32),
    V6(String),
}

fn main() {
    let v4 = IpAddr::V4(127, 0, 0, 1);
    let v6 = IpAddr::V6(String::from("::1"));
    println!("{:#?}, {:#?}", v4, v6);
}
