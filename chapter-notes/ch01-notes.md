# Chapter 1 - Getting Started with Rust

## Why Rust?

Every programming language makes a tradeoff:

- Python, JS - easy, but slow. It is also safe
- C, C++ - fast but dangerous (memory bugs, crashes, security vulnerabilities...)

Rust solves this problem - fast like C, C++, safe like Python

The Rust compiler catches the bug even before the execution of the program

---

## Running a Rust Program (without Cargo)

- Save file as `main.rs`
- Compile → `rustc main.rs`
- Execute → `.\main`

---

## Cargo - Rust's Build System and Package Manager

**Build System:** When we write Rust code we need to manually compile like above without Cargo.
With Cargo we can just do this with:
```bash
$ cargo run
```

**Package Manager:** The extra crates (packages) we will use in our program, we don't need to manually download it. We can just use Cargo.

```bash
$ cargo build
```
Cargo automatically downloads and installs it.

---

## Create a Project with Cargo

```bash
$ cargo new project_name
```

The new directory `project_name` will be created with two files:

1. **Cargo.toml** - Project's configuration file
   - `[package]` - basic info about project: name, version, edition
   - `[dependencies]` - allows us to add external crates

We can build and run the project using:
```bash
$ cargo run
```

Cargo is compatible with all OS