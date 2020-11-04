# Section I - Create a Package

In your main terminal at the bottom of the IDE, run the following command to ensure we are in the `environment` directory.

```text
cd $HOME/environment
```

Let first make sure the Rust is install correctly.

```text
ArchConfWorkshopUser:~/environment/rust-daas (master) $ rustup -V
rustup 1.22.1 (b01adbbc3 2020-07-08)
ArchConfWorkshopUser:~/environment/rust-daas (master) $ cargo -V
cargo 1.47.0 (f3c7e066a 2020-08-28)
ArchConfWorkshopUser:~/environment/rust-daas (master) $ rustup default stable
info: using existing install for 'stable-x86_64-unknown-linux-gnu'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.47.0 (18bf6b4f0 2020-10-07)
```

Create the `rust-daas`package

```text
[user@localhost workspace]$ cargo +stable new rust-daas --lib
     Created library `rust-daas` package
[user@localhost workspace]$ cd rust-daas/
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

