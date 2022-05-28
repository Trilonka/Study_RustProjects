fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("the value of x is: {x}");

    let y = 5;
    let y = y + 1;
    {
        let y = y*2;
        println!("The value of y in the inner scope is: {y}");
    }
    println!("The value of y is: {y}");

    let spaces = "     ";
    let spaces = spaces.len();
    println!("{spaces}");

    // При релизе не происходит паники при выходе за рамки диапазонов

    let tup = (500, 6.4, 1);
    let (_a, b, _c) = tup;
    println!("The value of b is: {b}");
    let one = tup.2;
    println!("The value of c is: {one}");

    let ar = [1,2,3,4,5];
    let t = ar[3];
    println!("The value of t is: {t}");
}
