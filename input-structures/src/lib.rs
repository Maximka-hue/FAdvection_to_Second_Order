//This lib will implement initial interaction in programm(command-line, basic functions, etc.)
#![feature(total_cmp)]
#[warn(unused_imports)]
#[macro_use] 
extern crate tcprint;
extern crate colorify;
#[macro_use]
extern crate colour;
extern crate colored;
extern crate clap;
use colored::Colorize;
pub mod initial_data_utils;
pub use crate::initial_data_utils::{PathBuf,Path, function_utils::{ cfutils::{self, Argumento, 
    run, parse_pair, parse_three, op_sys, approx_equal, parse_positive_int, create_output_dir}}};
pub use crate::initial_data_utils::initial_input_structures::{TaskType, TaskTypeCs,BurgerOrder, FileParametres, FileParametresBuilder, initial_information_of_advection};
use crate::initial_data_utils::function_utils::print_macros::macro_lrls;
use rustils::parse::boolean::str_to_bool;
extern crate rayon;
use rayon::prelude::*;
//use std::time::{Instant};
//use chrono::{Local};
use tutil::crayon::Style;
use tutil::crayon::Color::*;
extern crate rand;
use rand::{prelude::*};
pub use structopt::StructOpt;
use clap::{ ColorChoice, Arg, ArgGroup, App};
use clap::{app_from_crate, arg, crate_name};
use walkdir::{DirEntry};
use std::time::{Duration, Instant as SInstant};
use std::{thread, io::Write, fs::{self, File, OpenOptions, read_to_string}, env, error::Error};
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use log::info;
use libc::{c_double, c_long};


#[path="./smooth.rs"]
pub mod smooth;
use smooth::smooth_zf_rs;

pub const MY_ARGUMENT_PROCESS: bool = true;
pub const ARGUMENTS_PRINT: bool = true;
pub const PROCESS_DETAIL: bool = true;
pub const MONOTIZATION_MIN: usize = 50;
pub const MONOTIZATION_MAX: usize = 350;


extern "C" {
    fn smooth_arr_zm_fur(Fs: *mut c_double, Nmax: c_long /*i64*/, smooth_intensity: c_double, Fi: *mut c_double, Ftd: *mut c_double) ->  c_long;
    fn callback();
}
#[cfg(target_os = "linux")]
fn call_callback() -> Box<()>{
    unsafe{
        Box::new(callback())
    }
}
#[cfg(target_os = "linux")]
fn call_smooth(inner_vector: &mut Vec<f64>, all_steps: usize, smooth_intensity: f64, first_correction: &mut Vec<f64>, second_correction: &mut Vec<f64>) -> i64 //this is std c error if !=0.
    //-> Box<Fn(Vec<f32>, i32, f32, Vec<f32>, Vec<f32>) -> i32>  //I am transfer as arguments **mut reference**)
    {
        unsafe{//This is c function
            smooth_arr_zm_fur(inner_vector.as_mut_ptr() as *mut f64, all_steps as i64, smooth_intensity as f64,
                first_correction.as_mut_ptr() as *mut f64, second_correction.as_mut_ptr() as *mut f64)
        }
    //Box::new(smooth_arr_zm_fur(inner_vector.as_mut_ptr() as *mut f64, all_steps as i32, smooth_intensity,
    //    first_correction.as_mut_ptr() as *mut f64, second_correction.as_mut_ptr() as *mut f64))
}
#[cfg(not(target_os = "linux"))]
fn call_smooth(inner_vector: &mut Vec<f64>, all_steps: usize, smooth_intensity: f64, first_correction: &mut Vec<f64>, second_correction: &mut Vec<f64>) 
    //-> Box<Fn(Vec<f32>, i32, f32, Vec<f32>, Vec<f32>) -> i32>
    {//This is rust function
        smooth_zf_rs(inner_vector, all_steps, smooth_intensity, first_correction, second_correction)
}

