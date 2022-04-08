use std::io;

pub fn convert_example() {
    let convert_type = get_user_input();

    println!("Enter value");
    let mut input = String::new();

    loop {
        match io::stdin()
        .read_line(&mut input){
            Ok(input) => break input.to_string(),
            Err(_) => continue
        };  
    };

    let value: f64 = loop{
        match input.trim()
        .parse(){
            Ok(value) => break value,
            Err(_) => continue,
        }; 
    };
    
    let converted: f64;
    if convert_type.trim() == "F" {
        converted = convert_to_fahrenheit(value);
    }
    else{
        converted = convert_to_celsius(value);
    }

    print!("Converted: {} {}", convert_type, converted);
}

fn get_user_input() -> String {

    println!("Convert to Celsius: C or Fahrenheit: F");
    let mut convert_type = String::new();
    loop {
        io::stdin().read_line(&mut convert_type).expect("Error");
        if convert_type.len() == 0 {
            println!("Enter value");
            continue;
        }

        match convert_type.as_str().trim() {
            "F" => break convert_type,
            "C" => break convert_type,
            _ => {
                println!("enter valid value");
                continue
            },
        }
    }
}

fn convert_to_celsius(value: f64) -> f64 {
    return (value - 32.) / 1.8;
}

fn convert_to_fahrenheit(celius: f64) -> f64 {
    return (celius * 1.8) + 32.;
}