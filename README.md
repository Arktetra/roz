# roz

An interpreter written in Rust.

## Current Progress

The interpreter can now interpret variable declaration and expressions.

```roz
let a = "alpha";
{
    let a = "beta";
    print a;
}
print a;
```
