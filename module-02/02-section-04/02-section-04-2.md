# Section IV - library

> [lib.rs](https://github.com/dsietz/daas-workshop/blob/master/rust-daas/src/lib.rs)

The lib.rs file is our centralized library file. This file is where globally shared functions, constants and variables are located. It is also where we include modules that make up the library.

**Remove** the test module section of code at the bottom of the file. This is where your integrated testing \(module dependencies\) would be tested to ensure the library if working as expected. However, we will be ignoring this level of testing for the purpose of this workshop.

```text
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
```

At the very top of the file, add the external web service crate \(package\)

```text
extern crate hyper;
extern crate actix_web;
```

In top portion of the file \(where the global variables would be located after the **extern crates** and **use** sections\), add the following global variable.

```text
static VER: &str = "v1";
```

At the bottom of the file, add the following module as part of the library.

```text
pub mod hello_world;
```

> **NOTE**: If you build the code right now, it would raise an error that the module cannot be found.
>
> ```text
> error[E0583]: file not found for module `hello_world`
>  --> src/lib.rs:6:1
>   |
> 6 | pub mod hello_world;
>   | ^^^^^^^^^^^^^^^^^^^^
>   |
>   = help: to create the module `hello_world`, create file "src\hello_world.rs"
>
> error: aborting due to previous error
>
> For more information about this error, try `rustc --explain E0583`.
> error: could not compile `rust-daas`.
>
> To learn more, run the command again with --verbose.
> warning: build failed, waiting for other jobs to finish...
> error: build failed
> ```



