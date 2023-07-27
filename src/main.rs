mod shape;

use crate::shape::{Shape, Rectangle, Circle, Triangle};

fn main() {
    let rectangle = Rectangle::new(5.0, 10.0);
    let circle = Circle::new(7.5);
    let triangle = Triangle::new(6.0,5.0);
    let shapes: Vec<Box<dyn Shape>> = vec![Box::new(rectangle), Box::new(circle),Box::new(triangle)];

    for shape in shapes {
        println!("Area:{:?}", shape.area());
    }
}
