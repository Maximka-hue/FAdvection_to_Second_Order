//This lib will implement initial interaction in programm(command-line, basic functions, etc.)
#[macro_use] 
extern crate tcprint;
#[macro_use]
extern crate colorify;
#[macro_use]
extern crate colour;
pub mod initial_data_utils;
pub use crate::initial_data_utils::*;
use std::time::{Instant};
use chrono::{Local};
