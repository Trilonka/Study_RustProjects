use std::collections::HashMap;

fn main() {
    let mut a = vec![1,2,3,4,5,6,12,12,12,85,42,7,9,9,9];
    println!("{:#?}", first(&mut a))
}

fn first(v: &mut Vec<i32>) -> (f32, i32, i32) {
    let average = v.iter().sum::<i32>() as f32 / v.len() as f32;

    v.sort();
    let mediane = v[v.len()/2];

    let mut map: HashMap<i32, i32> = HashMap::new();
    for &mut el in v {
        *map.entry(el).or_insert(0) += 1;
    }
    let mode = map
        .into_iter()
        .max_by_key(|&(_, count)| count)
        .map(|(val, _)| val)
        .expect("Cannot compute ht mode of zero number");

    (average, mediane, mode)
}

fn second(word: &str) -> String {
    let vowels = [ 'a', 'e', 'i', 'o', 'u', 'y'];
    if let Some(s) = word.get(0..1) {
        if s.contains(vowels) {
            return format!("{}-hay", word)
        }
        return if let Some(wor) = word.get(1..word.len()) {
            format!("{}-{}ay", wor, s)
        } else {
            format!("-{}ay", s)
        }
    }
    "".to_string()
}

fn third() {} // am...