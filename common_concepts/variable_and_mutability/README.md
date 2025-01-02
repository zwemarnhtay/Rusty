# Variable & Mutability

In Rust, **variables are immutable** which mean they wouldn't be assigned twice

```rust
let x = 1;
x = 2;  // we got compiler error "cannot assign twice to immutable variable `x`"
```

---

But, we can use ```mut``` keyword to make mutable variables

```rust
let mut x = 2024;
x = 2025; // there is no more compiler error here
```