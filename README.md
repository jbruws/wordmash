# wordmash

Crate that allows arithmetic operations on strings by representing them as base36 numbers. 

This is done through the `Masher` struct, which can construct base36 numbers from all types implementing `Mashable` trait and perform arithmetics by reverting them back to base10. 

Right now, `Masher` only accepts unsigned integers, strings and string slices. The latter two must be in all uppercase and only contain numbers (0-9) and English letters. Here's an example:

```rust
use wordmash::masher::Masher;

fn main() {
    let mut initial: Masher = Masher::new("COLD").unwrap();
    let words: Vec<&str> = vec!["SLICE", "VAULT", "ZEN", "FACT", "OUNCE"];
    for i in words {
        initial = initial + Masher::new(i).unwrap();
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

Be careful! Although multiplication is supported, it frequently results in panics due to overflow. Use it with caution.
