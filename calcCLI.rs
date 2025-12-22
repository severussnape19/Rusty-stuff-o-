use std::io;

fn main() {
    println!("Enter your name: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read message");

    println!("Enter a number: ");
    let mut first = String::new();
    io::stdin()
        .read_line(&mut first)
        .expect("Failed to read message");


    println!("Enter second number: ");
    let mut second = String::new();
    io::stdin()
        .read_line(&mut second)
        .expect("Failed to read message");

    let first_num: f64 = match first.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid number!");
            0.0
        }
    };

    let second_num: f64 = match second.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a valid number!");
            0.0
        }
    };

    let mut numbers: Vec<f64> = Vec::new();
    numbers.push(first_num);
    numbers.push(second_num);

    println!("Operator: ");
    let mut op: String = String::new();
    io::stdin()
        .read_line(&mut op)
        .expect("Failed to read message");

    let answer =  match op.trim() {
        "+" => numbers[0] + numbers[1],
        "-" => numbers[0] - numbers[1],
        "*" => numbers[0] * numbers[1],
        "/" => {
            if numbers[1] == 0.0 {
                println!("Error: Division by zero not allowed!");
                return;
            }
            numbers[0] / numbers[1]
        },
        "%" => numbers[0] % numbers[1],
        "^" => numbers[0].powf(numbers[1]),
        _ => { 
            println!("Enter a valid operator!"); 
            return;
        }
    };

    println!("Hello, {}!", input.trim());
    println!("First Number: {} | Second Number: {} | Operator: {}", numbers[0], numbers[1], op.trim());
    println!("Result: {}", answer)

}