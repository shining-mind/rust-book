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

#### Tuple

```rust
let tup: (i32, f64, u8) = (500, 6.4, 1);

// Destructuring
let (mut x, mut y, mut z) = tup;

// Access via element position
x = tup.0;
y = tup.1;
z = tup.2;
```

#### Array

```rust
let arr: [i32; 5] = [1, 2, 3, 4, 5];

arr[0]; // 1
arr[2]; // 3
```

Shorthand for creating an array:

```rust
let arr = [3; 5]; // [3, 3, 3, 3, 3]
```

Rust performs runtime check when program tries to access an arbitrary index:

```rust
let arr = [1, 2, 3];

let index = 3;

arr[index]; // panic!
```

Use for loop to iterate the collection instead of while loop with index as it 
doesn't add runtime check for the index.

## Functions

### Statements and expressions

* **Statements** are instructions that perform some action and do not return a value.
* **Expressions** evaluate to a resultant value. Let’s look at some examples.

If you add a semicolon (;) to the end of the expression it will become a statement:

```rust
fn foo() -> u32 {
  let y: u32 = x + 1; // Statement
  y + 1 // Expression which will be returned as the result of the function
}
```

It's an expression and a syntactic scope:

```rust
{ /* ... */ } 
```

## Control flow

`if` is an expression:

```rust
let x = 3;
let y = if x > 3 {
  true
} else {
  false
};

println!("{y}") // false
```

