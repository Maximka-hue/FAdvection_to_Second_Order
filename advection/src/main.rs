//As this training task for first steps learning language, 
//DESIGNATIONS will be following:📔
//E!xt- Doesn't import crate(extension)
//C!ircumvent- desire to do smth advanced to avoid ...e.g. creating temp value, etc.(et cetera)
//W!ork - Doesn't work
//D!esire - I would like to use it, but didn't find appropriate method/way to use for it) 
//*********************************************************************
//My libraries 
//extern crate time_measure;
//extern crate input_structure;
#[macro_use]
use input_structure::initial_data_utils::function_utils::print_macros::macro_lrls::{pt};
pub use input_structure::initial_data_utils::initial_input_structures::{TaskType, TaskTypeCs, FileParametres, initial_information_of_advection};
#[macro_use]
extern crate colour;
#[macro_use] 
extern crate tcprint;
#[macro_use]
extern crate colorify;
extern crate clap;
use clap::{Arg, App, SubCommand};
///These imports from library as I already downloaded these crates)
use input_structure::function_utils::cfutils::{ColorPrintState};
use std::{env, io::{self, Write}};
//Determine in cycle all provided for constants arguments, otherwise default.
mod determine_my_impls{
    pub const INITIAL_INFO_ABOUT_PROGRAMM: bool = false;
    pub const ARGUMENT_PARSING: bool = false;
}

fn all_colours() {
    black!("black ");
    red!("red ");
    green!("green ");
    yellow!("yellow ");
    blue!("blue ");
    magenta!("magenta ");
    cyan!("cyan ");
    white!("white ");
    //dark_black!("dark_black ");
    dark_red!("dark_red ");
}
//Then will be blocks that only for me to understand rust!(Maybe you will do another initializations,....)
//Further mark it like ))) and my own implementations of crates like *******....
//))) green for paths and debug, 
fn main() {
///Here I am defining by default colored struct with task type
    let mut state = ColorPrintState::<TaskTypeCs>::default();
    tcprintln!(state,
         (""),
         [burger_cs: "{}", "OOO"],
         ("!"),("!")
    );
    all_colours();
///From this point I will determine file hierarchy on which output and input files will be. 
initial_information_of_advection();
    let env_path = env::current_dir().unwrap();
    green!("{:#?}", env_path);
}
