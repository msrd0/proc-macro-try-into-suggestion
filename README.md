# Compiler Output

rust/cargo 1.47.0, command `cargo check`

```
    Checking proc-macro-try-into-suggestion v0.0.0 (/home/msrd0/git/proc-macro-try-into-suggestion)
error[E0308]: mismatched types
 --> src/main.rs:3:10
  |
3 | #[derive(DeriveMacro)]
  |          ^^^^^^^^^^^ expected `usize`, found `i32`
  |
  = note: this error originates in a derive macro (in Nightly builds, run with -Z macro-backtrace for more info)
help: you can convert an `i32` to `usize` and panic if the converted value wouldn't fit
  |
3 | #[derive((DeriveMacro).try_into().unwrap())]
  |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
error: could not compile `proc-macro-try-into-suggestion`.

To learn more, run the command again with --verbose.
```
