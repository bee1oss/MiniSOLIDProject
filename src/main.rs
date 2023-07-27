mod shape;

use crate::shape::{Shape, Rectangle, Circle};

fn main() {
    let rectangle = Rectangle::new(5.0, 10.0);
    let circle = Circle::new(7.5);
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(rectangle), Box::new(circle)];

    for shape in shapes {
        println!("Area:{:2}", shape.area());
    }
}
