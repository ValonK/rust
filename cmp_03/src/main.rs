mod conversion;
mod fibonacci;

fn main() {
    // conversion::convert_example();
    for int in 0..15 {
        println!("fibonacci ({}) => {}", int, fibonacci::fib(200));
    }
}
