# Data Types

Rust is static typed language and we have to tell compiler what data types we used. Rust has string `"abc"`, character `'A'`, integer `2025`, float `3.14` and boolean `true` or `false`

---

## Integers

Rust has signed integer type `i8` and unsigned integer type `u8`. In this case, sign refers to `+` or `-` and unsigned integer type is only used for always positive cases. `i32` is default for integers.

`i8` can store numbers from -(2<sup>7</sup>) to 2<sup>7</sup> - 1, which equals -128 to 127 and then, `u8` can store numbers from 0 to 2<sup>8</sup> - 1, which equal 0 to 255.

- signed type => -(2<sup>n-1</sup>) to 2<sup>n-1</sup> - 1 
- unsigned type => 0 to 2<sup>n</sup> -1

## Floating points

Rust has only two primitive types: `f32` and `f64` for floating points then, default is  `f64`.