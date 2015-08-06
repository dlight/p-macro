#[macro_export]
macro_rules! __p_c {
    ($y:expr) => { print!("{:?}", $y); }
}

#[macro_export]
macro_rules! __p_u {
    ($y:expr) => { print!("{}", $y); }
}

#[macro_export]
macro_rules! __p_b {
    ($y:expr) => { print!("{} = {:?}", stringify!($y), $y); }
}

#[macro_export]
macro_rules! p {
    () => {
        println!("");
    };


    (: $y:expr) =>  { __p_c!($y); p!(); };
    (: $y:expr,) => { __p_c!($y); p!(); };
    (: $y:expr;) => { __p_c!($y); p!(); };

    (: $y:expr, $($x:tt)+) => {
        __p_c!($y);
        print!(" ");
        p!($($x)*);
    };

    (: $y:expr; $($x:tt)+) => {
        __p_c!($y);
        p!();
        p!($($x)*);
    };


    (_ $y:expr) =>  { __p_u!($y); p!(); };
    (_ $y:expr,) => { __p_u!($y); p!(); };
    (_ $y:expr;) => { __p_u!($y); p!(); };

    (_ $y:expr, $($x:tt)+) => {
        __p_u!($y);
        print!(" ");
        p!($($x)*);
    };

    (_ $y:expr; $($x:tt)+) => {
        __p_u!($y);
        p!();
        p!($($x)*);
    };


    ($y:expr) =>  { __p_b!($y); p!(); };
    ($y:expr,) => { __p_b!($y); p!(); };
    ($y:expr;) => { __p_b!($y); p!(); };

    ($y:expr, $($x:tt)+) => {
        __p_b!($y);
        print!(", ");
        p!($($x)*);
    };
    
    ($y:expr; $($x:tt)+) => {
        __p_b!($y);
        p!();
        p!($($x)*);
    };
}
