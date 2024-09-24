use std::collections::HashMap;

fn main() {
    let v: Vec<i32> = Vec::new();
    let quick_vec = vec![1, 2, 3];

    let mut mut_vec: Vec<i32> = Vec::new();

    mut_vec.push(5);
    mut_vec.push(6);
    println!("{:?}", mut_vec);

    let second: &i32 = &mut_vec[1];

    println!("{:?}", second);

    // only this method is OOB error safe
    let get_second = mut_vec.get(1);
    match get_second {
        Some(second) => println!("{:?}", second),
        None => println!("No second element"),
    }

    let s = "this isn't even a string before .to_string()".to_string();

    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    let s2 = "something to concat".to_string();

    let s3 = format!("{s}-{s2}");
    println!("{}", s3);

    for c in "Зд".chars() {
        println!("{c}");
    }
    for b in "Зд".bytes() {
        println!("{b}");
    }


    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team = "Blue".to_string();
    let score = scores.get(&team).copied().unwrap_or(0);
    println!("{:?}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    scores.entry(String::from("Blue")).or_insert(50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for chr in text.chars() {
        let c = map.entry(chr).or_insert(0);
        *c += 1;
    }

    println!("{:?}", map);


    let mut v = vec![5, 2, 1, 3, 4];
    v.sort();
    println!("{:?}", v);
    let median = v[v.len() / 2];
    println!("{:?}", median);


}
