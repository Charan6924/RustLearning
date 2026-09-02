#[derive(Debug)]
struct Rectangle { 
    height : u32,
    width : u32
}

impl Rectangle{
    fn area(&self) -> u32{
        self.width * self.height
    }
}

fn main(){
    let rect1 = Rectangle{
        height: 12,
        width : 4
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

}

