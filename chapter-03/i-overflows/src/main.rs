fn main() {
    let mut x: u8 = 255;

    x = x.wrapping_add(1); // 256 -> 0, 257 -> 1, etc.

    println!("255.wrapping_add(1) = {x}"); // x = 0

    x = 255;

    let (mut x, overflow) = x.overflowing_add(2);

    println!("255.overflowing_add(2) = {x}, has overflow: {overflow}"); // x = 1, overflow = true

    x = 255;

    x = x.saturating_add(1);

    println!("255.saturating_add(1) = {x}"); // x = 255

    x = 255;

    match x.checked_add(1) {
        None => println!("255.checked_add(1) = None"), 
        Some(_) => {},
    };

}
