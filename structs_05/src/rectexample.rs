pub struct Rectangle {
    pub height: u32, 
    pub width: u32,
}

pub fn area(rect: &Rectangle) -> u32{
    rect.width * rect.height
}