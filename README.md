# 30days2rust

My goal here is to learn Rust in 30 Days Through 30 Projects of Increasing Complexity

### Creating a project
Run ```cargo new my_project```
```
my_project/
├── Cargo.toml
└── src/
    └── main.rs       # Hello World by default

```

### Compilation
Even if its not the proper usage, every mini-project has its own Makefile compiling the project different ways (if possible).
The need for crates is such that rustc will quickly be abandoned in Makefiles in favor of cargo.
```
$> rustc main.rs
Hello World!

# OR

$> cargo new my_project
$> cd my_project
$> cargo build
$> ./target/debug/my_project
Hello World!

# OR

$> cargo new my_project
$> cd my_project
$> cargo build
$> cargo run
Hello-World!
```
<p align="center">
  <img src="https://www.rust-lang.org/logos/rust-logo-blk.svg" width="120" alt="Rust Logo">
</p>