type MyResult<T> = Result<T, Box<dyn Error>>;
pub fn advection_input()  -> MyResult<(Argumento, MyConfiguration)>{
    let start = SInstant::now();
    let clap_arguments = App::new(clap::crate_name!()).color(ColorChoice::Always)
    .version("0.1")
    .author("Maxim <mmmaximus1403@gmail.com>")
    .about("Does awesome things")
    .arg(Arg::new("SWITCH_TIME")
        .short('s')
        //.default_value("false")
        .long("switch_time")
        .help("Sets option for taking real-time or dt on every iteration in main.rs"))
    //This will determine from crate log output enable/disable
    .arg(Arg::new("debug")
        .short('d')
        //.min_values(1)
        .help("Sets the level of debugging information"))
    .arg(Arg::new("CORRECTION")
        .long("correction")
        .required(false)
        .help("Sets the input file to use"))
    .arg(Arg::new("transfer-velocity")
        .takes_value(true)
        .default_value("0_f64")
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
        .max_values(1)
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
        .min_values(1)
        .conflicts_with("in-file")
        .help("Gives your own path to main programm")
        .takes_value(true)
        .requires("cli-files")
    ).get_matches();
    let mut task_type: TaskType = TaskType::Burger(BurgerOrder::Arbitrary, clap_arguments.value_of("burger").unwrap().to_string());
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
    let mut directory_to_files = PathBuf::new();
    let examples_from_file: PathBuf = PathBuf::new();
    if outcli {
// we can safely unwrap as the argument is required in case of cli-files
    files_str = clap_arguments.values_of("path-to-files").clone().unwrap().map(|strs| String::from(strs)).collect::<Vec<String>>();
    files_buf = files_str.clone().into_iter().map(|strin| Path::new(&strin[..]).to_path_buf()).collect();
    println!("{}", "Files collected from terminal: ".italic().yellow());
    for fi in &files_str{
        println!("{}", fi);
        }
    }
    else if from_directory{
        directory_to_files = PathBuf::from(clap_arguments.values_of("from-directory").unwrap().next().unwrap_or(""));
    }
    else if from_files{
        let examples_from_file = PathBuf::from(clap_arguments.value_of("in-file").unwrap());
        files_str = extract_example_names(examples_from_file);
    }
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    let amf: usize = parse_positive_int(amf)? as usize;
    if ARGUMENTS_PRINT{
        cyan!("\nCASE_INSENSITIVE: {}\n", case_sensitive);}
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
        info!("You specified desire to input your own directory/file path for further search");
        let new_patbuf_vec = Vec::<PathBuf>::new();
        MyConfiguration {//this variable suitable for both[from language point]
            search_path: Some(directory_to_files),
            searched_files: new_patbuf_vec,
            debug: debug,
            amf: amf,
            correction: correction,
            out_style: out_style_from_cli,
            switch_time,
            task_type,
    }} else{
        MyConfiguration {//this variable suitable for both[from language point]
        search_path: Some(examples_from_file),
        searched_files: files_buf,
        debug: debug,
        amf: amf,
        correction: correction,
        out_style: out_style_from_cli,
        switch_time,
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
    switch_time: bool, 
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
    pub fn get_advection_modes(&self)-> (bool, bool, bool, usize, bool, TaskType) {
        (self.debug, self.correction, self.out_style , self.amf, self.switch_time, self.task_type.clone())
    }
}
fn extract_example_names(f_path: PathBuf) -> Vec<String> {
    //TODO extract example names or paths to them
    Vec::<String>::new()
}
mod strct_opt_impl{
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
type StdtResult<T> = std::result::Result<(Vec<T>, Vec<String>), Box<dyn Error>>;
pub fn process_files<'a>(new_path_obj: &'a mut Vec<PathBuf>, num_files: Option<usize>, db: Option<bool>, should_sleep: Option<bool>, init_dir: Option<String>) 
-> StdtResult<FileParametres>
    {
    let additional_print = if let Some(d) = db{
        d
    }
    else{
        true
    };
    let files_vec: Arc<Mutex<Vec<FileParametres>>> = if let Some(num_files) = num_files {
        Arc::new(Mutex::new(Vec::with_capacity(num_files * 2_usize)))
    }
    else{
        Arc::new(Mutex::new(Vec::new()))
    };
    let mut created_paths: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let paths_hs: HashSet<String> = new_path_obj.clone().into_iter().map(|h| String::from(h.to_string_lossy())).collect();
    let number_of_dif_files = paths_hs.len();
    let mut paths_vec: Vec<String> = paths_hs.into_iter().collect();
    let mut str_paths: Vec<&str> = paths_vec.iter().map(|s| s.as_ref()).collect();
    let arc_new_paths=  Arc::new(Mutex::new(paths_vec.clone()));
    let mut paths_in_option: Vec<Option<PathBuf>> = paths_vec.clone().into_iter().map(|p| Some(PathBuf::from(p))).collect::<Vec<_>>();
    let mut created_data_directories: Vec<File> = Vec::new();
//First of all create directories for data .csv/txt storage
    /*paths_in_option.iter_mut().enumerate().for_each(|(fi, fp)| {
        if let Some(path_to_example_file) = fp{
            yellow!("{}th - {:?}", fi+1, path_to_example_file);
            let (fnum, new_buf, new_path_string, processed_params)= create_output_dir(fi, num_files.unwrap_or(number_of_dif_files), 
                should_sleep.unwrap_or(true), init_dir.clone()).expect("In creating output files error ");
            created_data_directories.push(processed_params);  
        }
    });*/
    //let init_dir = init_dir.unwrap().map(|h| String::from(h.to_string_lossy()));
//Next from string paths to input file data preprocess and write afterwards to previously created directories
    paths_vec.into_par_iter().zip((0..number_of_dif_files).into_iter()).for_each(|(p, fi)| {
        let init_dir: &String = init_dir.as_ref().unwrap();
        let mut file_i = fi;
        let new_init_data = preprocess_text_for_parallel(&p.to_string(), PROCESS_DETAIL, &mut file_i);
        if additional_print { 
            println!("{:#?} - {}", new_init_data, file_i);}
        let files_vecs=  Arc::clone(&files_vec);
        let create_paths=  Arc::clone(&created_paths);
//For every preprocessed text ....
        new_init_data.into_par_iter().for_each(|new_init_data| {
        if additional_print {
            println!("New updated vector\n{:#?}", &new_init_data);}
        let (x_min, x_max) = parse_pair::<f64>(new_init_data[1].as_str(), ':').expect("Second argument margin_domain must be tuple of pair");
        let (i1,i2,i3) = parse_three::<f64>(new_init_data[5].as_str(), ':').expect("Forth argument is init_conditions, must be three digits here");
        let (t1, t2) = parse_pair::<f64>(new_init_data[2].as_str(), ':').expect("3d argument is time, also three digits");
        if additional_print {
            println!("Domain{:?}, Time{:?}, Initial conditions{:?}", (x_min,x_max), (t1,t2), (i1,i2,i3));}
        yellow!("{}th - {:?}", file_i, &p);
        let (fnum, new_buf, new_path_string, mut processed_params)= create_output_dir(file_i, num_files.unwrap_or(number_of_dif_files), 
                should_sleep.unwrap_or(true), Some(&init_dir)).expect("In creating output files error ");
        created_paths.lock().unwrap().push(new_path_string);
                //created_data_directories.push(processed_params); 
        let err= processed_params.write_all((format!("equation_type:{data1}  {sep} 
Optional argument(velocity): {dataadd}  {sep} 
Margin domain: {data3:?} {sep} 
Time evaluation period: {data4:?} {sep} 
Boundary type: {data5}  {sep}  
Initial type: {data6}  {sep}  
Initial conditions: {data7:?} {sep} 
Quantity split nodes: {data8:?} {sep} 
Courant number: {data9}  \n\nThis file was {fnum} with path \n{new_buf:?}",data1 = new_init_data[0], data3 = (x_min,x_max), data4 =  (t1,t2),//parse_pair(&init[2..4],","),
            data5 = new_init_data[3], data6 = new_init_data[4], data7 =(i1,i2,Some(i3)),// parse_three(String::as_str(String::from(init[6..8])),","),  
            data8 = new_init_data[6], data9 = new_init_data[7], dataadd =  new_init_data[8], sep = ',')).as_bytes());
            if additional_print{
                println!("{:?} ", err );}
            let eq = new_init_data[0].parse::<i8>().unwrap();
            let bound_type = new_init_data[3].parse::<i8>().unwrap();
            let init_type =  new_init_data[4].parse::<i8>().unwrap();
            let quantity_split_nodes = new_init_data[6].parse::<f64>().unwrap();
            let n_corant = new_init_data[7].parse::<f64>().unwrap();
            let vel = new_init_data[8].trim().parse().unwrap_or(0_f64);
            //From my impl new
            let all_datas =  FileParametres::new(eq, (x_min,x_max),
                (t1, t2, false/*this can determine switch time option*/), bound_type, init_type, (i1, i2, i3, 0_f64),
                quantity_split_nodes, n_corant,
            //Here I pass additional arguments!If not 0=> will be BURGER type, if !=0, then type TRANSFER
                (TaskType::Transfer{a: vel}, 0_i8, false)).unwrap();
            //from default
            let possible_error = FileParametresBuilder::default()
                .eq_type(eq).margin_domain((x_min,x_max)).time_eval_period_stage((t1, t2, Some(false)/*this can determine switch time option*/))
                .bound_type(bound_type).init_type(init_type).init_conditions((i1, i2, Some(i3), Some(0_f64)))
                .quantity_split_nodes(quantity_split_nodes).n_corant(n_corant).add_args((Some(TaskType::Transfer{a: vel}), Some(0_i8), Some(false)))
                .build();
        println!("\n{:?\n}", possible_error.ok());
        if additional_print{
            println!("{}{:#?}\n",ansi_term::Colour::Yellow.on(ansi_term::Colour::Green).paint("From file: "), all_datas);}
        let all_datas =  FileParametres::new(new_init_data[0].parse::<i8>().unwrap(), (x_min,x_max),
            (t1, t2, false), new_init_data[3].parse::<i8>().unwrap(), new_init_data[4].parse::<i8>().unwrap(), (i1, i2, i3, 0_f64),
            new_init_data[6].parse::<f64>().unwrap(), new_init_data[7].parse::<f64>().unwrap(),
            //Here I pass additional arguments!If not 0=> will be BURGER type, if !=0, then type TRANSFER
            (TaskType::Transfer{a: new_init_data[8].trim().parse().unwrap_or(0_f64)}, 0_i8, false)).unwrap();
            if additional_print{
                println!("{}{:#?}\n",ansi_term::Colour::Cyan.on(ansi_term::Colour::Green).paint("From file: "), all_datas);}
            //then push all in earlier created vector for storing processed files
            files_vecs.lock().unwrap().push(all_datas.clone());
            
            });
//Processed data 
    });
let result = files_vec.lock().unwrap().to_vec().clone();
let message_from_thread="The child thread ID: ".to_string();
let len_dots= message_from_thread.len();
//println!("{m:?} {0:?}", &files_vec, m= message_from_thread);
let repeated: String= std::iter::repeat(".").take(len_dots).collect();
println!("{:?}", repeated);
let successfuly_created_paths = created_paths.lock().unwrap().to_vec().clone();

println!("Processed: {:#?}", result);
Ok((result, successfuly_created_paths))
}
    //}
//}
pub fn preprocess_text_for_parallel<'a>(file: &String, deb: bool, file_number: &'a mut usize)-> Result<Vec<std::string::String>, ()>{
    use std::char;
    println!("{:?}", file);
        let file_content = read_to_string(&file)
            .expect("While reading occured an error");
        let crude_data: String = file_content.split("\n ").map(|x| str::to_string(x.trim())).collect();
        println!("{:#?}- unprocessed file with lenght: {} in file {} processing\n", crude_data, crude_data.len(), file_number);//let mut sep_sgn = String::new();
        let io_sgn = ',';//read_string("You can choose the separation sign in the processed file:"); //Какой выбрать знак разделения в обработанном файле
        let rinsed_data: Vec<&str> = crude_data.split("\n").collect();
        if deb{
            red!("\nRinsed: {:#?} in {file_number} file", &rinsed_data);}
        let mut new_init_data = Vec::with_capacity(25);
        let mut rubbish = Vec::with_capacity(25);
        for x in rinsed_data{
            let mut y =  x.trim_matches(char::is_alphabetic)
                .replace(","," ").replace("'","").replace(" ","");//.replace(" ",":");
            let lovely_sgn = '💝';
            let _lh: usize = '💝'.len_utf8();
            let mut b = [0; 4];
            lovely_sgn.encode_utf8(&mut b);
            if y.contains(char::is_numeric) { 
                if y.contains('💝') { 
                    let r = y.find('💝');
                    if let Some(rr)  = r {
                        let (z, zz) = y.split_at_mut(rr);//.chars().next().unwrap()
                        let new_z = z.trim_matches(char::is_alphabetic).replace("'", "").replace("\r", "").replace("\\", "").replace("\"","").to_string();
                        let mut new_zz: String = (&zz[..]).to_string();
                        new_zz = new_zz.trim_matches(char::is_alphabetic).replace("'", "").replace("\r", "").replace("\\", "").to_string();
                        rubbish.push(new_zz.to_string());
                        new_init_data.push(new_z.to_string());
                }//>>>>>>>>>>>>>>>>>>>>>
            }
            else {
                y = y.trim_matches(char::is_alphabetic).replace("'", "").replace("\r", "").replace("\\", "").replace(","," ").trim_matches(char::is_alphabetic).to_string();
                new_init_data.push(y);
            }
        }
        else if !y.contains(char::is_numeric) {
            panic!("Expected that in files would be digits.");
        }
        else{
            y = y.trim_matches(char::is_alphabetic).replace("'", "").replace("\r", "").replace("\\", "").replace(","," ");
            new_init_data.push(y);
            }
        }
        //*file_number+=1_usize;
        if deb{
            println!("\nRb_comments: {:#?}  in {file_number} file", rubbish);}
        Ok(new_init_data)
    }

