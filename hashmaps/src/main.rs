use std::collections::HashMap;
use std::iter::Iterator;

fn main() {
    let mut scores = HashMap::new();
    let cor = [(String::from("Blue"), 10),
                                       (String::from("Yellow"), 50)];
    
    let a: HashMap<_, _> = cor.into_iter().collect();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    for (key, value) in &scores { // Печатает в произвольном порядке
        println!("{}: {}", key, value);
    }

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.entry(String::from("yellow")).or_insert(50);
    scores.entry(String::from("blue")).or_insert(50);
    println!("{:?}", scores);

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
