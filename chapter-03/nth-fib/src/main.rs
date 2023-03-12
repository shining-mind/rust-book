use std::io::{stdin};
use std::collections::{HashMap};

fn main() {
    let mut cache: HashMap<u32, u128> = HashMap::new();
    
    println!("Enter n");
    loop {
        match prompt_input() {
            Some(n) => {
                let result = nth_fib(n, &mut cache);
                if result > 1e9 as u128 {
                    println!("-> {n}th fib = {result:.3e}");
                } else {
                    println!("-> {n}th fib = {result}");
                }
            },
            None => break,
        }
    }
}

fn nth_fib(n: u32, cache: &mut HashMap<u32, u128>) -> u128 {
    if let Some(cached) = cache.get(&n) {
        return *cached
    }
    if n <= 1 {
        n.into()
    } else {
        let result = nth_fib(n - 1, cache) + nth_fib(n - 2, cache);
        cache.insert(n, result);
        result
    }
}


fn prompt_input() -> Option<u32> {
    let mut input = String::new();

    stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if let Ok(n) = input.trim().parse() { Some(n) } else { None }
}