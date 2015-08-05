#[macro_use] extern crate p_macro;

fn main() {
    let a = [0, 1, 2, 3, 4, 5];
    
    let b = ["This", "is", ""];

    p!(a; b);
    
    p!(_ b[0], _":", a[2] * 42);
    
    p! {
        _"Never changes", _ a[0], a[0];
        b[1], b[2];
        "Qq";
    };
}
