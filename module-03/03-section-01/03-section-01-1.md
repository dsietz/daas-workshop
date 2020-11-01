# Section I - manifest

> [Cargo.toml](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/Cargo.toml)

Because there are SDKs for the [DaaS](https://crates.io/crates/daas) pattern as well as [Privacy by Design](https://crates.io/crates/pbd), we will not need to build out the underlying _object wrappers,_ _data model_ support, or logic to implement common _privacy strategies_.

First, we need to include the dependent crates into the project. Add the following lines to the `[dependencies]` section in the `Cargo.toml` file

```text
serde ="1.0"
serde_derive = "1.0"
serde_json = "1.0"
daas = "0.1.0"
pbd = "0.2.0"
```

