fn main() {
    shadowing_example();
}

// by default immutable
fn mutable_example() {
    let mut number = 2;
    println!("number: {}", number);
    number = 3;
    println!("number: {}", number);
}

const CONST_VALUE : i32 = 10_000;

fn shadowing_example() {

    let x = 5;

    let x = x + 1;
    {
        let x = x * 2;
        println!("The value if x in the inner scope is: {}", x);
    }

    println!("The value of x is: {}", x);

    let name = "Valon";
    {
        let name = "Adrian";
        println!("name: {}", name);
    }

    println!("name: {}", name);
}