# roz

An interpreter written in Rust.

## Current Progress

Place the following code in a file <`filename`>:

```rust
fn say(something) {
    print "---------------------------------------";
    print something;
    print "---------------------------------------";
}

say("We can return something too.");

fn return_something(something) {
    return something;
}

print return_something("Returning return from return_something");
```

Use the following to run the interpreter on <`filename`>:

```bash
cargo run filename
```

The output should be as follows:

```bash
---------------------------------------
We can return something too.
---------------------------------------
Returning return from return_something
```
