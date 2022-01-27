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
#![feature(path_try_exists)]
#[macro_use]
pub use input_structure;
use input_structure::initial_data_utils::{ Path, PathBuf, function_utils::print_macros::macro_lrls::{pt}};
use input_structure::initial_data_utils::{parse_into_file_parameters};
use input_structure::cfutils::{ColorPrintState, ArgumentParseFilesError, op_sys};
use input_structure::{TaskType, TaskTypeCs, FileParametres, FileParametresBuilder, initial_information_of_advection};
#[macro_use]
extern crate colour;
#[macro_use] 
extern crate tcprint;
#[macro_use]
extern crate colorify;
extern crate os_type;
extern crate ansi_term;
extern crate num_cpus;
extern crate rand;
pub use ansi_term::{Colour::{Black as AnsiBlack, Red as AnsiRed, Green as AnsiGreen, Yellow as AnsiYellow, Blue as AnsiBlue, Purple as AnsiPurple, 
    Cyan as AnsiCyan, Fixed as AnsiFixed}, Style as AnsiStyle};
///These imports from library as I already downloaded these crates)
use std::{env, fs, io::{self, Write}};
use rand::{distributions::{Distribution, Uniform}, prelude::*};
//use std::rand::{task_rng, Rng};
//Determine in cycle all provided for constants arguments, otherwise default.
mod determine_my_impls{
    pub const INITIAL_INFO_ABOUT_PROGRAMM: bool = false;
    pub const ARGUMENT_PARSING: bool = false;
}
mod determine_calculation_modes{
pub const PATH_DEBUG_INFO: bool = true;
pub const PATH_CREATION: bool = true;
pub const LETS_DO_PAUSE: bool = true;
pub const GENERATE_RANDOM_EXAMPLE: bool = true;
pub const RANDOM_PATH_CREATION: bool = false;
}
mod modifications{
pub const RANDOM_TRANSLATE_MARGINE_BOUNDARY: bool = true;
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
//))) blue for paths and debug, 
pub use determine_my_impls::*;
pub use determine_calculation_modes::*;
use modifications::*;
fn main() {
//Here I am defining by default colored struct with task type
    let mut burger_rng = rand::thread_rng();
    let mut burger_order: u16 = burger_rng.gen_range(0..3);
    let mut state = ColorPrintState::<TaskTypeCs>::default();
    tcprintln!(state,
         (""),
         [burger_cs_first_order: "{}", "OOO"],
         ("!"),("!")
    );
//General instruction about operating sys and environment variables
    op_sys();
    let num_threads = num_cpus::get();
//From this point I will determine file hierarchy on which output and input files will be. 
initial_information_of_advection();
///First I need to determine places where to save input images, datas, etc.
    let env_path = env::current_dir().unwrap();
    let advection_path = env_path.join("src");
    let animation_path = advection_path.join("animation");
    let calculation_path = animation_path.join("datas");
    let photos_path = animation_path.join("photos");
    let input_fpath = env_path.parent().expect("Determine input file's path").join("input-pstructures").join("src").join("advec_examples/");
    let directory_with_examples_exists = input_fpath.try_exists().expect("Can't check existence of file does_not_exist.txt");
    let is_example_dir: bool = if directory_with_examples_exists { input_fpath.is_dir()} else{ false};
    if PATH_DEBUG_INFO {
        assert_eq!(&env_path.as_path().display().to_string(), "/home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/advection");
        println!("{prog}  {env_path:#?}\n{anim}  {advection_path:#?}\n\
        {calc}  {calculation_path:#?}\n{photo}  {photos_path:#?}\n\
        {input_txt}  {input_fpath:#?}\n\
        ",
        prog=  AnsiGreen.blink().paint("Programm start location:"), photo=  AnsiGreen.bold().paint("Photos path:"),
        anim= AnsiGreen.bold().paint("Animation storage"), calc=  AnsiGreen.bold().paint("Calculation data:"), 
        input_txt= AnsiGreen.bold().paint(""));
        if !directory_with_examples_exists{
        println!("Number of threads on your Computador: {num_threads}\n\
        You entered path {input_fpath:?}\n that {} to exist",
        directory_with_examples_exists);
        }
        else if directory_with_examples_exists && !is_example_dir{
            println!("You entered path {input_fpath:?}\n that exists but {} to be directory\n\tPlease enter another path", is_example_dir);

        } 
    }
    println!("{}", burger_order);
    if PATH_CREATION {
        fs::create_dir_all(&animation_path).unwrap(); 
        fs::create_dir_all(&calculation_path).unwrap(); 
        fs::create_dir_all(&photos_path).unwrap(); 
        if !directory_with_examples_exists {
        fs::create_dir(&input_fpath).unwrap();
        println!("{input_fpath:?}Is it now directory? {}" , fs::canonicalize(&input_fpath).unwrap_or(PathBuf::new()).is_dir())
        }
    if RANDOM_PATH_CREATION {
            if directory_with_examples_exists{
    
            }
            else{
                    //fs::create_dir().unwrap();
            }
    
            }
    }
//Then I am initializing structure that would be passed as initial datas for program, but!
    let mut dataf = FileParametres::first_initializing(3).expect("Something wrong in Initializing");
//There are options: generate from file[GENERATE_RANDOM_EXAMPLE= false or datas from file will be illigal] and from txt files.
if GENERATE_RANDOM_EXAMPLE{    
    parse_into_file_parameters(RANDOM_TRANSLATE_MARGINE_BOUNDARY);
    }
    else{
        //Get txt with datas
    }
    let possible_error = FileParametresBuilder::default()
        .eq_type(0).time_eval_period_stage((0_f64 , 10_f64, Some(false)))
        .build().unwrap_err();
    println!("\n{}", &possible_error.to_string());

}
/* 
*/
