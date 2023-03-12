fn main() {
    let number = 3;
        
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    if_as_expr();
}

fn if_as_expr() {
    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}