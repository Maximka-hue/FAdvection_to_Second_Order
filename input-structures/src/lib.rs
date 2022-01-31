//This lib will implement initial interaction in programm(command-line, basic functions, etc.)
#[warn(unused_imports)]
#[macro_use] 
extern crate tcprint;
extern crate colorify;
#[macro_use]
extern crate colour;
extern crate colored;
#[macro_use]
extern crate clap;
use colored::Colorize;
pub mod initial_data_utils;
pub use crate::initial_data_utils::{PathBuf,Path, function_utils::cfutils::{self,run, parse_pair, parse_three, Argumento, op_sys, parse_positive_int}};
pub use crate::initial_data_utils::initial_input_structures::{TaskType, TaskTypeCs,BurgerOrder, FileParametres, FileParametresBuilder, initial_information_of_advection};
use crate::initial_data_utils::function_utils::print_macros::macro_lrls;
use rustils::parse::boolean::str_to_bool;
//use std::time::{Instant};
//use chrono::{Local};
use tutil::crayon::Style;
use tutil::crayon::Color::*;
extern crate rand;
use rand::{prelude::*, Rng, SeedableRng};
pub use structopt::StructOpt;
use clap::{ ColorChoice, Arg, ArgGroup, App};
use clap::{app_from_crate, arg, crate_name};
use walkdir::{DirEntry};
use std::time::Duration;
use std::{env, error::Error};
use std::time::Instant as SInstant;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

pub const MY_ARGUMENT_PROCESS: bool = true;
pub const ARGUMENTS_PRINT: bool = true;

