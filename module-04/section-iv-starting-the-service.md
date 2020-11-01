# Section IV - starting the service

We are now ready to start the RESTful service that will listen for data that needs to be sourced and feeds it to the event flow.

 There are 2 ways to start the service.

1. Running using `cargo run` command while developing \(local service testing\)

> NOTE: we provide the argument `--bin myapp_sourcing` because there are now multiple executables and must specify which one to run.

```text
PS C:\workspace\rust-daas> cargo run --bin myapp_sourcing
    Finished dev [unoptimized + debuginfo] target(s) in 8.66s
     Running `target\debug\myapp_sourcing.exe`
```

To stop the service, use `ctrl` + `c`.

   2. Running using the executable.

```text
PS C:\workspace\rust-daas> cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.37s
```

Whenever you use the `cargo build` command, it places the created executable in the target/debug directory with the same name that was defined in the `Cargo.toml` manifest.

Since it is an executable, simple run the executable from the command terminal.

> NOTE: Example below is for Windows.

```text
C:\workspace\rust-daas\target\debug> .\myapp_sourcing.exe
```

