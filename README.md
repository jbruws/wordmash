# wordmash

Crate that allows arithmetic operations on strings by representing them as base36 numbers. 

This is done through the `Masher` struct, which can construct base36 numbers from all types implementing `Mashable` trait and perform arithmetics by reverting them back to base10. On the "backend", wordmash uses `rug` crate for arbitrary-precision integers (after all, base36 numbers in decimal tend to be very large).

Right now, `Masher` only accepts unsigned integers, strings and string slices. The latter two must only contain numbers (0-9) and English letters. Here's an example:

```rust
use wordmash::masher::Masher;

fn main() {
    let mut initial: Masher = Masher::new("COLd").unwrap();
    let words: Vec<&str> = vec!["SLICE", "vault", "zeN", "FACT", "OUnCE"];
    for i in words {
        initial += Masher::new(i).unwrap();
        println!("{}", initial);
    }
}
```

Running this will result in

```
SY6XR
1O91JK
1OA0Y7
1OPBB0
2DJYNE
```
being written to the terminal.
