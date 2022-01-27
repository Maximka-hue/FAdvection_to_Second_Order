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
use input_structure::initial_data_utils::function_utils::print_macros::macro_lrls::{generate_random_parameters, pt};
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
use itertools::Itertools;
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
    let input_fpath = env_path.parent().expect("Determine input file's path").join("input-structurs").join("src").join("advec_examples/");
    if PATH_DEBUG_INFO {
        assert_eq!(&env_path.as_path().display().to_string(), "/home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/advection");
        println!("{prog}  {env_path:#?}\n{anim}  {advection_path:#?}\n\
        {calc}  {calculation_path:#?}\n{photo}  {photos_path:#?}\n\
        {input_txt}  {input_fpath:#?}\n\
        ",
        prog=  AnsiGreen.blink().paint("Programm start location:"), photo=  AnsiGreen.bold().paint("Photos path:"),
        anim= AnsiGreen.bold().paint("Animation storage"), calc=  AnsiGreen.bold().paint("Calculation data:"), 
        input_txt= AnsiGreen.bold().paint(""));
        println!("Number of threads on your Computador: {num_threads}\n\
        You entered directory that {} to exist and {} to be directory",
        input_fpath.try_exists().expect("Can't check existence of file does_not_exist.txt"), input_fpath.is_dir());
    }
    println!("{}", burger_order);
    if PATH_CREATION{
        fs::create_dir_all(&animation_path).unwrap(); 
        fs::create_dir_all(&calculation_path).unwrap(); 
        fs::create_dir_all(&photos_path).unwrap(); 
        if !input_fpath.try_exists().expect("Can't check existence of file does_not_exist.txt") && input_fpath.is_dir(){
        fs::create_dir(&input_fpath).unwrap();
        }
    }
//Then I am initializing structure that would be passed as initial datas for program
    let mut dataf = FileParametres::first_initializing(3).expect("Something wrong in Initializing");
    let (eq_type, bound_type, init_type, add_args, time_eval_period_stage, init_conditions , margin_domain, quantity_split_nodes, n_corant):
    (i8, i8, i8, i8, (f64, f64), f64, f64, f64, f64);
    let random_parameters = generate_random_parameters!().unwrap();
    let int_input = random_parameters.0;
    let float_input = random_parameters.1;
    assert_eq!(int_input.len(), 4);
    assert_eq!(float_input.len(), 10);
    if let Some((req_type, rbound_type, rinit_type, radd_args)) = int_input.into_iter().tuples().next(){
        eq_type= req_type; bound_type= rbound_type; init_type= rinit_type; add_args= radd_args;
    };
    if let Some((rtime_eval_period_stage_one, rtime_eval_period_stage_two, rinit_conditions , rmargin_domain, rquantity_split_nodes, rn_corant)) = float_input.into_iter().tuples().next(){
        //eq_type= req_type; bound_type= rbound_type; init_type= rinit_type; add_args= radd_args;
        time_eval_period_stage = (rtime_eval_period_stage_one, rtime_eval_period_stage_two);
        println!("{:?}", time_eval_period_stage);
    };
    if GENERATE_RANDOM_EXAMPLE{

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
