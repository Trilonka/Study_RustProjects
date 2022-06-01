enum IpAddr {
    V4(String),
    V6(String),
}

fn main() {
    let four = IpAddr::V4(String::from("127.0.0.1"));
    let six = IpAddr::V6(String::from("::1"));
}