pub fn main_initialization(steps: usize, debug_init: bool, calculation_path: &str, data_serial_number: usize, 
    equation: i8, type_of_initial_cond: i8, dx: f64, centre: f64, width: f64, height: f64, veloc: f64, left: f64, right: f64, check_flag_for_partition: bool)
    -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, File, File, File, f64){
    use std::time::Instant;
    let init_t  = Instant::now();
    let deb_init = true;
    println!("{}", Style::new().foreground(Blue).italic().paint("Constructing array \nfor saving values of function"));
    let mut vprevious = vec![0_f64; steps];
    if debug_init {
        println!("Size {} steps {}\n", vprevious.len(), steps as f32);
        assert!(vprevious.len() == steps);
        let values_all_same = vprevious.iter()/*.inspect(|val| println!("Inspect on size now-{}",val))*/.all(|& x| x == vprevious[0]);
        println!("All array's dtypes values the same?{}", values_all_same);
    }
    let mut inner_vector = vec![0_f64; steps]; // As next time step to vprevious
    if debug_init {
        println!("{}: {} # {} ", Style::new().foreground(Blue).italic().paint("Size of inner and previous arrays"), inner_vector.len(), vprevious.len());
        info!("{}== {}?", inner_vector.len(), vprevious.len());
        //They will be exchanging values in main loop.
        std::thread::sleep(std::time::Duration::from_millis(300_u64));
    }
    let mut exact_solvec = vec![vec![0_f64; steps], vec![0_f64; steps], vec![0_f64; steps]];//vec![vec![0_f32;steps + 2], vec![0_f32; steps + 2], vec![0_f32;steps + 2]];
    if debug_init{
        let all_same_length = exact_solvec.iter().all(|ref v| v.len() == exact_solvec[0].len());
        if all_same_length {
            println!("They're all the same");
        } else {
            println!("They are not the same");
        }
    }
    let elapsed_in = init_t.elapsed();
    if debug_init{
    println!("Creating arrays took: {:.2?}", elapsed_in);
    }
    info!("Start in determining initial shape");
    let mut first_ex = exact_solvec[0].clone();
    let mut second_ex = exact_solvec[1].clone();
    let mut temporary = exact_solvec[2].clone();
    //Needed in 1 and 2 shapes
    let mut all_steps= vprevious.len()+2;//eliminate in 0/1 shapes additional on bound type knots
//----------Create a lot of txt with differential mistakes and x u(numerical sol.) and exact solution
let calculation_path = calculation_path.to_string();
println!("{}", calculation_path);
let example_data_path = Path::new(&calculation_path[..]).join("example_datas");
/*Here will be differential errors in main cycle for fi_th example*/let new_path_dif = example_data_path
    .join(&format!("differ_errors_{}", data_serial_number)[..]);
/*Here will be x_u_w storage in txt in main cycle for fi_th example*/let new_xuv_txt = example_data_path
    .join(&format!("x_u_w_txt_{}", data_serial_number)[..]);
/*Here will be x_u_w storage in csv in main cycle for fi_th example*/let new_xuv_csv = example_data_path
    .join(&format!("x_u_w_csv_{}", data_serial_number)[..]);
    fs::create_dir_all(&new_path_dif).unwrap(); 
    fs::create_dir_all(&new_xuv_txt).unwrap();
    fs::create_dir_all(&new_xuv_csv).unwrap();
    let first_dif = new_path_dif.join("diferr_0.txt");
    let first_xuv_txt = new_xuv_txt.join("x_u_w_0.txt");
    let first_xuv_csv = new_xuv_csv.join("x_u_w_0.csv");
//This will be used for determining initial shape: line, gauss wave, etc.
    let mut diferr_0 = OpenOptions::new()
        .write(true).create(true).open(first_dif).expect("cannot open file x_v_w");
    diferr_0.write_all("t, norm1, norm2\n0,0,0\n".as_bytes()).expect("write failed"); 
    let mut x_v_w_txt_0 = OpenOptions::new()
            .write(true).create(true).open(first_xuv_txt).expect("cannot open file x_v_w");
    x_v_w_txt_0.write_all("x, u, w\n".as_bytes()).expect("write failed"); 
    let mut x_v_w_csv_0 = OpenOptions::new()
            .write(true).create(true).open(first_xuv_csv).expect("cannot open file x_v_w");
    x_v_w_csv_0.write_all("x, u, w\n".as_bytes()).expect("write failed"); 
//Now let's create first forms from initial data
//For this I do need: 1 type of equation 2 initial conditions 3 dx(fragmentation) 4velocity(if eq=1) 4 check_flag_for_partition
if check_flag_for_partition {
    assert!(approx_equal(left + steps as f64 * dx, right, 6));
    if equation == 0 || equation == 1 {
    assert!((centre - (width /2.0)) - left >= 0.0); assert!(right - (centre + (width /2.0)) >= 0.0);
    println!("{}", ansi_term::Style::new().underline().paint("Левая|правая точка треугольник в заданной области"));
    }
}
let start_left = (left * 1000_0000.0).ceil()/*because it is needed to be inside domain*//1000_0000.0;
let end_right = (right * 1000_0000.0).floor()/*because it is needed to be inside domain*//1000_0000.0;
let smax: f64 = match equation{
    0 => {
        //measurement in pace(n*dx), important on edge!
        let dip_start = ((start_left / dx) + ((centre - (width / 2.0) as f64) / dx)) as usize;//need to coincide with one of nodes
        let dip_end = ((end_right / dx) - (centre + (width / 2.0) / dx)) as usize;
        let dip = (width / dx) as usize;//For clarity
        let node_end = dip_start + dip;
        let start = ((centre - (width / 2.0) as f64) / dx) as usize;
        let end = ((centre + (width / 2.0) as f64) / dx) as usize;
        let mut x_next: usize;
        match type_of_initial_cond {//First check where will be throughout steps...so it will be inside
            0 => {     
                if deb_init {
                    println!(" {} {} {} steps: {} ---- start: {} end: {}", dip_start,  dip_end , node_end, steps, start, end);}
                //----------------Let's pace
                for n in 0..dip+1 {
                    x_next = start + n as usize;
                    vprevious[x_next] = height.max(-height) as f64;
                    first_ex[x_next] = height.max(-height) as f64;
                    if dip<30 {
                        if n % 1== 0 && deb_init {
                            println!("Получившиеся значения с шагом {} равны {}\n", n  + start , vprevious[x_next]);}
                    }
                    else if (n+1)%10 == 0 {
                        println!("Получившиеся значения с шагом {} равны {}\n", n + start, vprevious[x_next]);
                    }
                    info!("Runge: Step: {} - Value: {} ", n, vprevious[n + start]);
                }
                println!("Остальные == 0");   
            },
            1 => {
                println!("{}", ansi_term::Colour::Yellow.underline().paint("Равнобедренный треугольник под уравнение переноса"));
                for n in 0..dip/2+1 {//this is not odd dip
                    x_next = start + n as usize;
                    vprevious[x_next] = (height as f64 *2.0) as f64 * (dx * n as f64) /width as f64;
                    first_ex[x_next] = vprevious[x_next].clone();
                    temporary[x_next] = 2_f64 * width/height;
                    info!("Triangle: Step: {} - Value: {} ", start+ n, vprevious[n + start]);
                    if n > 0 && deb_init {println!("n: {} previous layer: {}", n, vprevious[n + start]);}
                    if dip/2 < 30 {
                        if n% dip/2 == 0 && deb_init {
                            println!("Получившиеся значения с шагом {} равны {}\n", n + start, vprevious[n as usize + start as usize]);
                        }
                    }
                    else if n+1%10 == 0{
                        println!("Получившиеся значения с шагом {} равны {}\n", n + start , vprevious[n as usize + start as usize]);
                    }
                    println!("Остальные слева == 0");
            }
                for n in dip/2+1..dip+1 {
                    x_next = start + n;
                    vprevious[x_next] = height - (height *2.0) * (dx*(n-dip/2) as f64) / width;
                    first_ex[x_next] = vprevious[x_next].clone();
                    temporary[x_next] = -2_f64 * width/height;
                    info!("Triangle: Step: {} - Value: {} ", start+ n, vprevious[start+n]);
                    if dip/2 < 11{
                        if n+1% dip/10 == 0 && deb_init {
                            println!("Получившиеся значения с шагом {} равны {}\n", n + start, vprevious[n + start]);}
                            println!("Остальные ==0");}
                        else if n+1%10 == 0{
                    println!("Получившиеся значения с шагом {} равны {}\n", n + start, vprevious[n +start]);}
                }
                println!("Остальные справа == 0");
            },
            2 =>  //Manage with some differences*
            {pt!(format!("{}", ansi_term::Style::new().underline().paint("Гауссова волна под уравнение переноса")));
            let cnt: f64 = 1.0/(width * (std::f64::consts::PI* 2_f64).sqrt());
            let cnt_tmp: f64 = 1.0/(width.powi(3) * (std::f64::consts::PI * 2_f64).sqrt());
            for n in  0..steps {
                let x_next: f64 = start_left as f64 + n as f64 * dx;//this needed to be on "domain" scale
                vprevious[n] = cnt * (-  ((x_next as f64 - centre).powi(2)  ) / (2.0 * width.powi(2))).exp();//exp^self  
                println!("This is copy from slice*: {}", first_ex[n]);
                temporary[n] = - cnt_tmp * (-((x_next as f64 - centre).powi(2))/
                    (2.0 * width.powi(2))).exp();
                info!("Gauss: Step: {} - Value: {} ", n, vprevious[start + n ]);
            }
            first_ex = vprevious.clone();
            let maxvalue = vprevious.iter().cloned().fold(0./0., f64::max);
            info!("Max value in array with gauss wave: {}", maxvalue);
                println!("MAXIMUM VALUE: {}", maxvalue);
            },
            3 => {
                pt!(format!("{}", ansi_term::Style::new().underline().paint("Синусоида под уравнение переноса")));
            //if start.clamp(f32::MIN, f32::MAX)==start && end.clamp(f32::MIN, f32::MAX)== end{
                let distance= end_right - start_left;
                let mut angle: f64;
                const DOUBLE_PI: f64 = 2.0 * std::f64::consts::PI;
                for n in  0..steps {
                    let x_next = start_left + n as f64 * dx;
                    angle = x_next as f64 * DOUBLE_PI / distance;
                    vprevious[n] = angle.sin();
                    info!("Sinusoid: Step: {} - Value: {} ", n , vprevious[n as usize]);
                }
                first_ex[..].copy_from_slice(&vprevious[..]);
            //else{panic!("Too extensive domain!");}
            },
            4 => {
                pt!(format!("{}", ansi_term::Style::new().underline().paint("Прямая под уравнение переноса")));
                let alpha = width.clone();//For clarity
                let c = height.clone();
                all_steps = steps;
                vprevious.resize(all_steps, 0.0);
                first_ex.resize(all_steps, 0.0);
                second_ex.resize(all_steps, 0.0);
                inner_vector.resize(all_steps, 0.0);
                for n in  0..all_steps {
                    let x_next = start_left + n as f64 * dx;
                    vprevious[n] = x_next.mul_add(alpha, c);
                    info!("Line: Step: {} - Value: {} ", n, vprevious[n as usize]);
                }
                first_ex.copy_from_slice(&vprevious[..]);
                //}
            },
            other => {println!("Options of initial conditions can be only 0...4! found {}", other)
            },
             // Anyway we return a velocity in TRANSFER: it's constant
        };//First check
        veloc}
    1=> {
        let start = ((centre - (width / 2.0) as f64) / dx) as usize;
        let end = ((centre + (width / 2.0) as f64) / dx) as usize;
        let dip = (width / dx) as usize;//For clarity
        let mut x_next: usize;
        let fsmax: f64 = match type_of_initial_cond {
            0 => {
                println!("{}", ansi_term::Colour::Yellow.underline().paint("Ступенька под уравнение <Бюргеррса>"));
                for n in 0..dip+1 {
                    x_next = start + n;
                    vprevious[x_next] = height.max(-height);
                    first_ex[x_next] = height.max(-height);
                if dip<30{
                    if n%2== 0 && deb_init {println!("Получившиеся значения с шагом {} равны {}\n", n  + start , vprevious[x_next]);
                }
                    println!("Остальные == 0");
                }
            else if (n+1)%10 == 0 {
                println!("Получившиеся значения с шагом {} равны {}\n", n + start, vprevious[x_next]);
                    }
            info!("Runge: Step: {} - Value: {} ", n, vprevious[n + start]);
                }
                height},
            1 => {
                println!("{}", ansi_term::Colour::Yellow.underline().paint("Равнобедренный треугольник под уравнение переноса"));
                for n in 0..dip/2 + 1_usize{//this is not odd dip
                    x_next = start + n;
                    vprevious[x_next] = (height  *2_f64) * (dx * n as f64) /width;
                    first_ex[x_next] = vprevious[x_next].clone();
                    temporary[x_next] = 2_f64 * width/height;
                    info!("Triangle: Step: {} - Value: {} ", start+ n, vprevious[(start+n) as usize]);
                    if n > 0 && deb_init {
                        println!("n: {} previous layer: {}", start+ n, vprevious[(start+n) as usize]);}
                if dip/2<30 {
                        if n% dip/2 == 0 && deb_init {
                            println!("Получившиеся значения с шагом {} равны {}\n",n + start , vprevious[n + start]);}
                            println!("Остальные == 0");
                        }
                else if n+1%10 == 0{
                        println!("Получившиеся значения с шагом {} равны {}\n", n  + start , vprevious[n + start]);
                        println!("Остальные == 0");
                    }
            }
            for n in dip/2+1..dip + 1_usize{
                x_next = start + n as usize;
                vprevious[x_next] = height  - (height *2.0)  * (dx*(n -dip/2) as f64) /width as f64;
                first_ex[x_next] = vprevious[x_next].clone();
                temporary[x_next] = -2_f64 * width/height;
                info!("Triangle: Step: {} - Value: {} ", start+ n, vprevious[(start+n) as usize]);
                if dip/2 < 11{
                    if n+1% dip/10 == 0 && deb_init {
                        println!("Получившиеся значения с шагом {} равны {}\n", n as f32 + start as f32, vprevious[n as usize + start as usize]);}
                        println!("Остальные ==0");}
                else if n+1%10 == 0{
                    println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start as f32, vprevious[n as usize+start as usize]);
                }
            }
            thread::sleep(Duration::from_millis(50_u64));
                    let max_value = *vprevious.iter().max_by(|a, b| a.total_cmp(b)).expect("Problem with Burger in type one"); 
                    max_value},
            2 =>  //Manage with some differences*
            {   let cnt: f64 = 1.0/(width * (std::f64::consts::PI* 2_f64).sqrt());
                let cnt_tmp: f64 = 1.0/(width.powi(3) * (std::f64::consts::PI * 2_f64).sqrt());
                for n in  0..steps {
                    let x_next: f64 = start_left as f64 + n as f64 * dx;//this needed to be on "domain" scale
                    vprevious[n] = cnt * (-  ((x_next as f64 - centre).powi(2)  ) / (2.0 * width.powi(2))).exp();//exp^self  
                    println!("This is copy from slice*: {}", first_ex[n]);
                    temporary[n] = - cnt_tmp * (-((x_next as f64 - centre).powi(2))/
                        (2.0 * width.powi(2))).exp();
                    info!("Gauss: Step: {} - Value: {} ", n, vprevious[start + n ]);
                }
                first_ex = vprevious.clone();
                let maxvalue = vprevious.iter().cloned().fold(0./0., f64::max);
                info!("Max value in array with gauss wave: {}", maxvalue);
                    println!("MAXIMUM VALUE: {}", maxvalue);//??Why not this as usual max value 1 on y axis??
                                    maxvalue
            },
        3 => {
            pt!(format!("{}", ansi_term::Style::new().underline().paint("Синусоида под уравнение Burger")));
            let distance= end_right - start_left;
            let mut angle: f64;
            const DOUBLE_PI: f64 = 2.0 * std::f64::consts::PI;
            for n in  0..all_steps {
                let x_next = left + n as f64 * dx;
                angle = x_next as f64 * DOUBLE_PI / distance;
                vprevious[n] = angle.sin();
                info!("Sinusoid: Step: {} - Value: {} ", n , vprevious[n as usize]);
            }
            first_ex[..].copy_from_slice(&vprevious[..]);
            let maxvalue = vprevious.iter().cloned().fold(0./0., f64::max);
            info!("Max value in array with sinusoid: {}", maxvalue);
            println!("MAXIMUM VALUE: {}", maxvalue);//??Why not this as usual max value 1 on y axis??
                                    maxvalue},
            4 => {
                pt!(format!("{}", ansi_term::Style::new().underline().paint("Прямая под уравнение Бюргерсса")));
                let alpha = width.clone();//For clarity
                let c = height.clone();
                all_steps = steps;
                vprevious.resize(all_steps, 0.0);
                first_ex.resize(all_steps, 0.0);
                second_ex.resize(all_steps, 0.0);
                inner_vector.resize(all_steps, 0.0);
                for n in  0..all_steps {
                    let x_next = start as f64 + n as f64 * dx;
                    vprevious[n] = x_next.mul_add(alpha, c);
                    info!("Line: Step: {} - Value: {} ", n, vprevious[n as usize]);
                }
                first_ex.copy_from_slice(&vprevious[..]);
                let maxvalue = vprevious.iter().cloned().fold(0./0., f64::max);
                info!("Max value in array with lines: {}", maxvalue);
                println!("MAXIMUM VALUE: {}", maxvalue);//??Why not this as usual max value 1 on y axis??
                    maxvalue},
    _ => panic!("Initial equation condition incorrect") ,
    }; 
    fsmax}, 
    _ => panic!("Initial equation condition incorrect") ,
};
    let new_now = std::time::Instant::now();
    println!("Main initialization: {:?} < {:?}", elapsed_in, new_now.duration_since(init_t));
