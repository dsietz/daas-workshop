# Section I - Create a Package

Let first make sure the Rust is install correctly.

```text
[user@localhost workspace]$ rustup -V
rustup 1.18.3 (435397f48 2019-05-22)
[user@localhost workspace]$ cargo -V
cargo 1.35.0 (6f3e9c367 2019-04-04)
[user@localhost workspace]$ rustup default stable
info: using existing install for 'stable-x86_64-pc-windows-msvc'
info: default toolchain set to 'stable-x86_64-pc-windows-msvc'

  stable-x86_64-pc-windows-msvc unchanged - rustc 1.35.0 (3c235d560 2019-05-20)
```

Create the _rust-daas_ package

```text
[user@localhost workspace]$ cargo +stable new rust-daas --lib
     Created library `rust-daas` package
[user@localhost workspace]$ cd rust-daas
[user@localhost rust-daas]$
```

cargo has generated ...

```text
.
|-- .git
|-- .gitignore
|-- src
     |-- lib.rs
|-- Cargo.toml
```

