use std::io;
use std::process;

fn main() {
    loop {
        let number1: u32 = read_user_input();
        let number2: u32 = read_user_input();

        let result = sum(number1, number2);
        println!("{} + {} = {}", number1, number2, result);
    }
}

fn say_message(message: String) {
    println!("{}", message);
}

fn sum(a: u32, b: u32) -> u32 {
    a + b
}

fn read_user_input() -> u32 {
    let mut input = String::new();
    let number: u32;
    say_message("enter a number".to_string());
    io::stdin().read_line(&mut input).unwrap();

    match input.trim().parse() {
        Ok(val) => number = val,
        Err(_err) => {
            println!("not a num");
            process::exit(1);
        }
    }
    number
}