(first_ex , second_ex , temporary, vprevious, inner_vector, diferr_0, x_v_w_txt_0, x_v_w_csv_0, smax)
}
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
pub fn main_cycle_first_order(vprevious: &mut Vec<f64>, inner_vector: &mut Vec<f64>, fuu: f64, mut fu_next: f64, mut fu_prev: f64, dt: f64, dx: f64,
        equation: i8, bound: i8, a_positive: bool, possgn_smax: bool,
        all_steps: usize, debug_init: bool){
//This case will pass when f<0
    if (!a_positive && equation==0) || (!possgn_smax && equation==1){//f<0
        //Doing forward scheme
        for k in 0..all_steps-1 {// from first to prelast
            fu_next = match equation {
                0=> fuu * vprevious[k+1],
                1=> vprevious[k+1] * vprevious[k+1] / 2.0,
                _ =>  0.0};
            fu_prev = match equation {
                0=> fuu * vprevious[k],
                1=> vprevious[k] * vprevious[k] / 2.0,
                _ =>  0.0};
            inner_vector[k] = vprevious[k] - (dt/dx)*(fu_next - fu_prev);
        }
    }
//This case will pass when f>0
    else if (a_positive && equation==0)||(possgn_smax && equation==1) {
        for k in 1..all_steps {// from second to last
            fu_next = match equation {
                0=> fuu * vprevious[k],
                1=> vprevious[k] * vprevious[k] / 2.0,
                _ =>  0.0};
            fu_prev = match equation {
                0=> fuu * vprevious[k],
                1=> vprevious[k-1] * vprevious[k-1] / 2.0,
                _ =>  0.0};
            inner_vector[k] = vprevious[k] - (dt/dx)*(fu_next - fu_prev);
        }
    }
    else if (fuu == 0.0 && equation==0) || (equation == 1) {
        std::panic!("{}", &format!("{}", "This mustn'be the case!".on_truecolor(135, 28, 167))[..]);
    }
    //Then set Boundary conditions
    if bound == 0 //   non-reflective condition
    {
        inner_vector[0]= inner_vector[1];
        if debug_init{
            println!("Boundary condition established: {}, on dx {} with dt {}", inner_vector[1] == inner_vector[0], dx, dt);
        }
    }
    else 
    {inner_vector[0] = inner_vector[all_steps-2];
        if debug_init{
            println!("Bound condition established: {}...{}...{}", inner_vector[0] == inner_vector[all_steps-2], dx, dt);
        }
    }  
    if bound == 0 
        {inner_vector[all_steps-1]= inner_vector[all_steps-2];}//      v[n]= v[n-1]
    else
        {inner_vector[all_steps-1] = inner_vector[1];}
}
pub fn main_cycle_with_correction(vprevious: &mut Vec<f64>, inner_vector: &mut Vec<f64>, prediction: &mut Vec<f64>, first_correction: &mut Vec<f64>, second_correction: &mut Vec<f64>,
        fuu: f64, mut fu_next: f64, mut fu_prev: f64, mut fp_next: f64, mut fp_prev: f64, dt: f64, dx: f64, equation: i8, bound: i8, all_steps: usize, debug_init: bool,
            type_of_correction_program: bool, smooth_intensity: f64) {
    for k in 1..all_steps-1 {// from second(cause of overflow in prediction[k-1]) to prelast
//First intermidiate future step
        fu_next = match equation{
            0=> fuu * vprevious[k+1] as f64,
            1=> vprevious[k+1].powi(2) / 2.0,
            _ =>  0.0};
        fu_prev = match equation {
            0=> fuu * vprevious[k] as f64,
            1=> vprevious[k].powi(2)/ 2.0,
            _ =>  0.0};
        prediction[k] =  vprevious[k] - (dt/dx)*(fu_next - fu_prev); 
        println!("{}", prediction[k]);
        //*prediction = *prediction.wrapping_offset(step);
//Then last backward propagation
        fp_next =  match equation {
            0=> fuu * prediction[k] as f64,
            1=> (prediction[k] * prediction[k])/2.0,
            _ =>  0.0};
        //prediction = prediction.wrapping_offset(-2 * step);
        fp_prev = match equation{
            0=> fuu * prediction[k-1] as f64,
            1=> prediction[k-1] * prediction[k-1] /2.0,
            _ =>  0.0};
        inner_vector[k] = 0.5 * (vprevious[k] + prediction[k] - (dt/dx) * (fp_next - fp_prev));
        if type_of_correction_program {//type_of_correction_program true, then will be used .rs file programm
            monotization_rs(inner_vector, first_correction, second_correction,
                all_steps, smooth_intensity);
        }
        else{
            monotization_c(inner_vector, first_correction, second_correction,
                all_steps, smooth_intensity);
        }
        if k % 5 as usize == 0 && debug_init{
            println!("{} {}, fu_next(u) {}\n", "Array on next layer".italic().yellow(), inner_vector[k], fu_next);
            info!("{}", format!("{} element: with value {}", k, inner_vector[k]));
        }
        else if k % (all_steps / 10_usize) == 0_usize{
            println!("{} {}, fu_next(u) {}\n", "Array on next layer".italic().yellow(), inner_vector[k], fu_next);
            info!("{}", format!("{} element: with value {}", k, inner_vector[k]));
        }
    }
    //Then set Boundary conditions
        if bound == 0 //   non-reflective condition
        {
            inner_vector[0]= inner_vector[1];
            if debug_init{
                println!("Boundary condition established: {}, on dx {} with dt {}", inner_vector[1] == inner_vector[0], dx, dt);
            }
        }
        else 
        {inner_vector[0] = inner_vector[all_steps-2];
            if debug_init{
                println!("Bound condition established: {}...{}...{}", inner_vector[0] == inner_vector[all_steps-2], dx, dt);
            }
        }  
        if bound == 0 
            {inner_vector[all_steps-1]= inner_vector[all_steps-2];}//      v[n]= v[n-1]
        else
            {inner_vector[all_steps-1] = inner_vector[1];}
}

