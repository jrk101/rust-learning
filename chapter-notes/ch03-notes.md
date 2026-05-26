# Chapter 3 - Variables, Functions, and Control Flow

## Constants

- They are always immutable
- We declare constants using const instead of let
- The type must be annotated

const PI_VALUE: u32 = 3.14;

---

## Shadowing

We can declare a new variable with the same name as a previous variable.

Shadowing vs mut:
- let mut x = 5;
  x = 6;        // just change the value

- let x = 5;
  let x = 6;    // shadowing creates a new variable
                // we can also change the type

The mut is not scope aware. But in case of shadowing it is.
mut type doesn't allow type change. Shadowing does.

---

## Data Types

Two types:
1) Scalar
2) Compound

---

## 1) Scalar Type - represents single value

### a) Integers
Types:
- Signed   - i8, i16, i32, i64, i128, isize
- Unsigned - u8, ...  usize

### b) Floating Point
Types - f32, f64
All are signed.
Default is f64.

### c) Numeric Operations
Add, Sub, Mul, Division, Remainder

### d) Boolean
Using bool

let t = true;
let f: bool = false;

### e) Characters
let c = 'C';
It can also store emojis

---

## 2) Compound Types - can group multiple values into one type

### a) Tuple
- Fixed length
- Group values of different type into one compound type

Creating a tuple:
let tup: (i32, f64, u8) = (500, 5.1, 10);

Accessing values from tuple:

1) Pattern matching:
let tup = (5, 6.3, 1000);
let (x, y, z) = tup;

2) Direct method using index value:
let tup = (5, 10, 12);
let five = tup.0;

### b) Array Type
- Store multiple values of same type
- Fixed length

Initialize:
let a = [1, 2, 3, 4];
let a: [i32; 5] = [1, 2, 3, 4, 5];
(5 indicates the number of elements)

Initialize to contain same values of each element:
let a = [3; 5]   // same as
// let a = [3, 3, 3, 3, 3]

Accessing the element:
let a = [1, 2, 3, 4, 5];
let first = a[0];

---

## Functions

Passing arguments to the parameters:

fn second_function(x: i32) {
    println!("The value of x is: {x}");
}
fn main() {
    second_function(x);
}

In function signature we must declare the type of parameters.
While defining multiple parameters, it can be separated by commas.

fn main() {
    function_2("Manu", 20);
}
function_2(name: &str, age: i32) {
    println!("Name: {name}, Age: {age}");
}

---

## Statements and Expressions

Statements - Instructions that perform some action and do not return a value.
Expressions - Evaluate to a resultant value.

let x = {
    let y = 4    // x = 4
    y
}
(we cannot write let x = let y = 4, let is a statement and it doesn't return a value)

---

## Functions with Return Values

eg1:
fn five() -> i32 {
    5
}
fn main() {
    let x = five();
}

eg2:
fn main() {
    let x = five(6);
}
fn fun(x: i32) -> i32 {
    x - 1
}

---

## Control Flow

### if Expressions

eg1:
let n = 5;
if n > 3 {
    println!("The condition is true");
}
else {
    println!("The condition is false");
}

eg2:
if condition {
    ...
} else if condition {
    ...
} else {
    ...
}

### Using if in a let statement

let condition = true;
let number = if condition { 5 } else { 6 };

The type of if and else arm must be same.