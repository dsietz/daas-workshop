# Section IV - manifest

> [Cargo.toml](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/Cargo.toml)

We already have the binary file defined in the manifest file, but there are dependent packages that we will need to include in order to make it a RESTful service. 

In the **\[dependencies\]** section of the `Cargo.toml` file add the following packages.

```rust
actix-web = "3"
```



