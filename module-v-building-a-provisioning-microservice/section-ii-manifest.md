# Section II - manifest

> [Cargo.toml](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/Cargo.toml)

Let's begin by declaring a new executable for the service that will act as the _order.clothing provisioning service_. We will do this by adding a `[[bin]]` section to our `Cargo.toml`manifest file.

```text
[[bin]]
name = "myapp_order_clothing"
path = "src/bin/order_clothing.rs"
```

Because we are not in need of any additional cretes, we don't need to make any changes to the `[dependencies]` section in the `Cargo.toml` file
