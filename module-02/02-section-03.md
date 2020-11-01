# Section III - Creating an Executable

Although this package was created as a library, we can make it dual binary and library by doing the following:

1. Modify the `Cargo.toml` file to point to a binary during build time

```rust
[[bin]]
name = "hello_world"
path = "src/bin/hello-world.rs"
```

1. Create a new **bin** folder in the **src** directory
2. Create a new file `hello-world.rs` in the **bin** directory with the following code

```rust
pub fn main() {
    println!("Hello World");
}
```

1. Build and run the package 

```text
[user@localhost rust-daas]$ cargo run
   Compiling rust-daas v0.1.0 (C:\workspace\rust-daas)
    Finished dev [unoptimized + debuginfo] target(s) in 0.67s
     Running `target\debug\hello_world.exe`
Hello World
```

> There should now be an executable named **hello\_world** in the /target/debug directory which you can execute directly.

```text
[user@localhost rust-daas]$  cd .\target\debug\
[user@localhost debug]$ ./hello_world
Hello World
```

