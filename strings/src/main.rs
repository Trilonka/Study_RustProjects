fn main() {
    let mut s = String::new();
    let data = "initial contents";
    let s = data.to_string();

    // the method also works on a literal directly;
    let s = "initial contents".to_string();
    // UTF-8
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");

    let mut s = String::from("foo");
    s.push_str(" bar");

    let mut s1 = String::from("foo");
    let s2 = "bar";
    s1.push_str(s2);
    println!("s2 is {}", s2); // we can do this, because push_str takes &str

    let mut s = String::from("lo");
    s.push('l'); // push takes symbol
    
    let s3 = s1+&s2; // + use add, add looks: fn add(self, s: &str) -> String {...}
    // we can't use s1, but can use s2.

    
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("tor");
    let s = format!("{}-{}-{}", s1, s2, s3); // Легче читается и не забирает во владение ни одного параметра

    let hello = "Здравствуйте";

    let s = &hello[0..4]; // Берем перве 4 байта, это будут первые 2 символа
    println!("{s}");

    let hello = "Hello";

    let s = &hello[0..4]; // Берем перве 4 байта, это будут первые 4 символа // Они по-разному кодируются, одно unicode, Другое utf-8, такие приколы
    // Так использовать срезы надо очень осторожно
    println!("{s}");


    for c in "नमस्ते".chars() { // 6 символов (2 каких то служебных или вроде связующих, интересный язык)
        println!("{}", c);
    }

    for b in "नमस्ते".bytes() { // 18 байт, по 3 байта на каждый из 6 символов, потому что Unicode кодировка
        println!("{}", b);
    }
}
