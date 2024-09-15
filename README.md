# roz

An interpreter written in Rust.

## Current Progress

Place the following code in a file <`filename`>:

```roz
fn say(something) {
    print "---------------------------------------";
    print something;
    print "---------------------------------------";
}

say("Return statements are left to be added.");
```

Use the following to run the interpreter on <`filename`>:

```bash
cargo run filename
```

The output should be as follows:

```bash
---------------------------------------
Return statements are left to be added.
---------------------------------------
```
