fn main() {
    //let mut v: Vec<i32> = Vec::new();
    //let v = vec![1, 2, 3]; // <i32>
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);

    let v = vec![1, 2, 3, 4, 5]; // Затенение
    let third: &i32 = &v[2];
    println!("the third element is {third}");
    let third: Option<&i32> = v.get(2);
    match third {
        Some(value) => println!("The third element is {value}"),
        None => println!("There is no third element."),
    }
    // ---------wow---
    let v = vec![1, 2, 3, 4, 5];
    //let does_not_exist = &v[100]; // it's PANIC!!!
    let does_not_exist = v.get(100);
    // ---------wow---
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // v.push(6); // Так сделать нельзя, тк мы имеем неизменную ссылку на 1 элемент, а потом изменяем вектор, 
    // а при изменении может произойти выделение новой памяти и копирование, а значит мы будем иметь недействительную ссылку на 1 элемент
    println!("The first element is: {first}");
    // ---------wow---
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
        println!("{}", i);
    }
    // ---------wow---
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Float(10.12),
        SpreadsheetCell::Text(String::from("blue")),
    ];
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String), 
}
