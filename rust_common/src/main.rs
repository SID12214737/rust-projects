#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let scale = 2;
    let rect1 = Rectangle{
        width: dbg!(30 * 2),
        height: 50
    };
    dbg!(&rect1);
    println!("rect is {rect1:#?}");
    
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}