fn main() {
    let mut x = 5;
    println!("x = {x}");
    x = 6;
    println!("x = {x}");

    const THREE_HOURS_IN_SECONDS : u32 = 60 * 60 * 3;


    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("in inner scope x = {x}");
    }

    println!("outside of inner scope x = {x}");

    let spaces = "    ";
    let spaces = spaces.len();
    println!("spaces = {spaces}");
}
