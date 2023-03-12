fn main() {
    loop_as_expr();
    println!("---");
    loop_labels();
    println!("---");
    while_loop();
    println!("---");
    for_loop();
    println!("---");
    countdown_using_for_loop();
}

fn loop_as_expr() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}");
}

fn loop_labels() {
    let mut count = 0;

    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }

            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }

    println!("End count = {count}");
}

fn while_loop() {
    let mut number = 3;

    while number != 0 {
        println!("{number}!");
        
        number -= 1;
    }

    println!("LIFTOFF!");
}

fn for_loop() {
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }
}

fn countdown_using_for_loop() {
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}