use std::collections::HashMap;
use rand::Rng;
use std::{cmp::Ordering, io};
use std::collections::*;

fn main() {
    let mut map = HashMap::new();
    map.insert("key", 1);

    println!("{:?}", map);
    println!("{:?}", map.get("key"));


    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret numer is {}", secret_number);


}