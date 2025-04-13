# Unsafe to Verified

This simple proc macro was inspired by a tweet by HSVSphere, suggesting that the better name for unsafe was verified, which I totally agree with.


## Motivation
Unsafe carries a stigma, implying "dangerous" or "broken" when, in reality, it’s just Rust code that requires manual verification of certain invariants (e.g., memory safety, type correctness). By using verified, you’re highlighting the trust in the programmer’s diligence, which can make the code more approachable, especially in teams or projects where unsafe might scare off contributors.



## Usage

To add unsafe to a function definition
```rust
#[verified_item]
pub fn some_unsafe_code()
```


Unsafe blocks within a function
```rust
pub fn unsafe_code_inside() {
    verified!{
        //do unsafe thing
    }
}
```

Also works for impl blocks