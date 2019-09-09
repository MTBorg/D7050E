#[macro_use]
#[allow(unused_macros)]
pub mod debug{
    macro_rules! debug_print{
        ($x:expr) => {
            println!("{:#?}", $x);
        };
    }
}
