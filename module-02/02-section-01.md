# Section I - Create a Package

In your main terminal at the bottom of the IDE, run the following command to ensure we are in the `environment` directory.

```text
cd $HOME/environment
```

Let first make sure the Rust is install correctly.

```text
[user@localhost workspace]$ rustup -V
rustup 1.22.1 (b01adbbc3 2020-07-08)
[user@localhost workspace]$ cargo -V
cargo 1.47.0 (f3c7e066a 2020-08-28)
[user@localhost workspace]$ rustup default stable
info: using existing install for 'stable-x86_64-pc-windows-msvc'
info: default toolchain set to 'stable-x86_64-pc-windows-msvc'

  stable-x86_64-pc-windows-msvc unchanged - rustc 1.47.0 (18bf6b4f0 2020-10-07)
```

Create the `rust-daas`package

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

