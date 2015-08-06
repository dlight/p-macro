#[macro_export]
macro_rules! p {
    () => {
        println!("");
    };
    
    (_ $y:expr) => {
        println!("{}", $y);
    };
    
    
    (_ $y:expr,) => {
        println!("{}", $y);
    };
    
    (_ $y:expr;) => {
        println!("{}", $y);
    };

    (_ $y:expr, $($x:tt)+) => {
        print!("{} ", $y);
        p!($($x)*);
    };
    
    (_ $y:expr; $($x:tt)+) => {
        println!("{}", $y);
        p!($($x)*);
    };

    ($y:expr) => {
        println!("{:?}", $y);
    };

    ($y:expr,) => {
        println!("{:?}", $y);
    };
    
    ($y:expr;) => {
        println!("{:?}", $y);
    };

    ($y:expr, $($x:tt)+) => {
        print!("{:?} ", $y);
        p!($($x)*);
    };
    
    ($y:expr; $($x:tt)+) => {
        println!("{:?}", $y);
        p!($($x)*);
    };
}