type MyResult<T> = Result<T, Box<dyn Error>>;
pub fn advection_input()  -> MyResult<(Argumento, MyConfiguration)>{
    let start = SInstant::now();
    let clap_arguments = App::new(clap::crate_name!()).color(ColorChoice::Always)
    .version("0.1")
    .author("Maxim <mmmaximus1403@gmail.com>")
    .about("Does awesome things")
    .arg(Arg::new("SWITCH_TIME")
        .short('s')
        .default_value("false")
        .long("switch_time")
        .help("Sets option for taking real-time or dt on every iteration in main.rs"))
    //This will determine from crate log output enable/disable
    .arg(Arg::new("debug")
        .short('d')
        //.min_values(1)
        .help("Sets the level of debugging information"))
    .arg(Arg::new("CORRECTION")
        .short('c')
        .long("correction")
        .required(false)
        .help("Sets the input file to use"))
    .arg(Arg::new("transfer-velocity")
        .takes_value(true)
        .default_value("10_f64")
        .conflicts_with("burger")
        .long("transfer-velocity"))
    .arg(Arg::new("burger")
        .takes_value(true)
        .conflicts_with("debug")
        .default_value("Burger_task")
        .long("burger-task"))
    .arg(Arg::new("amount-of-files")
        .short('q')
        .long("fquantity")
        .takes_value(true)
        //.map(parse_positive_int)
        //.map_err(|e| format!("illegal amount of files number -- {}", e))?
        .default_value("6")
        .help("Sets how many files will be processed[default MAXIMUM_FILES_TO_EXPECT=6]"))
    .arg(Arg::new("cli-files")
        .long("cli-files"))
    .arg(Arg::new("in-file")
        .long("in-file")
        .takes_value(true))
    .arg(Arg::new("from-directory")
        .long("dir-path") 
        .takes_value(true)
        //.required_unless_present("path-to-files")
    )
    .group(ArgGroup::new("output-style")
            .args(&["cli-files",//arg!(--cli-files [COMMANDLINE] "whether or not to get from cli file paths")
            "in-file",
            "from-directory"])
            .required(true))//Only one of them!
    .arg(Arg::new("path-to-files")
        .short('f')
        .long("file-paths")
        .multiple_occurrences(true)
        .conflicts_with("in-file")
        .help("Gives your own path to main programm")
        .takes_value(true)
        .requires("cli-files")
    ).get_matches();
    let mut task_type: TaskType = TaskType::Burger(BurgerOrder::Arbitrary, clap_arguments.value_of("burger").unwrap().to_string());;
        //.try_get_matches_from(vec!["advection", "--cli-files"]);
    if clap_arguments.is_present("transfer-velocity") {
        let vel = clap_arguments.value_of("transfer-velocity").unwrap().parse::<f64>().unwrap_or(0_f64);
        task_type = TaskType::Transfer{a: vel};
    }
    if ARGUMENTS_PRINT{
            println!("{:#?}", &clap_arguments);}
    assert!(clap_arguments.is_present("output-style"));
    //Check what style I/someone had chosen
    let mut outcli = false;
    let mut from_files = false;
    let mut from_directory = false;
    let (stdoutput, to_file, out_get_dir) = (
        clap_arguments.is_present("cli-files"),
        clap_arguments.is_present("in-file"),
        clap_arguments.is_present("from-directory"),
    );
    match (stdoutput, to_file, out_get_dir) {
        (true, _, _) => outcli = true,
        (_, true, _) => from_files = true,
        (_, _, true) => from_directory = true,
        _ => {},//unreachable!(),
    };
    let out_style_from_cli = clap_arguments.is_present("cli-files");
    let switch_time = clap_arguments.is_present("SWITCH_TIME");
    let debug = clap_arguments.is_present("debug");
    let correction = clap_arguments.is_present("CORRECTION");
    // we can safely unwrap as the argument has default value
    let amf = clap_arguments.value_of("amount-of-files").unwrap();
    if ARGUMENTS_PRINT{
        format!("stdout?{}-fileout?{}-from_directory?{}", outcli, from_files, from_directory);
        println!("Value for SWITCH_TIME: {}", switch_time);
    }
    let mut files_str: Vec<String> = Vec::new();
    let mut files_buf: Vec<PathBuf> = Vec::new();
    if outcli {
        // we can safely unwrap as the argument is required in case of cli-files
    files_str = clap_arguments.values_of("path-to-files").clone().unwrap().map(|strs| String::from(strs)).collect::<Vec<String>>();
    files_buf = files_str.clone().into_iter().map(|strin| Path::new(&strin[..]).to_path_buf()).collect();
    println!("{}", "Files collected from terminal: ".italic().yellow());
    for fi in &files_str{
        println!("{}", fi);
        }
    }
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    let amf: usize = parse_positive_int(amf)? as usize;
    if ARGUMENTS_PRINT{cyan!("\nCASE_INSENSITIVE: {}\n", case_sensitive);}
//So the last and most: What I need to get?
//query- switch case[default false], if there are ! files in terminal- return filled Argumento, else empty;
//MyConfig will get all other stuff
    //let clap_args: Vec<String> = vec![switch_time.to_string(), ];
    let argumento = if outcli {//so argumento will get paths from cli
        Argumento{query: "From command line".to_string(),
            filenames: files_str, case_sensitive}
        }
        else {
            Argumento{query: String::new(), filenames: (&[]).to_vec(), case_sensitive: false}
        };
    let my_config = if from_files || from_directory {
        let new_patbuf_vec = Vec::<PathBuf>::new();
        let directory_to_files = clap_arguments.value_of("dir-to-files").unwrap();
        MyConfiguration {//this variable suitable for both[from language point]
            search_path: Some(PathBuf::from(directory_to_files)),
            searched_files: new_patbuf_vec,
            debug: debug,
            amf: amf,
            correction: correction,
            out_style: out_style_from_cli,
            task_type,
    }} else{MyConfiguration {//this variable suitable for both[from language point]
        search_path: None,
        searched_files: files_buf,
        debug: debug,
        amf: amf,
        correction: correction,
        out_style: out_style_from_cli,
        task_type}};
    let elapsed = start.elapsed();
    println!("Millis: {} ms", elapsed.as_millis());
    return Ok((argumento, my_config))
    
}
#[derive(Default, Debug, PartialEq)]
pub struct MyConfiguration {
    // Option defaults to None, directory in which search files.
    search_path: Option<PathBuf>,
    // Vecs default to empty vector, files from directory or clone from cli
    searched_files: Vec<PathBuf>,
    debug: bool, 
    amf: usize, 
    correction: bool, 
    out_style: bool, 
    task_type: TaskType,
}

