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
extern crate time; 
pub use input_structure;
use input_structure::initial_data_utils::{Path,PathBuf, function_utils::print_macros::macro_lrls::{pt}};
use input_structure::initial_data_utils::{parse_into_file_parameters};
#[warn(unused_imports)]
use input_structure::cfutils::{ChooseSleepTime, ColorPrintState, ArgumentParseFilesError, op_sys};
use input_structure::{TaskType, TaskTypeCs, FileParametres, FileParametresBuilder, initial_information_of_advection, is_dir_hidden, advection_input};
#[macro_use]
extern crate colour;
#[macro_use] 
extern crate tcprint;
#[macro_use]
extern crate ansi_term;
extern crate num_cpus;
extern crate rand;
extern crate env_logger;
extern crate log;
use log::{debug, error, info, warn};
extern crate walkdir;
use walkdir::{DirEntry, WalkDir};
use gtk::prelude::*;
use gio::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label};
pub use ansi_term::{Colour::{Fixed, Black as AnsiBlack, Red as AnsiRed, Green as AnsiGreen, Yellow as AnsiYellow, Blue as AnsiBlue, Purple as AnsiPurple, 
    Cyan as AnsiCyan, Fixed as AnsiFixed}, Style as AnsiStyle};
///These imports from library as I already downloaded these crates)
use std::{env, fs, io::{self, Write}};
use time::Duration;
#[warn(unused_imports)]
use rand::{distributions::{Distribution, Uniform}, prelude::*};
//use std::rand::{task_rng, Rng};
//Determine in cycle all provided for constants arguments, otherwise default.
mod determine_my_impls{
    pub const INITIAL_INFO_ABOUT_PROGRAMM: bool = false;
    pub const MY_ARGUMENT_PARSING: bool = false;
}
mod determine_calculation_modes{
pub const PATH_DEBUG_INFO: bool = true;
pub const PATH_CREATION: bool = true;
pub const LETS_DO_PAUSE: bool = true;
pub const GENERATE_RANDOM_EXAMPLE: bool = false;
pub const RANDOM_PATH_CREATION: bool = false;
pub const GET_FILES_FROM_DIRECTORY: bool = false;
}
mod modifications{
pub const RANDOM_TRANSLATE_MARGINE_BOUNDARY: bool = true;
pub const MAXIMUM_FILES_TO_EXPECT: usize = 6;
}

//Then will be blocks that only for me to understand rust!(Maybe you will do another initializations,....)
//Further mark it like ))) and my own implementations of crates like *******....
//))) blue for paths and debug, 
pub use determine_my_impls::*;
pub use determine_calculation_modes::*;
use modifications::*;
type StdResult<T> = std::result::Result<T, String>;//Box<dyn Error>
fn main()  {//-----------------------------------------
    let application = Application::new(
        Some("com.github.rust-ui-rundown.rust-ui-gtk"),
        Default::default(),
    );//.expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("This is advection programm!");
        window.set_default_size(700, 200);

        let container = Box::new(gtk::Orientation::Vertical, 10);

        let label = Label::new(None);
        let button = Button::with_label("Click me!");

        container.add(&label);
        container.add(&button);
        window.add(&container);

        button.connect_clicked(move |_| {
            let _ = &label.set_label("Hello, World!");
        });
        initial_information_of_advection();
        window.show_all();
    });
    application.run();
    let began_advection = Duration::ZERO;
    //5.seconds().saturating_add(5.seconds() Duration::nanoseconds(
    let mut time_counter = ChooseSleepTime::add_default_time();
