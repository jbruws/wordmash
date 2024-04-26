# wordmash

Crate that allows arithmetic operations on strings by representing them as numbers in a specialised Masher alphabet. 

This is done through the `Masher` struct, which can construct "numbers" from all types implementing `Mashable` trait and perform arithmetics by reverting them back base10. On the "backend", wordmash uses `rug` crate for arbitrary-precision integers (after all, Masher numbers in decimal tend to be very long).

Right now, `Masher` only accepts unsigned integers, strings and string slices. The latter two must only contain uppercase English letters and symbols `'_-,.!?`, as well as spaces (` `). Here's an example:

```rust
use wordmash::masher::Masher;

fn main() {
    let mut initial: Masher = Masher::new("THIS IS A LONGER STRING").unwrap();
    let words: Vec<&str> = vec!["EYES", "OF", "CORPORATE INSIGHT"];
    for i in words {
        let new = Masher::new(i).unwrap();
        initial += new;
        println!("{}", initial);
    }
}
```

Running this will result in

```
THIS IS A LONGER STV?RY
THIS IS A LONGER STV?!,
THIS IVNSOZ!NZJRH?D.FFO
```
being written to the terminal.
