Shows some examples of issues that static analysis tools for C/C++ can find,
but rust/clippy can't.

- Correctness, security, halting, concurrency and performance issues. 
- For standards like MISRA, AUTOSAR, and OWASP.

You can see rust and clippy does not report any issues with this code.
Note, it could be other tools detect these issues statically.
Just at the time of writing rust and clippy do not.


```shell
cargo build
cargo clippy
```

See source for explanations of the issues: https://github.com/illume/static_analysis_c_not_rust/blob/main/src/main.rs
