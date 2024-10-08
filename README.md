# roz

An interpreter written in Rust.

## Current Progress

Place the following code in a file <`filename`>:

```rust
fn info() {
    print "============================";
    print "Mutually Recursive Functions";
    print "============================";
}

fn is_odd(n) {
    if (n == 0) return false;
    return is_even(n - 1);
}

fn is_even(n) {
    if (n == 0) return true;
    return is_odd(n - 1);
}

info();

for (let i = 0; i <= 10; i = i + 1) {
    print i + " => " + is_odd(i);
}
```

Use the following to run the interpreter on <`filename`>:

```bash
cargo run filename
```

The output should be as follows:

```bash
============================
Mutually Recursive Functions
============================
0 => false
1 => true
2 => false
3 => true
4 => false
5 => true
6 => false
7 => true
8 => false
9 => true
10 => false
```
