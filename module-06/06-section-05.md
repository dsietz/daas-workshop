# Section V - testing

> [rust-daas.postman\_collection.json](https://github.com/dsietz/rust-daas/blob/master/tests/rust-daas.postman_collection.json)

On the command line, run the `cargo run --bin reporting` command to start the service. \(Or open a new command terminal and start the service using the executable in the target/debug directory\).

```text
PS C:\workspace\rust-daas> cargo run --bin reporting
    Finished dev [unoptimized + debuginfo] target(s) in 0.39s
     Running `target\debug\reporting.exe`
```

Now you can open the `report data` POST request in Postman and monitor the logging on the command line of the running service.

> **Note** If you get an error message stating "Could not broker document", then make sure to go into the `lib.rs` file and check the value of the KAFKA\_BROKERS variable.

The response payload should be something similar to the following:

```text
{
    "rows": [
        {
            "key": "acknowledge",
            "value": 108
        },
        {
            "key": "new",
            "value": 4
        },
        {
            "key": "shipped",
            "value": 0
        }
    ]
}
```
