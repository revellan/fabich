use std::io;
use std::io::Write;
use std::process;
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
        let input: i32 = get_input(
            "Choose an Option:\n
    1. Convert temperatures between Fahrenheit and Celsius.
    2. Generate the nth Fibonacci number.
    3. Print the lyrics to the Christmas carol “The Twelve Days of Christmas“
    4. Exit\n\n>>> ",
            4,
        );
        match input {
            1 => return Arg::Temp,
            2 => return Arg::Fib,
            3 => return Arg::Chr,
            4 => process::exit(0),
            _ => continue,
        }
    }
}
fn get_input(msg: &str, max_options: i32) -> i32 {
    loop {
        print!("{}", &msg);
        io::stdout().flush().expect("Failed to flush stdout!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        match input.trim().parse() {
            Ok(num) => {
                if num >= 1 && num <= max_options {
                    break num;
                }
                println!("\n{} is not a valid option!\n", num);
            }
            _ => {
                println!("\nPlease enter a number!\n");
            }
        }
    }
}
fn get_int(msg: &str) -> u64 {
    loop {
        print!("{}", &msg);
        io::stdout().flush().expect("Failed to flush stdout!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        match input.trim().parse() {
            Ok(num) => {
                break num;
            }
            _ => {
                println!("\nPlease enter a number!\n");
            }
        }
    }
}
fn get_float(msg: &str) -> f64 {
    loop {
        print!("{}", &msg);
        io::stdout().flush().expect("Failed to flush stdout!");
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");
        match input.trim().parse() {
            Ok(num) => {
                break num;
            }
            _ => {
                println!("\nPlease enter a number!\n");
            }
        }
    }
}
fn temperature() {
    match get_input(
        "\nChoose an Option:
    1. Fahrenheit to Celsius
    2. Celsius to Fahrenheit
    3. Exit\n\n>>> ",
        3,
    ) {
        1 => f_to_c(),
        2 => c_to_f(),
        3 => process::exit(0),
        _ => panic!("Fatal Error 0!"),
    }
}
fn f_to_c() {
    let fahrenheit = get_float("\nEnter the Temperature in Fahrenheit!\n\n>>> ");
    let celcius = (fahrenheit - 32.0) * (5.0 / 9.0);
    println!("{}°F is {}°C", fahrenheit, celcius);
}
fn c_to_f() {
    let celcius = get_float("\nEnter the Temperature in Celsius!\n\n>>> ");
    let fahrenheit = celcius / (5.0 / 9.0) + 32.0;
    println!("{}°C is {}°F", celcius, fahrenheit);
}
fn fibonacci() {
    let out: String = loop {
        match get_int("\nEnter the Index of the Fibonacci Number that should be printed!\n\n>>> ") {
            0 => {
                println!("The 0th Fibonacci Number doesn't exist, it starts at 1!");
                continue;
            }
            1 => {
                break String::from("The first Fibonacci Number is: 0");
            }
            2 => {
                break String::from("The 2nd Fibonacci Number is: 1");
            }
            3 => {
                break String::from("The 3rd Fibonacci Number is: 1");
            }
            a => {
                let fib_index = a;
                let mut fib_num: u64 = 1;
                let mut prev_fib_num: u64 = 0;
                let mut tmp_fib_num: u64;
                for _ in 3..=fib_index {
                    tmp_fib_num = prev_fib_num;
                    prev_fib_num = fib_num;
                    fib_num = match (fib_num).checked_add(tmp_fib_num) {
                        Some(nm) => nm,
                        None => {
                            println!("Overflow was prevented. Exiting...");
                            process::exit(0);
                        }
                    }
                }
                break String::from(format!(
                    "The {}th Fibonacci Number is: {}",
                    fib_index, fib_num
                ));
            }
        };
    };
    println!("{}", out);
}
fn christmas_carol() {
    const DAYS: usize = 12;
    let days: [&str; DAYS] = [
        "first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth",
        "tenth", "eleventh", "twelfth",
    ];
    let lines: [&str; DAYS] = [
        "Twelve drummers drumming",
        "Eleven pipers piping",
        "Ten lords a-leaping",
        "Nine ladies dancing",
        "Eight maids a-milking",
        "Seven swans a-swimming",
        "Six geese a-laying",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves and",
        "A partridge in a pear tree",
    ];
    for i in 0..DAYS {
        println!(
            "\nOn the {} day of Christmas, my true love sent to me",
            days[i]
        );
        for n in DAYS - (i + 1)..DAYS {
            println!("{}", lines[n]);
        }
    }
}
