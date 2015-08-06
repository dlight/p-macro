# p-macro

Tired of typing `{:?}`? This crate defines a macro `p!()` shorter to
type than `println!()`, for your debugging needs.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
p-macro = "*"
```

And use like this:

```rust
#[macro_use] extern crate p_macro;

fn main() {
    p!(5); // same as println!("{:?}", 5);
}
```

It does other things too:

```rust
    p!(10, "a"); // same as println!("{:?} {:?}", 10, "a");
```

```rust
    p! {
        1, 2, 3;  // same as:  println!("{:?} {:?} {:?}", 1, 2, 3);
        4, 5, 6;  //           println!("{:?} {:?} {:?}", 4, 5, 6);
    };
```

Strings are printed with quotes. If this is undesirable, prefix them
with an _. This will make the macro use `Display` instead of `Debug`.

```rust
    p!(_"Test"); // same as println!("{}", "Test");
	let a = "x"; p!(_ a); // if necessary insert a space
```

To run the file on the examples folder, type `cargo run --example
print`.