//-----------------------------------------
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
//----------------------------------------- 
///First I need to determine places where to save input images, datas, etc.
    let env_path = env::current_dir().unwrap();
    let advection_path = env_path.join("src");
    let animation_path = advection_path.join("animation");
    let calculation_path = animation_path.join("datas");
    let photos_path = animation_path.join("photos");
    let output_path = advection_path.join("OutputFiles");
    let log_output_path = output_path.join("logging/advec_log.txt");
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
    println!("burger_order: {}", burger_order);
    if PATH_CREATION {
        fs::create_dir_all(&animation_path).unwrap(); 
        fs::create_dir_all(&calculation_path).unwrap(); 
        fs::create_dir_all(&photos_path).unwrap(); 
        if !directory_with_examples_exists {
        fs::create_dir_all(&input_fpath).unwrap();
        println!("{}", Fixed(221).on(Fixed(124)).paint(format!("{input_fpath:?}Is it now directory? {}" , fs::canonicalize(&input_fpath).unwrap_or(PathBuf::new()).is_dir())));
        }
        if output_path.try_exists().expect("Can't check existence of OuputFiles directory[tex and log]"){
            fs::create_dir_all(&log_output_path).unwrap();
            println!("{}", Fixed(221).on(Fixed(124)).paint(format!("file for Logging created at {:?}", &log_output_path)));
        }
        else {
            fs::create_dir_all(&output_path).unwrap();
        }
    }
    if RANDOM_PATH_CREATION {
        if directory_with_examples_exists{
            pt!("\nDirectory for examples already exists\n");
            fs::create_dir_all(&input_fpath.join("random_examples")).unwrap();
        }
        else{
            fs::create_dir_all(&input_fpath).unwrap();
        }
    }
//Then I am initializing structure that would be passed as initial datas for program, but!
    let mut dataf = FileParametres::first_initializing(1).expect("Something wrong in Initializing");//It is as default for program
//There are options: 1 generate from file[GENERATE_RANDOM_EXAMPLE= false or datas from file will be illigal]
//(in that case supported Transfer task)
//2 from txt files which *will be from input path getted *collected from command line *from file[their paths].
// **Command line can be processed by hand-made parser into struct Argumento or with clap
    if GENERATE_RANDOM_EXAMPLE {    
        parse_into_file_parameters(RANDOM_TRANSLATE_MARGINE_BOUNDARY);
    }
    else{
        //Get txt with datas
        if GET_FILES_FROM_DIRECTORY{
            let mut all_txt: Vec<PathBuf> = Vec::new();
            let walker = WalkDir::new(&input_fpath).into_iter();
            for entry in walker.filter_entry(|e| !is_dir_hidden(e)) {
                all_txt.push(PathBuf::from(entry.unwrap().path().clone()));
            }
                    //First is directory itself
            let all_txt = all_txt[1..MAXIMUM_FILES_TO_EXPECT+1usize].to_vec();
            if PATH_DEBUG_INFO{ 
                for path_txt in all_txt{
                    println!("{}", path_txt.display());}
                }
        }
        else{
            if MY_ARGUMENT_PARSING{
                //process it by myself
            }
            else{
                //with clap
                //cargo run -- -output-style -file-paths input-pstructures/src/advec_examples/TransferBurgerMccornack_iconditions00.txt
                let from_cli= advection_input();
                //home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/input-pstructures/src/advec_examples/TransferBurgerMccornack_iconditions0.txt
                //input-pstructures/src/advec_examples/TransferBurgerMccornack_iconditions0.txt input-pstructures/src/advec_examples/TransferBurgerMccornack_iconditions00.txt 
            }
        }
    }
}


/*
    let possible_error = FileParametresBuilder::default()
        .eq_type(0).time_eval_period_stage((0_f64 , 10_f64, Some(false)))
        .build().unwrap_err();
    println!("\n{}", &possible_error.to_string());*/
