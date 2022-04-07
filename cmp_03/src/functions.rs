

pub fn normal() {}

pub fn with_parm(name: i32){
    println!("Param: {}", name);
}

pub fn with_multiple_params(one: i32, two: i32, three: char){
    println!("One: {}", one);
    println!("Two: {}", two);
    println!("Three: {}", three);
}

pub fn expression(){

    let x = {
        let y = 2 + 2;
        y + 1
    };

    println!("expression result: {}", x);

    let m = {
        let a = 2 + 5;
        a - 3
    };

    println!("expression result: {}", m);
}

// function expression 
pub fn function_with_return_value() -> i32{
    10
}

// function with normal return
pub fn also_function_with_return_value() -> i32 {
    return 10;
}