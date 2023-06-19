#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32,
}

fn main() {
    let width1 = 30;
    let height1 = 50;
    let rect1 = (30,50);
    let rectangle_struct = Rectangle{
        width: 30,
        height: 50,
    };

    println!("The area of rectangle is {}", area(width1, height1));
    println!("The area of rectangle using tuples is {}", area_with_tuples(rect1));
    println!("The area of rectangle using structs is {}", area_with_structs(&rectangle_struct));
    dbg!(&rectangle_struct);
}

fn area(w: u32, h: u32) -> u32{
    w*h
}

fn area_with_tuples(dimesnions :(u32, u32)) -> u32{
    dimesnions.0 * dimesnions.1
}

fn area_with_structs(rectangle: &Rectangle) ->u32{
    println!("Rectangle is {:#?}",rectangle);
    rectangle.width * rectangle.height
}