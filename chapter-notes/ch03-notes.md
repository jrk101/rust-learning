# Chapter 3 - Data Types

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

---

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