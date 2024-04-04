# Chapter 02

## Notes

```rust
let bananas = 5; 
let mut bananas = 5; // Mutable variable
```
`::` - access the associated function of the type, i.e.: `String::new()`

`String` - is a growable UTF-8 string

`&` - a reference

`&mut` - a mutable reference

`Result` - is an enumeration, it is a type which can be in the `Ok` or `Err` state

It's possible to interpolate the variable into the string:

```rust
let x = 5;
let y = 10;

println!("x = {x} and y + 2 = {}", y + 2); // x = 5 and y + 2 = 12
```