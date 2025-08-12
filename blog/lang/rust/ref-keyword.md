# Overview

The `ref` keyword allows you unpack a pattern match item as a reference.  
Normally, pattern matched items are moved.

```rs
let opt: Option<Vec<i32>> = Some(Vec<i32>::new());
match opt {
    // so here, vec is a reference
    Some(ref vec) => {},
    None => {}
}
```

It would seem more natural to do something like this.

```rs
Some(&vec)
```

However, in Rust, only the right-hand side of an expression should have an `&`.

```rs
let x = 10;
let y = &x; // we don't write let &y
```
