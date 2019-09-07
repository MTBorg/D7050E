#[macro_use]
pub mod debug{
    macro_rules! debug_print{
        ($x:expr) => {
            println!("{:#?}", $x);
        };
    }
}
