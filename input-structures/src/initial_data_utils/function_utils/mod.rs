#[macro_export]
pub mod print_macros;
pub(crate) use print_macros::*;
pub mod cfutils;
pub(crate) use cfutils::*;
pub mod custom_colours;
pub(crate) use custom_colours::{PrimaryColor, ForegroundColor, BackgroundColor};
