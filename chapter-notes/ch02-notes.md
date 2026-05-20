# Chapter 2 - Guessing Game

## Receiving User Input

```rust
let mut guess = String::new();
```

- `let` declares an immutable variable by default
- `let mut` makes a variable mutable
- `String::new()` creates a new empty string

---

## To Accept an Input from User

1. Import io library: `use std::io;`
2. `stdin()` - creates a handle to read from the keyboard
3. `read_line(&mut guess)` - reads input from user and stores it to `guess`

Here we use `&mut guess` to pass only the reference to `read_line` function.

If we pass `guess` to `read_line`, it will take full ownership and the `guess` cannot be used after that.

`read_line` returns `Result` type, that's why we use `.expect()`

---

## To Convert the String to Actual Number

1. Read the string input:
```rust
let mut guess = String::new();
```

2. Redeclare `guess` into integer (shadowing):
```rust
let guess: u32 = guess.trim().parse().expect("Enter a number");
```

- `trim()` removes `\n` and white spaces from front and end
- `parse()` converts string to integer