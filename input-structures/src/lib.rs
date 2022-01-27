//This lib will implement initial interaction in programm(command-line, basic functions, etc.)
#[macro_use] 
extern crate tcprint;
#[macro_use]
extern crate colorify;
#[macro_use]
extern crate colour;
pub mod initial_data_utils;
pub use crate::initial_data_utils::function_utils::cfutils::{self, op_sys};
pub use crate::initial_data_utils::initial_input_structures::{TaskType, TaskTypeCs, FileParametres,FileParametresBuilder, initial_information_of_advection};
pub use crate::initial_data_utils::function_utils::print_macros::macro_lrls;
use std::time::{Instant};
use chrono::{Local};
extern crate rand;
use rand::{prelude::*, Rng, SeedableRng};
