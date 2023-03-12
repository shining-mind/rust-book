use std::io::{stdin};
use regex::Regex;

fn main() {
    println!("Enter temperature in Farenheits or Celsius, i.e.: 10C");
    loop {
        match prompt_input() {
            None => {
                println!("Invalid input!");
                continue;
            },
            Some((value, measure)) => {
                let mut result_measure = 'C';
                let result = if measure == 'F' {
                    convert_to_celsius(value)
                } else {
                    result_measure = 'F';
                    convert_to_farenheits(value)
                };
                if result.fract() > 0.0 {
                    println!(" -> {result:.2}{result_measure}");
                } else {
                    println!(" -> {result}{result_measure}");
                }
                break;
            }
        }
    }
}

fn convert_to_farenheits(celsius: i32) -> f64 {
    celsius as f64 * (9.0 / 5.0) + 32.0
}

fn convert_to_celsius(farenheits: i32) -> f64 {
    (farenheits - 32) as f64 * (5.0 / 9.0)   
}

fn prompt_input() -> Option<(i32, char)> {
    let mut input = String::new();

    let re = Regex::new(r"^(-?\d+)(F|C)$").unwrap();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match re.captures(input.trim()) {
        Some(groups) => {
            let mut result: (i32, char) = (0, '.');

            if let Some(group) = groups.get(1) {
                result.0 = group.as_str().parse().unwrap();
            }
            if let Some(group) = groups.get(2) {
                result.1 = group.as_str().chars().next().unwrap();
            } 
            Some(result)
        },
        _ => None,
    }

}
