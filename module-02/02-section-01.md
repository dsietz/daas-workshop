# Section I - Create a Package

In your main terminal at the bottom of the IDE, run the following command to ensure we are in the `environment` directory.

```
cd $HOME/environment
```

Let first make sure the Rust is install correctly.

```
ArchConfWorkshopUser:~/environment $ rustup -V
rustup 1.24.3 (ce5817a94 2021-05-31)
```

```
ArchConfWorkshopUser:~/environment $ cargo -V
cargo 1.56.0 (4ed5d137b 2021-10-04)
```

```
ArchConfWorkshopUser:~/environment $ rustup default stable
info: using existing install for 'stable-x86_64-unknown-linux-gnu'
info: default toolchain set to 'stable-x86_64-unknown-linux-gnu'

  stable-x86_64-unknown-linux-gnu unchanged - rustc 1.56.1 (59eed8a2a 2021-11-01)
```

Create the `rust-daas`package

```
cargo +stable new rust-daas --lib
```

```
ArchConfWorkshopUser:~/environment $ cargo +stable new rust-daas --lib
     Created library `rust-daas` package
```

Change directory to the rust-daas project.

```
cd rust-daas/
```

cargo has generated ...

```
.
|-- src
     |-- lib.rs
|-- Cargo.toml
```
