# Chapter 03

## Variables

```rust
let x = 5; // Immutable variable
let mut y = 6; // Mutable variable
const PI: f32 = 3.14; // Constant
```

### Constants

- The type of the constant value must be annotated.
- Constant can be declared in any scope.
- Constant may be set only to a [constant expression](https://doc.rust-lang.org/reference/const_eval.html) 

### Shadowing

```rust
fn main() {
    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // The value of x in the inner scope is: 12
    }

    println!("The value of x is: {x}"); // The value of x is: 6
}
```

The main point of shadowing is the ability to reuse the same name for a variable,
which can be useful when we have a user's input from stdin in a string form that
we can transform into any type we need:

```rust
let mut guess = String::new();

std::io::stdin()
  .read_line(&mut guess)
  .expect("Failed to read line");

let guess: u32 = guess.trim().parse().expect("Please type a number!");
```

## Data types

### Scalar types

- Integer types (i8 - i128, isize, u8 - u128, usize)
- Floating types (f32, f64) 
- Boolean
- Char (use single quotes) - it's 4 bytes in size and represents a unicode scalar value 

[Integer Overflows](./i-overflows/src/main.rs)

### Compound types

TODO