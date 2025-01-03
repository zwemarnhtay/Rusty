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

## Tuples

Tuple is a collection of values of different data types and it has **fixed length**: once declared, they cannot grow or shrink in size.

Tuples can be created like below:

```Rust
let tup1 = ('A', 32, -99, 78.5, false);

let tup2 (char, u8, i8, f32, bool) = ('A', 32, -99, 78.5, false);

let tup3 = ('A', 32u8, -99i8, 78.5f32, false);
```

The first index in tuple is `0` and we can take tuple's member by using a period `.`.

```rust
let first_member = tup1.0; // A
```

## Array

Array must have **the same data type** for its element and Rust's array have **fixed length**.

Array can be created below:

```rust
let arr1 = ['a', 'b', 'c'];

let arr2: [i32; 5] = [123, 456, 789, 11, 321];
```

Array that contain same value for all elements can be created below:

```rust
let same_val_arr: [3; '🦀'] //['🦀', '🦀', '🦀']
```

We can use `arr[0]` to take first element in array.