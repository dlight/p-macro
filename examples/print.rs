#[macro_use] extern crate p_macro;

#[derive(Debug)]
struct Point { x: i32, y: i32 }

fn main() {
    let a = [0, 1, 2, 3, 4, 5];
    
    let b = ["This", "is", ""];

    let point = Point { x: 100, y: -50 };

    p!(:a; :b);

    p!();

    p!(point.x, point.y + 30);
    
    p!(_ b[0], _"=>", a[2] * 42);

    p! {
        _"The value is:", point.x + a[2];
        b[0], :a[2], :a[3], :a[1]
    };
}
