## Observations

* The keyword `mut` enables modification of value but the type of the variable must remain same through out the block.
```rust
let mut var = String::new();
var = 10; // Compilation error: mismatched type
```
* To achieve variable shadowing one must redeclare the variable
```rust
let mut var = String::new();
let mut var = 10;
```

* In lesser words, to "change the type of a variable", I should "shadow" (redeclare) the variable.

* Are we rebinding the variable here?

* Could this result in bugs? [discussion](https://mail.mozilla.org/pipermail/rust-dev/2013-May/004293.html)