impl MyConfiguration {
    pub fn get_directory_with_files(&self) -> PathBuf{
        if let Some(ps) = &self.search_path{
            ps.to_path_buf()
        }
        else{
            PathBuf::new()
        }
    }
    pub fn get_files(&self) -> Vec<PathBuf>{
        let empty = self.searched_files.is_empty();
        if !empty{
            self.searched_files.clone()
        }
        else{
            Vec::new()
        }
    }
    pub fn get_files_len(&self)  -> usize {
        let empty = self.searched_files.is_empty();
        if !empty{
            self.searched_files.clone().len()
        }
        else{
            0_usize
        }
    }
    pub fn get_advection_modes(&self)-> (bool, bool, bool, usize, TaskType) {
        (self.debug, self.correction, self.out_style , self.amf, self.task_type.clone())
    }
}

mod StrctOptImpl{
    use super::{StructOpt, PathBuf};
    //___________________________________________________________________________________________________
#[derive(Debug, StructOpt)]
#[structopt (name = "debug_parametres", about = "additional info", author= "M")]// name(arg1, arg2, ...) form.
pub struct DebOpt{
    /// Activate debug mode --debug
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short= "d", long= "debug", help = "Pass `-h`: debug is needed to see intermidiate steps of computation")]
    debug: bool,
    #[structopt(short= "s", long= "switchtime", help = "Pass `-h`: True- Measure on world time, false- on period t")]
    time_switch: bool,
    ///choose to apply/not correction Mc
    #[structopt(short = "cc", long = "correct", help = "Pass `-h`: correction is needed to optimize computation")]
    correction: bool,
    /// Output file, stdout if not present
    #[structopt(parse(from_os_str))]
    pub output: Option<PathBuf>,
    /// Where to write the output: to `stdout` or `file`
    #[structopt(short="out", default_value = "stdout", case_insensitive = true)]
    out_type: String,
    /// File name: only required when `out-type` is set to `file`
    #[structopt(name = "FILE", required_if("out-type", "file"))]
    pub file_name: Vec<String>,
    #[structopt(name = "AmountOfFiles", short = "af", long ="amount_of_files", default_value = "3",
        help = "Pass `-h`: These will process exact amount of initial data files")]
    pub amount_of_files: i32,
    }
}
pub fn process_files<'a>(new_path_obj: &'a mut Vec<PathBuf>, num_files: Option<usize>, db: Option<bool>) 
//-> StdtResult<FileParametres>
{
    let files_vec: Arc<Mutex<Vec<FileParametres>>> = if let Some(num_files) = num_files {
        Arc::new(Mutex::new(Vec::with_capacity(num_files * 2_usize)))
    }
    else{
        Arc::new(Mutex::new(Vec::new()))
    };
    let mut paths_buf: Vec<PathBuf>= Vec::<PathBuf>::new();
    let paths_hs: HashSet<PathBuf> = paths_buf.clone().into_iter().collect();
    let arc_new_paths=  Arc::new(Mutex::new(paths_hs));
    let mut paths_in_option: Vec<Option<PathBuf>> = new_path_obj.clone().into_iter().map(|p| Some(p)).collect::<Vec<_>>();
    for (fi, fp) in  paths_in_option.iter().enumerate(){
    if let Some(path_to_example_file) = fp{
        let mut file_i = 0_usize;
        yellow!("{}th - {:?}", fi+1, path_to_example_file);
        file_i+=1_usize;
        }
    }
}

    // add setters here


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
