use crate::garden::vegetables::Asparagus;

pub mod garden;

fn main() {
    let plant = Asparagus {
        color: String::from("blue"),
        stalks: 3,
    };

    println!("plant is {plant:?}!");
}