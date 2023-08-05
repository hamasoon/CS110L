# Memory Safety in Rust

## Problem of C/C++ : Why is it to easy to screw up in C?
- Dangling Pointers
- Double Free
- Iterator Invalidation
- Memory Leak

## Rust's Solution

#### Ownership
- Each value in Rust has a variable that’s called its owner.
- Each "onwer" has the responsibility of cleaning up its value when it goes out of scope.
- When ownership move from one scope to another, the first owner can no longer be used.

#### Borrowing
- Basically shared reference of a value.
- Only one mutable reference can exist at a time.

#### Lifetime
- The lifetime of a variable starts when it is created and ends the last time it is used.
- Rust computes the lifetime of a variable using a system called Lifetime Elision Rules.(in a compile time)

- By computng all ownership. memory and lifetime in compile time, rust make fixed cont investment