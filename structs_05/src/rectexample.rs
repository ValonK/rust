use std::sync::mpsc::Receiver;

pub struct Rectangle {
    pub height: u32, 
    pub width: u32,
}

impl Rectangle {

    // Methods must have a parameter named self of type Self 
    // for their first parameter, 
    // so Rust lets you abbreviate this with only
    // the name self in the first parameter spot.
    pub fn area(&self) -> u32 {
        self.width * self.height
    }

    // getter
    pub fn width(&self) -> u32{
        self.width
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool{
        self.area() > other.area()
    }

    // associated function, not a method, often used for creation / constructor
    pub fn square(size: u32) -> Rectangle{
        Rectangle { height: size, width: size }
    }

    pub fn cirumconference(&self) -> u32{
        2 * (self.width + self.height) 
    }        
}