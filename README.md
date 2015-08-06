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
	let x = 5;
	let y = 2;
    p!(x + y); // same as println!("x + y = {:?}", x + y);
	p!(x, y); // same as println!("x = {:?}, y = {:?}", x, y);
	p!(); // same as println!("");
}
```

It does other things too. With a colon you disable printing the source
code snippet:

```rust
p!(:10, :"a"); // same as println!("{:?} {:?}", 10, "a");
```

It accepts multiple lines too:

```rust
let (a, b, c) = (1, 2, 3);

p! {
    a, b, c;
    a + b, 0, 1;
};
// same as:
// println!("a = {:?}, b = {:?}, c = {:?}", a, b, c);
// println!("a + b = {:?}, 0 = {:?}, 1 = {:?}", a + b, 0, 1);
```

Strings are printed with quotes. If this is undesirable, prefix them
with an _. This will make the macro use `Display` instead of `Debug`.

```rust
p!(_"Test"); // same as println!("{}", "Test");
let a = "x"; p!(_ a); // if necessary insert a space
```

To run the file on the examples folder, type `cargo run --example
print`.