/* 
extern crate once_cell;
extern crate log4rs;

use log::{info, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
use clap::{Arg, App, SubCommand};
use std::fmt::{Debug, Display};//For boundary in struct's types
use tutil::crayon::Style;
use std::iter;
use std::fs;
use std::path::PathBuf;
use tutil::crayon::Color::{Red, Blue};
use ansi_term::{self};
use ansi_term::Colour::Fixed;
use std::{env, io};
use once_cell::sync::OnceCell;
use walkdir::WalkDir;
use std::fs::OpenOptions;
use std::{sync::Mutex, collections::HashMap};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    Ok(())
}
fn test<T: AsRef<str>>(inp: &[T]) {
    for x in inp { print!("{} ", x.as_ref()) }
    println!("");
}
use ansi_term::{ANSIString, ANSIStrings};
use ansi_term::ANSIByteStrings;
let some_value = format!("{:b}", 42);
let strings: &[ANSIString<'static>] = &[
    ansi_term::Colour::Yellow.paint("["),
    ansi_term::Colour::Red.bold().paint(some_value),
    ansi_term::Colour::Green.paint("]"),
];
println!("Value: {}", ANSIStrings(strings));
println!("This will be {} and this will be {}.",
         Style::new().foreground(Red).bold().paint("red and bold"),
         Style::new().foreground(Blue).italic().paint("blue and italic"));
ansi_term::Colour::Green.paint("user data".as_bytes()).write_to(&mut std::io::stdout()).unwrap();

ANSIByteStrings(&[
    Green.paint("user data 1\n".as_bytes()),
    Green.bold().paint("user data 2\n".as_bytes()),
]).write_to(&mut std::io::stdout()).unwrap();
let dirs = dirs.map(|file| file.unwrap().path());

// now, our iterator just for our config file
let config = iter::once(PathBuf::from("foorc"));

// chain the two iterators together into one big iterator
let files = dirs.chain(config);

// this will give us all of the files in .foo as well as .foorc
for f in files {
    println!("{:?}", f);
}

    // Gets a value for config if supplied by user, or defaults to "default.conf"
    let config = matches.value_of("config").unwrap_or("default.conf");
    println!("Value for config: {}", config);

    // Calling .unwrap() is safe here because "INPUT" is required (if "INPUT" wasn't
    // required we could have used an 'if let' to conditionally get the value)
    println!("Using input file: {}", matches.value_of("INPUT").unwrap());

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("v") {
        0 => println!("No verbose info"),
        1 => println!("Some verbose info"),
        2 => println!("Tons of verbose info"),
        3 | _ => println!("Don't be crazy"),
    }

    // You can handle information about subcommands by requesting their matches by name
    // (as below), requesting just the name used, or both at the same time
    if let Some(matches) = matches.subcommand_matches("test") {
        if matches.is_present("debug") {
            println!("Printing debug info...");
        } else {
            println!("Printing normally...");
        }
    }
    let num_str = matches.value_of("num");
match num_str {
    None => println!("No idea what your favorite number is."),
    Some(s) => {
        match s.parse::<i32>() {
            Ok(n) => println!("Your favorite number must be {}.", n + 5),
            Err(_) => println!("That's not a number! {}", s),
        }
    }
}
let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();

    log::info!("Hello, world!");   // more program logic goes here...

}     
//fn check_dirs_and_fill()

//extern crate chrono;
#![feature(map_into_keys_values)]
extern crate env_logger;
use std::num::{self};
//extern crate num-traits;
//extern crate retainer;
//extern crate cashed;
pub use std::sync::{Arc, Mutex};
//use std::thread;//use std::include_str;
pub use chrono::{self, DateTime, Duration, Local, Utc};//,DateTime, FixedOffset, Utc};
pub use std::fmt::{Debug, Display};//For boundary in struct's types
pub use std::collections::HashMap;
pub use env_logger::{Builder, Target};
pub use num_traits::{Num, NumCast};
pub use std::io::Write;
use std::thread;
use log::{info, warn};
use log::{Record, Level, Metadata, LevelFilter, SetLoggerError};

pub use std::error::Error as StdError;
type StdResult<T> = std::result::Result<T, Box<dyn StdError>>;//dyn for dynamic return when 
//we don't know before execution what type will be returned(Also simply error without classification)

//use retainer::cache::Cache; Only in async
//use cached::proc_macro::cached;
use std::marker::PhantomData;
///For following purposes:(Controlling every needed parts of code)
///1. begin when the main func launch (*by default)
///2. after, update each time as needed to take in account next block of logic.
///Will add difference between one and previous in second member
///and log expired times for blocks

#[derive(Debug, Clone)]
pub struct GlobalExpiredTime<T: NumCast, V>
//First-  time of entry in block (Local or UTC)
//Second- key: number of block (or associated name) (Option- maybe you don't want to take it into account)
//        value: time count (on whether it was done in threads or simply run the same block twise, thrice, fourfold etc.)
    (Arc<Mutex<Vec<DateTime<Local>>>>, HashMap<Option<T>, V>, );

impl<'i, K, V> GlobalExpiredTime<K, V> where
//K must be the number! of block in program counted times, 
K: Copy + Sync + 'static + NumCast + std::cmp::Ord + std::hash::Hash + std::fmt::Debug,//, TimeSpec:
V: Sync +std::fmt::Debug + Clone {
    fn new(t_kind: Option<String>) -> GlobalExpiredTime<K, V>{
        if let Some(kind_of_time) = t_kind {
            //.chars().flat_map(char::to_uppercase).collect::<String>();
            if kind_of_time.to_uppercase()=="UTC" {
                let now: DateTime<Utc> = Utc::now();
                println!("UTC now is: {}, so you had instantiated time in program TBC_eq(TransferBurguerCorrection_eq)", now.to_rfc2822());
            }
            else if kind_of_time.to_uppercase()=="LOCAL" {
                let now: DateTime<Local> = Local::now();
                println!("Local time now is: {}, so you had instantiated time in program TBC_eq(TransferBurguerCorrection_eq)", now.to_rfc2822());
            }
        }
            let mut date_vec = Vec::<DateTime<Local>>::new();
            date_vec.push(Local::now());
            Self(
                Arc::new(Mutex::new(date_vec)), HashMap::new()
            )
        
    }
    fn details(&mut self, detailed_output: Option<u8> ){
        let lt = &Local::now();
        self.0.lock().expect("Error with access to mutex").first()
            .unwrap_or_else(|| lt);// Not good- temporary local
        if !self.1.is_empty(){
                while let Some(mut detail_counter) = detailed_output{
                    println!("Your choice- detailed output: {}\r", detail_counter);
                         while !self.0.lock().unwrap().is_empty() && !detail_counter== (0|255){
                            let entry_t= self.0.lock().expect("Error with access to mutex in nested block!");
                            let elements = entry_t.iter();//Also
                            let mut annotation = self.1.values();
                                for (k,el) in elements.enumerate(){ 
                                    println!("Time {element:>width1$} : №{number:>0width2$}- {hash_annotation:?}, Location of code block: {}",
                                    number= k,
                                    width2= 2,
                                    element= el,
                                    width1= 4,
                                    hash_annotation= annotation.next().unwrap());
                                    detail_counter-=1;
                                }
                         } 
                    }
                }
        else { 
            let elements = self.0.lock().expect("Error with access to mutex");
            let size = (*elements).len();
            for (k,el) in elements.iter().enumerate(){                    
                println!("Time block {}- {element:>width1$}", k,
                 element= el,
                 width1=size +1);
            }
        }
    }
    fn update_new(&mut self){
        let mut next_block= self.0.lock().expect("Error with access to mutex");
        let lt = Local::now();
        next_block.push(lt);
    }
    fn loc_block(&self)-> DateTime<Local>{
        *self.0.lock().expect("Error with access to mutex").last().expect("Last doesn't exist")
    }
    fn update_next_block(&'static mut self, print_flog: Option<(bool, Option<(bool, bool)>)> ){// logger: Log
//Check if there is something counted already earlier otherwise initialize
        //let mut gvec = &self.0.lock()
            //.unwrap_or_else(|_| GlobalExpiredTime::new(None).0.lock().expect("Internal break in mutex!"));
//Check the penultimate one
        let old_key_num= self.1.keys().max_by_key(|&key| key).unwrap();
    I want! .unwrap_or_else(| | match K::type{
                i32 => &Some(K.is_zero()),
                i64 => &Some(1),
                bool => std::process::exit(1),
                _ => println!("Type don't match requirements"),
            });//: Vec<&K> 
        let last_num_block = self.1.get(&old_key_num); 
            //.collect().max().iter().max_by(|a, b| a.partial_cmp(b).unwrap());
        const N: usize = 4;
        if let Some((pprint, llog)) = print_flog {
        //let mut threads = Vec::<chrono::Duration>::with_capacity(N);
           if pprint{
//will print last N  
                (0..N).for_each(|seq_num| { // <---- Closure 1
                //access last N in vec
                let arc_clone = Arc::clone(&self.0);
                let size_vec= arc_clone.lock().unwrap().len();
                    thread::spawn(move || {  // <---- Closure 2
                        let loc_clone = arc_clone.lock().unwrap();
                        let cur_loc= loc_clone.get(size_vec- seq_num as usize).unwrap();
                        let lt = &Local::now();
                        let next_loc= loc_clone.get(size_vec- seq_num +1 as usize).unwrap_or_else(|| lt);
                        let utc_time = DateTime::<Utc>::from_utc(next_loc.naive_utc(), Utc);
                        let dif_locs = next_loc.signed_duration_since(*cur_loc);
                        println!("Element {} was executed in {:?}\n",  size_vec - seq_num, dif_locs);
                        //threads.push(dif_locs);
                   });
               });
            }
            if let Some(should_log)= llog{
                let mut log_file = std::fs::File::create("log_time.txt").expect("create failed");
                env_logger::init();
                Builder::new()
                    .target(Target::Stdout)
                    .init();
                log::set_max_level(LevelFilter::Error);
                let arc_clone = Arc::clone(&self.0);
                let mut block_info = (self.1.keys(), self.1.values());
                let size_vec= arc_clone.lock().unwrap().len();
                let mut file = std::fs::OpenOptions::new().append(true).open("log_time.txt").expect(
                    "cannot open file");
                thread::spawn(move || {
                    use std::str;
                    let mut cur: usize= 0;
                    while let Some(vec_elem)= arc_clone.lock().unwrap().get(size_vec-cur){
                        println!("Block number {0:?} executed in {1:?}", block_info.0.next().unwrap().unwrap(),//excessive, i know)
                            block_info.1.next().unwrap());
                            //file.write_all(format!("Block number {:?} executed in {:?}\n".as_bytes(), String::from_str(block_info.0.next().unwrap().unwrap()).as_bytes(),
                            //block_info.1.next().unwrap().as_bytes()));
                            log::info!("Written {} time", cur);
                            cur+= 1;
                    };
                });
            }
        }//end log
    }
}
impl<K, V> log::Log for GlobalExpiredTime< K, V> where
//K must be the number! of block in program counted times, 
K: Sync + Send + NumCast + std::cmp::Ord + std::hash::Hash + std::fmt::Debug,//, TimeSpec:
V: Sync + Send + std::fmt::Debug + Clone {
    fn enabled(&self, metadata: &Metadata) -> bool {
       metadata.level() <= Level::Info
    }
    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("Rust says: {} - {}", record.level(), record.args());
        }
    }
    fn flush(&self) {}
}        
     
}
    let b=
    if let time= OldMeasureTime{
        let gr_zero: bool=  ContinueMeasureTime.sub(time) > chrono::Duration::zero();
        gr_zero
    }
    else{false};
    if b{
        let ExpiredTime= ContinueMeasureTime- time;}
    else {ContinueMeasureTime= self.0}
    self.0 + time}
trait SplitTimeOnUnits{
    //fn 
}
fn main()-> Result<(), log::SetLoggerError>{
Ok(())    //let n= test_timing();
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_timing(){
        let builder = Builder::new()
            .is_test(true).filter_level(LevelFilter::Info);//.format_timestamp(Some(chrono::offset::Local::now()));
        println!("{:?}", chrono::offset::Local::now());
        println!("{:?}", chrono::offset::Utc::now());
        let mut  time: super::GlobalExpiredTime<u32, String>= GlobalExpiredTime::new(Some("UTC".to_owned()));
        const BLOCKS: usize= 10;
        for i in 0.. BLOCKS{
            time.update_new();
        }
        time.details(Some(1 as u8));
        let accuracy: i64= 1_i64;
        assert!(time.loc_block().signed_duration_since(Local::now()) < 
            Duration::microseconds(accuracy.pow(2)));
    }
} 
*/
