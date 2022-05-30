fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1);

    println!("The length of {s1} is {len}.");

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);

    let r3 = &mut s;
    //println!("{}, {}, {}", r1, r2, r3); // we can't use immutable reference after declare mutable reference
    println!("{}", r3);

    let s = String::from("hello world");
    let hello = &s[0..5];
    let world = &s[6..11];

    let word = first_word(&s[..]);

    // s.clear(); // word is immutable slice, we can't create mutable slice
    let message = "hello dany"; // it is immutable slice / string literal is a slice
    println!("{}, {}, {}, {message}", hello, world, word);

    // ----

    let my_string = String::from("hello world"); // String in heap
    let _word = first_word(&my_string[0..6]);
    let _word = first_word(&my_string[..]);

    let _word = first_word(&my_string); // equivalent to whole slices of String

    let my_string_literal = "hello world"; // literal string in stack

    let _word = first_word(&my_string_literal[0..6]);
    let _word = first_word(&my_string_literal[..]);
    let _word = first_word(my_string_literal); // this works because my_string_literal is slice!

    // we can use slices to arrays, it works similarry
    let a = [1,2,3,4,5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2,3]); // slice type is &[i32]
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
