#[derive(Debug)]
struct Rect {
    w: u32,
    h: u32,
}


impl Rect { 
    fn area(&self) -> u32 {
        self.w * self.h
    }
    fn can_hold(&self, other: &Rect) -> bool {
        self.w > other.w && self.h > other.h
    }

    fn square(size: u32) -> Rect {
        Rect { w: size, h: size }
    }
}

fn main() {
    let rect1 = Rect{
        w: 30,
        h: 50,
    };

    println!("rect1 is {:?}", rect1);
    println!("The area of the rectangle is {} square pixels.", rect1.area());


    let rect2 = Rect{
        w: 10,
        h: 40,
    };

    println!("rect1 can hold rect2? {}", rect1.can_hold(&rect2));
    println!("rect2 can hold rect1? {}", rect2.can_hold(&rect1));
    

    let sq = Rect::square(3);
    println!("sq is {:?}", sq);

}
