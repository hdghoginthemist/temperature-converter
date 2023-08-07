use std::io;

fn main(){

    println!("Temperature Converter");

    loop {
        println!("Press 1 to convert Celsius to Fahrenheit");
        println!("Press 2 to convert Fahrenheit to Celcius");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let input = input.trim();
        match input {
            "1" => {
                let (a, b) = c_to_f();
                println!("{} degrees Celsius is {} degrees Fahrenheit.", a, b);
            }
            "2" => {
                let (a, b) = f_to_c();
                println!("{} degrees Fahrenfeit is {} degrees Celcius.", a, b);
            }
            _ => {
                println!("Got some other input");
                continue;
            }
        };

        break;
    };
}

fn get_temperature() -> f64 {
    loop {
        println!("Please enter the temperature you want to convert");
        let mut temperature = String::new();
        io::stdin().read_line(&mut temperature).expect("Failed to read line");
        let temperature: f64 = match temperature.trim().parse() {
            Ok(num) => num,
            Err(_) => {
            println!("please input a valid temperature");
            continue;
            }
        };
        return temperature;
    };
}

fn c_to_f() -> (f64, f64) {
    let t = get_temperature();
    let converted = t * 1.8 + 32.0;

    return (t, converted);
}

fn f_to_c() -> (f64, f64) {
    let t = get_temperature();
    let converted = (t - 32.0) / 1.8;

    return (t, converted);
}