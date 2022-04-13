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
}