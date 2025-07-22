use std::io;
use std::io::Write;
use std::mem;
fn main() {
    match argparse() {
        Arg::Temp => temperature(),
        Arg::Fib => fibonacci(),
        Arg::Chr => christmas_carol(),
    }
}
enum Arg {
    Temp,
    Fib,
    Chr,
}
fn argparse() -> Arg {
    loop {
        let input: i32 = loop {
            println!(
                "Choose an Option:
    \n1. Convert temperatures between Fahrenheit and Celsius.
    \n2. Generate the nth Fibonacci number.
    \n3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas“
    "
            );
            print!(">>>");
            io::stdout().flush().expect("Failed to flush stdout!");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line!");
            match input.trim().parse() {
                Ok(num) => {
                    if num >= 1 && num <= 3 {
                        break num;
                    }
                    println!("{} is not a valid option!", num);
                }
                _ => {
                    println!("Please enter a number!");
                }
            }
        };
        match input {
            1 => return Arg::Temp,
            2 => return Arg::Fib,
            3 => return Arg::Chr,
            _ => continue,
        }
    }
}
fn temperature() {}
fn fibonacci() {}
fn christmas_carol() {}
