// basic stuff
pub fn normal_if(){
    let x = 5;
    if x == 5 {
        println!("equal");
    }
    else if x < 5 {
        println!("smaller");   
    }
}


pub fn if_in_let(){
    let condition = true;
    let number = if condition { 5 } else { 3 };
    println!("Number: {}", number);
}

pub fn loop_function(){

    loop {
        println!("endless loop until we break");
        break;
    }
}

pub fn breaking_with_labeled_loop(){

    let mut count = 0;
    'counting_up: loop {
        println!("Count: {}", count);
        let mut remaining = 10;

        loop {
            
            println!("remaining = {}", remaining);
            if remaining == 9{
                break;
            }

            if count == 2 {
                break 'counting_up;
            }

            remaining -= 1;
        }
        
        count += 1;
    }

    println!("End cound={}", count);
}

pub fn returning_values_with_break(){
    let mut counter = 0;
    
    let loopval = loop {
        
        counter += 1;
        if counter >= 5{
            break 2 * 2;
        }    
    };
}

pub fn normal_for(){
    let arr = [1, 2, 3, 4, 5];
    for x in arr {
        println!("val: {}", x);
    }
}

pub fn range_loop(){
    for x in (1..4).rev(){
        println!("{}", x);
    }
    println!("Done");
}

