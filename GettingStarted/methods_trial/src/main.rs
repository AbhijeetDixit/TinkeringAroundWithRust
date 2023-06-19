#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) ->u32{
        self.width * self.height
    }
}


fn main() {
    let rectangle_struct = Rectangle{
        width:30,
        height:50,
    };

    println!("Area of the rectangle is : {}", rectangle_struct.area());
}