pub fn monotization_rs(inner_vector: &mut Vec<f64>, first_correction: &mut Vec<f64>, second_correction: &mut Vec<f64>,
    all_steps: usize, smooth_intensity: f64){
    if all_steps > MONOTIZATION_MIN && all_steps < MONOTIZATION_MAX {
        println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
            Style::new().foreground(Red).bold().paint("smooth_intensity"),
            Style::new().foreground(Blue).italic().paint("will be smoothed out with rust function 'smoothZF_rs'."));
        smooth_zf_rs(inner_vector, all_steps, smooth_intensity, first_correction, second_correction);
    }
    else{
        println!("Steps must be set to be within range ({}) : {MONOTIZATION_MIN}...{MONOTIZATION_MAX} ", 
            Style::new().foreground(Red).bold().paint("default average - maximum value(200)"));
        panic!("For correction needed another step value!")
    }
}
pub fn monotization_c(inner_vector: &mut Vec<f64>, first_correction: &mut Vec<f64>, second_correction: &mut Vec<f64>,
    all_steps: usize, smooth_intensity: f64){
    if all_steps > MONOTIZATION_MIN && all_steps < MONOTIZATION_MAX {//this will call native program on c
        println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
            Style::new().foreground(Red).bold().paint("smooth_intensity"),
            Style::new().foreground(Blue).italic().paint("will be smoothed out with c function 'Smooth_Array_Zhmakin_Fursenko'."));
        call_smooth(inner_vector, all_steps, smooth_intensity,
            first_correction, second_correction);
    }
    else{
        println!("Steps must be set to be within range ({}) : {MONOTIZATION_MIN}...{MONOTIZATION_MAX} ", 
            Style::new().foreground(Red).bold().paint("default average - maximum value(200)"));
        panic!("For correction needed another step value!")
    }
}
//-----------------------------------------------------------------------------------
pub fn do_exact_solutions(equation: i8, all_steps: usize, start_left: f64, dx: f64, curtime_on_dt: f64, curtime_on_vel: f64, alpha: f64, c: f64, deb_my: bool, 
    vprevious: &mut Vec<f64>, first_ex: &mut  Vec<f64>, second_ex: &mut Vec<f64>)// -> (Vec<f64>, Vec<f64>, Vec<f64>)
    {
    let mut l: f64;
    let mut l_new: usize;
    let mut x_next: f64;
    //let mut h = (all_steps as f64/ print_npy as f64).floor() as usize;
    for k in 0 .. all_steps {
        x_next = start_left + k * dx;
        l =  k as f64 - curtime_on_vel.floor();
        if equation == 0 {
            //On which side exact solution will disapear? depends on sgn velocity
            if velocity <=0.0 {
                if l >= all_steps as f64 {
                    l_new = (l % all_steps as f64) as usize;
                    second_ex[k] = first_ex[l_new].clone();
                }
                else {
                    l_new = if l >= 0.0 {l as usize} else { (all_steps as f64 + l) as usize};
                    second_ex[k] = first_ex[l_new].clone();
                }
            if deb_my { 
                println!("l: {}, curtime_on_vel: {}", l, curtime_on_vel);
            }
        }
        else if velocity >0.0 {
            if l <= 0 as f64 {
                l_new = (l % all_steps as f64).abs() as usize;
                second_ex[k] = first_ex[l_new].clone();
            }
            else {
                l_new = if l >= 0.0 {l as usize} else { (all_steps as f64 + l) as usize};
                second_ex[k] = first_ex[l_new].clone();
            }
        }
    }
    else if equation == 1 {
        first_ex[k]= (alpha * x_next as f64 + c)/
            (alpha * curtime_on_dt + 1.0);
        println!("Exact vector: {}", first_ex[k]);
            //This will work **Only** with lines initial forms
            //Is it needed to live after reaching boundary?
        }
    }   
    if equation ==0 {
        first_ex.copy_from_slice(&second_ex[..]);
        println!("Exact: {:?}\n Numeric: {:?}", first_ex, vprevious);
    }
    //(vprevious, first_ex, second_ex)
}
pub fn calculate_cycles_per_sec(dtotal_loop_nanos: i64){
    if dtotal_loop_nanos - chrono::Duration::seconds(1).num_nanoseconds().unwrap() > 0 {
        
    }
}
pub fn make_vec_output(dtotal_loop_nanos: i64){
    if dtotal_loop_nanos - chrono::Duration::seconds(1).num_nanoseconds().unwrap() > 0 {
}
}
pub fn calculate_output_time_vec_based_on_outtime_rate(all_steps: usize, current_time_on_dt: f64, hor_time_step: usize,
    mut x_index: usize, mut time_output_precised_secs: f64, 
    vector_time: &mut Vec<f64>, vector_time_exact: &mut Vec<f64>, inner_vector: &Vec<f64>, first_ex: &Vec<f64>,
        do_step_reduce_now: bool, print_npy: usize, my_deb: Option<bool>) -> (usize, f64){
        let my_deb = if let Some(debug) = my_deb{
            debug
        }
        else{
            false
        };
//This function determine one horizont layer over unit of time
    if my_deb{
        println!("ST: {}", current_time_on_dt - time_output_precised_secs);
    }
    if (current_time_on_dt - time_output_precised_secs) > 0.0 {
        let mut on_line: usize;
        let mut next_vec_index: usize;
        let mut all_step_size = if do_step_reduce_now{
            print_npy
        }
        else{
            all_steps
        };
            for k in 0 .. print_npy {
            //step over whole horizontal line
                on_line = k * hor_time_step as usize;
            //This measure step as in one_dimentional array
                next_vec_index = x_index + k;
                if my_deb{
                    println!(" on_line: {} ^ inner_vector[on_line] {} = ", on_line, inner_vector[on_line]);
                }
                    vector_time[next_vec_index] = inner_vector[on_line].clone();
                println!("vector_time[next_vec_index]: {}", vector_time[next_vec_index]);
                if my_deb{
                    thread::sleep(Duration::from_secs(1_u64));
                }
                    vector_time_exact[next_vec_index] = first_ex[on_line].clone();
                    if my_deb{
                        println!("current x_index {}, time in exact_vector: {} & time in vector: {}", next_vec_index, vector_time_exact[next_vec_index],
                        vector_time[next_vec_index]);
                    }
                        //thread::sleep(time::Duration::from_millis(400_u64));
                        //println!("Получившиеся значения с шагом {} равны {}\n", k, Vector_time[k+x_index as usize]);
                }//Last step will be exact...
                vector_time[x_index + print_npy] = inner_vector[all_steps-1];
                vector_time_exact[x_index + print_npy] = first_ex[all_steps-1];
                    //will be print_npy + 1 every time
                    x_index= x_index + print_npy as usize;
        time_output_precised_secs += time_output_precised_secs;
        }
        else{

        }
    println!("Value of x_index: {}", x_index);
//as it from beginning had value i, then will be 2i, 3i...
    (x_index, time_output_precised_secs)
    }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
