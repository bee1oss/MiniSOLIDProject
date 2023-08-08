/*1-Single Responsibility Principle:We will create a separate class for each geometric shape so that each class
performs only the area calculation function.*/
pub trait Shape {
    fn area(&self) -> f64;
}

pub struct Rectangle {
    width: f64,
    height: f64,
}

impl Rectangle {
    pub fn new(width: f64, height: f64) -> Rectangle {
        Rectangle { width, height }
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

pub struct Circle {
    radius: f64,
}

impl Circle {
    pub fn new(radius: f64) -> Circle {
        Circle { radius }
    }
}

impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

pub struct Triangle {
    base: f64,
    height: f64,
}

impl Triangle {
    pub fn new(base: f64, height: f64) -> Triangle {
        Triangle { base, height }
    }
}

impl Shape for Triangle{
    fn area(&self) -> f64 {
        0.5*self.base*self.height   
    }
}









/*******
2-Open/Closed Principle:We've designed our app to be open to expansion. We will be able to create new
classes to add new geometric shapes without changing the existing code.

3-Liskov Substitution Principle:The Rectangle and Circle classes are types that conform to the Shape
interface and are therefore interchangeable

4-Interface Segregation Principle:Since we only have one Shape interface here, we naturally follow
this principle anyway

5-Dependency Inversion Principle:High-level modules should not depend on low-level modules. The main
application will calculate the areas of shape objects using the Shape interface.
*******/