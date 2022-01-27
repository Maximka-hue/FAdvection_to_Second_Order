//This program use nightly rust, so input in terminal: rustup override add nightly &| cargo +nightly 
//Also if you want to use python functions,  i had created pyenv(for experiments with export as i supposed to use, but have problems with build.dependences on windows)
// and also you can set to build dependencies pyO3 for nightly,
//you can activate pyenv or not: pypyint-env\Scripts\activate.bat
//Windows: total path
//cargo run main.rs "C:\Users\2020\RUSTFirstOrderEquation\src\Other\src\TransferBurgerMccornack_iconditions.txt"
//Some other ways for running in win (-d now is needed to work)
//cd C:\Users\2020\RUSTprojects\trburcor      cd C:\Users\*insert_your_name*\RUSTprojects\trburcor
//  cargo run main.rs txt_to_parse\TransferBurgerMccornack_iconditions2.txt -d 
//  cargo run main.rs txt_to_parse\TransferBurgerMccornack_iconditions1.txt  -d
//  txt_to_parse\TransferBurgerMccornack_iconditions11.txt txt_to_parse\TransferBurgerMccornack_iconditions111.txt -d -amf 3 
//Also on Ubuntu:
//I had write on my user defined system: cd /home/computadormaxim/RUSTprojects/trburcor
//cargo run main.rs txt_to_parse\TransferBurgerMccornack_iconditions0.txt  -d
//As this training task for first steps learning language, 
//DESIGNATIONS will be following:📔
//E!xt- Doesn't import crate
//C!ircumvent- desire to do smth else to avoid ... creating temp value, etc.(et cetera)
//W!ork - Doesn't work
//D!esire - I would like to use it, but didn't find appropriate method/way to use for it) 
//*********************************************************************
//#![feature(slice_fill_with)]
#![feature(with_options)] //to enable write in file
#![feature(allocator_api)]    // in fn preprocess_text
#![feature(iter_intersperse)] //in csv writing
#![feature(toowned_clone_into)]
#![feature(slice_ptr_get)]
#[macro_use]
extern crate log;
//extern crate pyo3;
extern crate same_file; 
extern crate num_cpus;
extern crate log4rs;
extern crate libc;

use libc::{c_double, c_int};
pub use log4rs::append::file::FileAppender;
pub use log4rs::encode::pattern::PatternEncoder;
pub use log4rs::config::{Appender, Config, Root};
use log::LevelFilter;
use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashSet;
use std::cmp::Ordering;
//extern crate tokio;
//use std::sync::Arc;
use same_file::Handle;
//use num_traits::bounds::Bounded;//min_value() max_value()
//use num_traits::clamp;
use std::{env, ops::Mul, str::FromStr};
use std::fs::{self, File, write};//to_string
#[macro_use]
pub mod lib;
pub mod smooth; 
use smooth::smoothZF_rs;/*{
    mod src{
    pub mod main;
    pub mod time_code;
    }
*/
use lib::{TypeTsk,  FileParametres, pt, ph};
use std::path::{self, PathBuf, Path};
use std::time::Duration;
use std::time::{self, Instant};
pub(crate) use std::process::Command;
//use walkdir::WalkDir;
use std::process;
use chrono::{DateTime, self, Local, Utc};
use tutil::crayon::Style;
use tutil::crayon::Color::*;
use std::io::{self, stdin, 
    //stdout, Read, 
    BufRead, BufReader, ErrorKind};
use structopt::StructOpt;
use std::fs::OpenOptions;
//extern crate grep_regex;
//use itertools::Itertools;
//___________________________________________________________________________________________________
        //let c: Vec<i32> = a.into_iter().chain(b.into_iter()).collect(); consumed
        //let c: Vec<&i32> = a.iter().chain(b.iter()).collect(); reference
        //let c: Vec<i32> = a.iter().cloned().chain(b.iter().cloned()).collect(); // Cloned
        //let c: Vec<i32> = a.iter().copied().chain(b.iter().copied()).collect(); // Copied
//#[link(name = "smooth")]
extern "C" {
    fn smooth_arr_zm_fur(Fs: *mut c_double, Nmax: c_int, smooth_intensity: c_double, Fi: *mut c_double, Ftd: *mut c_double) ->  c_int;
    fn callback();
}
#[cfg(target_os = "linux")]
fn call_callback() -> Box<Fn()->()>{
    unsafe{
        Box::new(callback())
    }
}

#[cfg(target_os = "linux")]
fn call_smooth(inner_vector: &mut Vec<f32>, all_steps: usize, smooth_intensity: f32, first_correction: &mut Vec<f32>, second_correction: &mut Vec<f32>) 
    //-> Box<Fn(Vec<f32>, i32, f32, Vec<f32>, Vec<f32>) -> i32>
    {
        unsafe{
            smooth_arr_zm_fur(inner_vector.as_mut_ptr() as *mut f64, all_steps as i32, smooth_intensity,
                first_correction.as_mut_ptr() as *mut f64, second_correction.as_mut_ptr() as *mut f64)
        }
    //Box::new(smooth_arr_zm_fur(inner_vector.as_mut_ptr() as *mut f64, all_steps as i32, smooth_intensity,
    //    first_correction.as_mut_ptr() as *mut f64, second_correction.as_mut_ptr() as *mut f64))
}
#[cfg(not(target_os = "linux"))]
fn call_smooth(inner_vector: &mut Vec<f32>, all_steps: usize, smooth_intensity: f32, first_correction: &mut Vec<f32>, second_correction: &mut Vec<f32>) 
    //-> Box<Fn(Vec<f32>, i32, f32, Vec<f32>, Vec<f32>) -> i32>
    {
}

#[derive(Debug,Clone)]
pub struct Argumento{
    pub query : String,
    pub filenames : Vec<String>,
    pub case_sensitive: bool
}

impl Argumento {
    pub fn new(args: &[String]) -> Result<Argumento, &'static str>{
        if args.len() < 3 {
            return Err("parsing args: not enough arguments:\nThis program expect name main.rs + other txts \n\r containing info of initial values");
        }
    let mut args_vec: Vec<String> = Vec::with_capacity(args.len()+5 as usize); 
        for argument in env::args().skip(2) { //skip name of programm 
            if argument == "--help" {
              println!("You passed --help as one of the arguments!");
            }
            else if argument.ends_with(".txt"){
                args_vec.push(argument);
                //pt("argument", None);
                ph(&args_vec);// print name of file first time...
            }
/*Very important!*/else if argument.starts_with("--")|argument.starts_with("-"){
                continue
            }
            else{
                pt!("Now support text files only");
            }
        }
        println!("Vector of passed arguents");
        ph(&args_vec);// below(only convenient print)
        let query = args[1].clone();
        println!("args[1]: ");
        pt!(&query);
    let mut vec_ap: Vec<String> = Vec::with_capacity(5*4);
        for f in args_vec.into_iter(){
            let filename = f.clone();
            vec_ap.push(filename);
    }
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        Ok(Argumento{query,
            filenames: vec_ap,
            case_sensitive})
    }
}
#[macro_export]
#[warn(unused_macros)]
macro_rules! scanline {
    ($x: expr) => ({
    io::stdin().read_line(&mut $x).unwrap();
    });
}
//use tokio::prelude::*;
//use tokio::fs::DirBuilder;
#[path="./lib/src/time_code.rs"]
mod time_code;
use time_code::GlobalExpiredTime;

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
    #[structopt(short = "c", long = "correct", help = "Pass `-h`: correction is needed to optimize computation")]
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
use std::error::Error as SError;//**** 
type StdResult<T> = std::result::Result<T, Box<dyn SError>>;

fn main() -> StdResult<()> {//Result<(), dyn std::error::Error + 'static>
let now = Instant::now();
//Struct with specifics: unpack some arguments
let opt = DebOpt::from_args();
println!("{:?}", opt);
let d: bool = opt.debug;//opt.debug;
let c: bool = opt.correction;
let type_of_correction_program: bool = true;
let time_switch: bool = opt.time_switch;
let time_decrease: f32 = 5.0;
let amf: usize = opt.amount_of_files as usize;
println!("debug- {} correction- {} time_switch- {}", d, c, time_switch);
let num_threads = num_cpus::get();
if d {
    println!("Number of threads on your Computador: {}", num_threads);}
    const SLEEP_LOW: u64 = 100;
    const SLEEP_NORMAL: u64 = 300;
    const SLEEP_HIGH: u64 = 500;
    let mut time_on_sleep_in_main: u64 = 0;
if d{thread::sleep(time::Duration::from_millis(SLEEP_LOW));}
//Then instantiate struct that will measure time(purpose to create this- share with it in threads)
let mut time_inst: GlobalExpiredTime<u32, u32, u32>= GlobalExpiredTime::new(Some(String::from("UTC")));
let env_path = env::current_dir().unwrap();//File for time logging
let new_path = env_path.join(format!(r"src\timing_log"));
fs::create_dir_all(&new_path).unwrap(); //env::temp_dir();
let temp_fi = new_path.join(format!(r"main_time.txt"));
File::create(temp_fi.clone())?;
time_inst.update_loc(Some("Time began counting..."), Some(&temp_fi))?;//Add local time to field:Vec<Local>
    let data: Vec<FileParametres>; //save text files in it
    let dataf = FileParametres::first_initializing().expect("Something wrong in Initializing"); //initialize data from file (as needed in rust)
    //if d{println!("{:#?}", dataf);}
//There will be stored paths to txt files  
let data_directory = PathBuf::new(); 
let next_data_directory= PathBuf::new(); 
const DIFEQTYPES: usize = 4;//Different initial types
let mut data_directories = Vec::with_capacity(amf*DIFEQTYPES + 1);
data_directories.extend(vec![data_directory.clone(); amf * DIFEQTYPES as usize].into_iter());
//Do something...*
    data = process_clfiles(dataf, &mut data_directories, Some(amf), &d)?;
    println!("Here are all files that will be evaluated: {:?}", data_directories);
    data_directories.reverse();
    let iter_over_filepaths = data_directories.into_iter().clone();
    let logfile = FileAppender::builder()
    .encoder(Box::new(PatternEncoder::new("{d} : {m} : {n}\n")))
    .build(format!("log/output_.log"))?;
let config = Config::builder()
    .appender(Appender::builder().build("logfile", Box::new(logfile)))
    .build(Root::builder()
           .appender("logfile")
           .build(LevelFilter::Info))?;
log4rs::init_config(config)?;
for (nf, it_file) in iter_over_filepaths.enumerate(){//To create for separate files separate log_files
    if it_file == PathBuf::from("") {break;}
    info!("Now proccess in main: {:?}", it_file);
    println!("Iterable file: {:?}", it_file);
    //wf(None)?;
    thread::sleep(time::Duration::from_millis(SLEEP_HIGH + SLEEP_NORMAL));
    time_inst.update_loc(Some("After processing arguments: "), Some(&temp_fi))?;
    //let fs_per_band = data.len() / num_threads + 1;
    let file_num= data.len();//Iterate over FileParameters
    //Process several files (maybe in dif threads)
    time_inst.update_loc(Some(&format!("Iteration on number- {}", nf)[..]), Some(&temp_fi))?;
//For every file in vec<files from comand line>
//if d{println!("File number: {}, contain {:?} ", nf, data[nf]);}
/*&String: file_ith_argument*/let fiarg = &data[nf]; //This ith file from command line!
/*type*/             let equation = fiarg.eq_type;
/*nodes*/            let steps = fiarg.quantity_split_nodes as usize;
/*domain*/           let domain = fiarg.margin_domain;
/*step*/             let dx = (domain.1-domain.0)/steps as f32;
/*Courant*/          let co = fiarg.n_corant;
/*Ic*/               let i_parameters = fiarg.init_conditions;
/*It*/               let i_type = fiarg.init_type;
/*Transfer_velocity*/let velocity_t = fiarg.add_args.0.as_ref();
/*period of end and output*/let time_ev = fiarg.time_eval_period_stage;
    //In any way in process_clfile I had written TRANSFER, so i can switch it there
    //But more convienient as I suppose that if TRANSFER=0_f32, then switch)  
        let veloc = match velocity_t.expect("Maybe velocity not specified"){
                TypeTsk::TRANSFER{a} => {if d{println!("Speed: {}", a);
                        thread::sleep(time::Duration::from_millis(SLEEP_LOW));} 
                        a},
                TypeTsk::BURGER{b} => {println!("However, this is burger equation:{}", b); &0_f32},
            };
        let a_positive: bool = veloc > &0_f32; //add parameter to detect sheme later
        info!("Sign of speed: {}\n", a_positive);
//____________________________________________________________________________________________________//
        println!("{}", Style::new().foreground(Blue).italic().paint("Constructing array \nfor saving values of function"));
        //let mut array: [i32; steps] = [0; steps];//additional array(intermidiate)
        //let mut aprev: Vec<f32> = Vec::with_capacity(steps);
        //let mut inner_vector = Vec::<f64>::with_capacity(steps as usize);//then insert in col_vec
        let mut vprevious = vec![0_f32; steps as usize + 2 as usize];
        if d{
            println!("Size {} steps {}\n", vprevious.len(), steps as f32);
            assert!(vprevious.len() == steps+2);
            let values_all_same = vprevious.iter()/*.inspect(|val| println!("Inspect on size now-{}",val))*/.all(|& x| x == vprevious[0]);
            println!("All array's dtypes values the same?{}", values_all_same);
        }
        let mut inner_vector = vec![0_f32; steps as usize + 2 as usize]; // As next time step to vprevious
        if d {println!("{}: {} # {} ", Style::new().foreground(Blue).italic().paint("Size of inner and previous arrays"), inner_vector.len(), vprevious.len());
        info!("{}== {}?", inner_vector.len(), vprevious.len());
        //They will be exchanging values in main loop.
        thread::sleep(time::Duration::from_millis(SLEEP_NORMAL));
        }
        //save on two layers exact solution+ addiotional values on shapes 1,2
        let mut exact_solvec = vec![vec![0_f32; steps + 2], vec![0_f32;steps + 2], vec![0_f32;steps + 2]];//vec![vec![0_f32;steps + 2], vec![0_f32; steps + 2], vec![0_f32;steps + 2]];
        if d{let all_same_length = exact_solvec.iter().all(|ref v| v.len() == exact_solvec[0].len());
            if all_same_length {
                println!("They're all the same");
            } else {
                println!("They are not the same");
            }
        }
        if d{let elapsed_in = now.elapsed();
        println!("Elapsed for initialization: {:.2?}", elapsed_in);}

    //____________________________________________________________________________________________________//
    // for n in dip+1..vprevious.len(){
        //if n% all_steps/print_npy ==0{ x_v_w.write_all(format!("{},{},{}", domain.0 + (n as f32 *dx).floor(),
        //0_f32, 0_f32).as_bytes()).expect("Cannot write init values");}
//}
     //println!("Size of vector:{}, and his elements{} \n",col_vec.len(), col_vec[0].len());
     //let same_vec_sizes = 
     //    col_vec.iter().map(|x|x.len()).all_equal();//checking sizes of two(or more better) inner vectors
     //println!("All vector's lenght the same?{}",same_vec_sizes);
    //if d{let start_point = Utc::now();
    //println!("So now we start primary part of programm at {}", start_point);}
    //use ordered_float::NotNaN;//dependency due to the f32 is not Ord
//Now investigate the first t step and max velocity in this array
info!("Start in determining initial shape");
let mut first_ex = exact_solvec[0].clone();
let mut second_ex = exact_solvec[1].clone();
let mut temporary = exact_solvec[2].clone();//Needed in 1 and 2 shapes
let mut all_steps= vprevious.len()-2;//eliminate in 0/1 shapes additional on bound type knots
//______________________________________________________________________________________//
let path = env::current_dir().unwrap();
let new_path_dif = path.join(format!(r"src\differential_errors{}.txt", nf));
let new_path_all = path.join(format!(r"src\treated_datas_{0}\x_u_w_{0}_0.txt", nf));
//This will be as output to lenght from left boundary, numerical velocity and exact solution
let mut x_v_w = OpenOptions::new()
        .write(true).create(true).open(&new_path_all).expect("cannot open file x_v_w");
        x_v_w.write_all("x, u, w".as_bytes()).expect("write failed");
let mut tr_path = String::new();
tr_path = format!(r"src\treated_datas_{}", nf);
let tr = PathBuf::from(&tr_path[..]);//csv_files
std::fs::create_dir_all(&tr_path)?;
    let smax: f32 = match &equation{
        0 => {let transfer_v = &veloc as &f32;
            //Init parameters to define initial shapes
            let c = i_parameters.0;// 1-5;
            let w = i_parameters.1;// 1-4;
            let h = i_parameters.2.unwrap_or(0_f32);// 1-6;
            if dx == 3f32 || dx ==4f32 && w/dx <2.001 {panic!("You are on the boundary, please enter another values in file");}
            match &i_type //initial values of vectors in TRANSFER
                {
            0=>{println!("Ступенька под уравнение переноса"); 
//Not possible to access values in vector by f32, so we count steps from left bound
/*Let's choose a little more inside shape*/let dip = (w / dx).floor() as usize;//Number of pieces *inside*+2 will be on boundary took in account in array
//if doesn't match on integer w-c/2 =dx*N , dx=0.01,0.1...;
//Start- ceil(), end-floor() + Also because i want to avoid with little step boundaries in my shape (important!)
                let left_b: f32= c-w/2 as f32;
                let right_b= c+w/2_f32;
                let mut x_next;
                let start = (left_b/dx as f32).ceil() as usize;//so it will be inside
                let end = (right_b/dx as f32).floor() as f32;
                info!("Start point(inside): {1} ==? Left boundary: {2} - #Cells inside: {0} ", dip, start, left_b/dx);
                if start > steps || start < (domain.0 as f32/dx).ceil() as usize {
                    println!("Левая|правая точка ступенька вне заданной области");
                    panic!("Out of domain!");} 
                if start as f32 != left_b/dx  {
                    info!("start!=left_b");
                    let add_start = (left_b/dx).floor() as usize;
                    vprevious[add_start] = h;
                    first_ex[add_start] = h;
                    all_steps+=1;}
                for n in 1..dip+1{
                    x_next = start + n as usize;
                    vprevious[x_next] = h.max(-h) as f32;
                    first_ex[x_next] = h.max(-h) as f32;
                    if dip<30{
                    if n%2== 0 && d {println!("Получившиеся значения с шагом {} равны {}\n", n  + start , vprevious[x_next]);}
                    println!("Остальные == 0");
                }
                    else if (n+1)%10 == 0 {println!("Получившиеся значения с шагом {} равны {}\n", n + start, vprevious[x_next]);}
                    //info!("Runge: Step: {} - Value: {} ", n, vprevious[n as usize+start]);
                }
                if right_b/dx != end {
                    println!("...");
                    info!("end != right_b");
                    //let add_end = (right_b/dx).ceil() as usize;
                    vprevious[dip +1 as usize]= h;
                    first_ex[dip +1 as usize] = h;
                    all_steps+=1;
                }
            },
            //let mut z: Coord = [0, 0, 0];
            //for ((zref, aval), bval) in z.iter_mut().zip(&a).zip(&b) {
            //    *zval = aval + bval;
            //}
            1 => {println!("{}", ansi_term::Colour::Yellow.underline().paint("Равнобедренный треугольник под уравнение переноса"));
                    let c = i_parameters.0;
                    let w = i_parameters.1;
                    let h = i_parameters.2.unwrap_or(0.0);
                    let left_b: f32= c-w/2 as f32; 
                    let right_b= c+w/2_f32;
                    //let start = (((c-w/2)as f32*100_f32 as f32).ceil()/((100.0*dx)as f32)) as usize;//С какого кусочка начать
                    let dip = (w as f32/ dx).floor() as usize + 1_usize; //Количество кусочков внутри 1 dx=0.01;
                    let start = (left_b /dx as f32).ceil() as usize; //1-300; this is "picies"
                    let end = (right_b/dx as f32).floor() as f32;
                    if ((c-w/2_f32) as f32)<domain.0 || (c+w/2_f32) as f32>= domain.1 {
                        println!("{}", ansi_term::Style::new().underline().paint("Левая|правая точка треугольник вне заданной области"));
                    }//check left boundary and next left ceil
                    if start as f32 != left_b/dx {
                        let add_start = (left_b/dx).floor() as usize;
                        vprevious[add_start] = 0.0;
                        first_ex[add_start] = 0.0;
                        temporary[add_start] = 2_f32*w/h;//Maybe i want to disturbe it later)(put in not null)
                        all_steps+=1;}
                    //An isosceles(Равнобедреннный) triangle(треугольник)
                    for n in 0..dip/2+1{//this is not odd dip
                        let mut x_next = start + n as usize;
                        vprevious[x_next] = (h as f32 *2_f32) as f32 * (dx*n as f32) /w as f32;
                        first_ex[x_next] = vprevious[x_next].clone();
                        temporary[x_next] = 2_f32*w/h;
                        info!("Triangle: Step: {} - Value: {} ",start+ n, vprevious[(start+n) as usize]);
                        if n > 0 && d {println!("n: {} previous layer: {}", n, vprevious[(start+n) as usize]);}
                    if dip/2<30{
                            if n% dip/2 == 0 && d {
                                println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start as f32, vprevious[n as usize + start as usize]);}
                                println!("Остальные == 0");}
                    else if n+1%10 == 0{
                            println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start as f32, vprevious[n as usize+start as usize]);
                            println!("Остальные == 0");}
                    }
                    for n in dip/2+1..dip+1{
                        let mut x_next = start + n as usize;
                        vprevious[x_next] = h as f32 - (h as f32 *2_f32) as f32 * (dx*(n-dip/2) as f32) /w as f32;
                        first_ex[x_next] = vprevious[x_next].clone();
                        temporary[x_next] = -2_f32*w/h;
                        info!("Triangle: Step: {} - Value: {} ",start+ n, vprevious[(start+n) as usize]);
                        if dip/2 < 11{
                            if n+1% dip/10 == 0 && d {
                                println!("Получившиеся значения с шагом {} равны {}\n", n as f32 + start as f32, vprevious[n as usize + start as usize]);}
                                println!("Остальные ==0");}
                        else if n+1%10 == 0{
                            println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start as f32, vprevious[n as usize+start as usize]);}
                    }
                    if right_b/dx != end {//end in triangle, right boundary to the right
                        println!("...");
                        info!("end != right_b");
                        //let add_end = (right_b/dx).ceil() as usize;
                        vprevious[dip+1 as usize]= 0_f32;
                        first_ex[dip+1 as usize] = vprevious[dip+1 as usize].clone();
                        temporary[dip+1 as usize] = -2_f32*w/h;
                        all_steps+=1;
                    }
                },
            2 =>  //Manage with some differences*
            {pt!(format!("{}", ansi_term::Style::new().underline().paint("Гауссова волна под уравнение переноса")));
                        let m = i_parameters.0 as f32;
                        let d = i_parameters.1 as f32;
                        let cnt: f32 = 1_f32/(d as f32 * (std::f32::consts::PI* 2_f32).sqrt());
                        let start: f32= domain.0;//this is integer parameter:left/right boundary in programm
                        let mut x_next;
                        all_steps = steps;
                        vprevious.resize(all_steps, 0_f32);
                        first_ex.resize(all_steps, 0_f32);
                        second_ex.resize(all_steps, 0_f32);
                        inner_vector.resize(all_steps, 0_f32);
//(domain.0 / dx as f32).ceil() as usize;
                        for n in  0..all_steps {
                            x_next = start + n as f32 * dx;//this neede to be on "domain" scale
                            vprevious[n as usize] = cnt* (-((x_next - m).powi(2))/
                                (2_f32 * d.powi(2))).exp();//exp^self  
                            println!("This is copy from slice*: {}", first_ex[n as usize]);
                            temporary[n as usize] = -cnt* (-((x_next - m).powi(2))/
                                (2_f32 * d.powi(6))).exp();
                            info!("Gauss: Step: {} - Value: {} ", n, vprevious[(start + n as f32) as usize]);
                        }
                        first_ex = vprevious.clone();
                        let maxvalue = vprevious.iter().cloned().fold(0./0., f32::max);
                        info!("Max value in array with gauss wave: {}", maxvalue);
                            println!("MAXIMUM VALUE: {}", maxvalue);//??Why not this as usual max value 1 on y axis??
                        }
            3 => {pt!(format!("{}", ansi_term::Style::new().underline().paint("Синусоида под уравнение переноса")));
            let start = domain.0 as f32;
            let end= domain.1 as f32;
            all_steps = steps;
            vprevious.resize(all_steps, 0_f32);
            first_ex.resize(all_steps, 0_f32);
            second_ex.resize(all_steps, 0_f32);
            inner_vector.resize(all_steps, 0_f32);
            //if start.clamp(f32::MIN, f32::MAX)==start && end.clamp(f32::MIN, f32::MAX)== end{
                let distance= end - start;
                let mut angle: f32;
                let mut x_next;
                const DOUBLE_PI: f32 = 2_f32 * std::f32::consts::PI;
                for n in  0..all_steps {
                    x_next = start + n as f32 * dx;
                    angle = x_next as f32 * DOUBLE_PI / distance;
                    vprevious[n] = angle.sin();
                    info!("Sinusoid: Step: {} - Value: {} ", n , vprevious[n as usize]);
                }
                first_ex[..].copy_from_slice(&vprevious[..]);
            //else{panic!("Too extensive domain!");}
            }
            4 => {pt!(format!("{}", ansi_term::Style::new().underline().paint("Прямая под уравнение переноса")));
            let alpha = i_parameters.1;
            let c = i_parameters.2.unwrap_or(0.0);
            let start = domain.0 as f32;
            let end= domain.1 as f32;
            let mut x_next;
            all_steps = steps;
            vprevious.resize(all_steps, 0_f32);
            first_ex.resize(all_steps, 0_f32);
            second_ex.resize(all_steps, 0_f32);
            inner_vector.resize(all_steps, 0_f32);
            //if start.clamp(f32::MIN, f32::MAX)==start && end.clamp(f32::MIN, f32::MAX)== end{
                //let start_arr: usize = (domain.0 / dx as f32).ceil() as usize;
                for n in  0..all_steps {
                    x_next = start + n as f32 * dx;
                    vprevious[n] = x_next * alpha + c;
                    info!("Line: Step: {} - Value: {} ", n, vprevious[n as usize]);
                }
                first_ex.copy_from_slice(&vprevious[..]);
                //}
            },
            other => println!("Options of initial conditions can be only 0,1,2.... found {}", other)}
            *transfer_v}//     Anyway we return a velocity in TRANSFER: it's constant
        1 =>{println!("{}", ansi_term::Colour::Yellow.underline().paint("Равнобедренный треугольник под уравнение <Бюргеррса>"));
        let c = i_parameters.0;
        let w = i_parameters.1;
        let h = i_parameters.2.unwrap_or(0_f32);
        let fsmax: f32 = match &i_type{
            0 => {println!("Ступенька под уравнение Бюргерса)"); 
            //Not possible to access values in vector by f32, so we count steps from left bound
/*Let's choose a little more inside shape*/let dip = (w / dx).floor() as usize;//Number of pieces *inside*+2 will be on boundary took in account in array
//if doesn't match on integer w-c/2 =dx*N , dx=0.01,0.1...;
//Start- ceil(), end-floor() + Also because i want to avoid with little step boundaries in my shape (important!)
        let left_b: f32= c-w/2 as f32; let right_b= c+w/2_f32;
        let start = (left_b/dx as f32).ceil() as usize;//so it will be inside
        let end = (right_b/dx as f32).floor() as f32;
        info!("Start point(inside): {1} ==? Left boundary: {2} - #Cells inside: {0} ", dip, start, left_b/dx);
        if start > steps || start < (domain.0 as f32/dx).ceil() as usize {
            println!("Левая|правая точка ступенька вне заданной области");
            panic!("Out of domain!");} 
        if start as f32 != left_b/dx  {
            info!("start!=left_b");
            let add_start = (left_b/dx).floor() as usize;
            vprevious[add_start] = h;
            first_ex[add_start] = h;
            all_steps+=1;}
        for n in 1..dip+1{
            let mut x_next = start + n as usize;
            vprevious[x_next] = h.max(-h) as f32;
            first_ex[x_next] = h.max(-h) as f32;
        if dip<30{
            if n%2== 0 && d {println!("Получившиеся значения с шагом {} равны {}\n", n  + start , vprevious[x_next]);}
            println!("Остальные == 0");
        }
    else if (n+1)%10 == 0 {println!("Получившиеся значения с шагом {} равны {}\n", n + start, vprevious[x_next]);}
    //info!("Runge: Step: {} - Value: {} ", n, vprevious[n as usize+start]);
        }
        if right_b/dx != end {
            println!("...");
            info!("end != right_b");
            //let add_end = (right_b/dx).ceil() as usize;
            vprevious[dip +1 as usize]= h;
            first_ex[dip +1 as usize] = h;
            all_steps+=1;
        }h},//.abs()
            1 => //let LeftPoint = IParameters.2 - IParameters.1/2;
            {println!("{}", ansi_term::Colour::Yellow.underline().paint("Равнобедренный треугольник под уравнение переноса"));
            let c = i_parameters.0;
            let w = i_parameters.1;
            let h = i_parameters.2.unwrap_or(0.0);
            let left_b: f32= c-w/2 as f32; 
            let right_b= c+w/2_f32;
            let mut x_next;
            //let start = (((c-w/2)as f32*100_f32 as f32).ceil()/((100.0*dx)as f32)) as usize;//С какого кусочка начать
            let dip = (w as f32/ dx).floor() as usize + 1_usize; //Количество кусочков внутри 1 dx=0.01;
            let start = (left_b /dx as f32).ceil() as usize; //1-300;
            let end = (right_b/dx as f32).floor() as f32;
            if ((c-w/2_f32) as f32)<domain.0 || (c+w/2_f32) as f32>= domain.1 {
                println!("{}", ansi_term::Style::new().underline().paint("Левая|правая точка треугольник вне заданной области"));
            }//check left boundary and next left ceil
            if start as f32 != left_b/dx {
                let add_start = (left_b/dx).floor() as usize;
                vprevious[add_start] = 0.0;
                first_ex[add_start] = 0_f32;
                temporary[add_start] = 2_f32*w/h;//Maybe i want to disturbe it later)(put in not null)
                all_steps+=1;}
            //An isosceles(Равнобедреннный) triangle(треугольник)
            for n in 0..dip/2+1{//this is not odd dip
                x_next = start + n as usize;
                vprevious[x_next] = (h as f32 *2_f32) as f32 * (dx*n as f32) /w as f32;
                first_ex[x_next] = vprevious[x_next].clone();
                temporary[x_next] = 2_f32*w/h;
                info!("Triangle: Step: {} - Value: {} ",start+ n, vprevious[(start+n) as usize]);
                if n > 0 && d {println!("n: {} previous layer: {}",start+ n, vprevious[(start+n) as usize]);}
            if dip/2<30{
                    if n% dip/2 == 0 && d {
                        println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start as f32, vprevious[n as usize + start as usize]);}
                        println!("Остальные == 0");}
            else if n+1%10 == 0{
                    println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start as f32, vprevious[n as usize+start as usize]);
                    println!("Остальные == 0");}
            }
            for n in dip/2+1..dip+1{
                x_next = start + n as usize;
                vprevious[x_next] = h as f32 - (h as f32 *2_f32) as f32 * (dx*(n-dip/2) as f32) /w as f32;
                first_ex[x_next] = vprevious[x_next].clone();
                temporary[x_next] = -2_f32*w/h;
                info!("Triangle: Step: {} - Value: {} ", start+ n, vprevious[(start+n) as usize]);
                if dip/2 < 11{
                    if n+1% dip/10 == 0 && d {
                        println!("Получившиеся значения с шагом {} равны {}\n", n as f32 + start as f32, vprevious[n as usize + start as usize]);}
                        println!("Остальные ==0");}
                else if n+1%10 == 0{
                    println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start as f32, vprevious[n as usize+start as usize]);}
            }
            if right_b/dx != end {//end in triangle, right boundary to the right
                println!("...");
                info!("end != right_b");
                //let add_end = (right_b/dx).ceil() as usize;
                vprevious[dip+1 as usize]= 0_f32;
                first_ex[dip+1 as usize] = vprevious[dip+1 as usize].clone();
                temporary[dip+1 as usize] = -2_f32*w/h;
                all_steps+=1;
            }
                    thread::sleep(time::Duration::from_millis(50_u64));
                    let max_value = vprevious.iter().cloned().fold(0./0., |acc, x| acc*x*0_f32); max_value },
                2 =>  //Manage with some differences*
                {pt!(format!("{}", ansi_term::Style::new().underline().paint("Гауссова волна под уравнение переноса")));
                let m = i_parameters.0 as f32;
                let d = i_parameters.1 as f32;
                let cnt: f32 = 1_f32/(d as f32 * (std::f32::consts::PI* 2_f32).sqrt());
                let start: f32= domain.0;//this is integer parameter:left/right boundary in programm
                let mut x_next;
                all_steps = steps;
                vprevious.resize(all_steps, 0_f32);
                first_ex.resize(all_steps, 0_f32);
                second_ex.resize(all_steps, 0_f32);
                inner_vector.resize(all_steps, 0_f32);
//(domain.0 / dx as f32).ceil() as usize;
                for n in  0..all_steps {
                    x_next = start + n as f32 * dx;//this neede to be on "domain" scale
                    vprevious[n as usize] = cnt* (-((x_next - m).powi(2))/
                        (2_f32 * d.powi(2))).exp();//exp^self  
                    println!("This is copy from slice*: {}", first_ex[n as usize]);
                    temporary[n as usize] = -cnt* (-((x_next - m).powi(2))/
                        (2_f32 * d.powi(6))).exp();
                    info!("Gauss: Step: {} - Value: {} ", n, vprevious[(start + n as f32) as usize]);
                }
                first_ex = vprevious.clone();
                let maxvalue = vprevious.iter().cloned().fold(0./0., f32::max);
                info!("Max value in array with gauss wave: {}", maxvalue);
                    println!("MAXIMUM VALUE: {}", maxvalue);//??Why not this as usual max value 1 on y axis??
                                    maxvalue},
                        /*let max_value = vprevious.iter().max() ; f32::max
                        match max_value {
                            Some(max) => println!( "Max value: {}", max as &f32),
                            None      => println!( "Vector is empty" ),
                        };max_value},*/
                        3 => {pt!(format!("{}", ansi_term::Style::new().underline().paint("Синусоида под уравнение переноса")));
                        let start = domain.0 as f32;
                        let end= domain.1 as f32;
                        all_steps = steps;
                        vprevious.resize(all_steps, 0_f32);
                        first_ex.resize(all_steps, 0_f32);
                        second_ex.resize(all_steps, 0_f32);
                        inner_vector.resize(all_steps, 0_f32);
                        //if start.clamp(f32::MIN, f32::MAX)==start && end.clamp(f32::MIN, f32::MAX)== end{
                            let distance= end - start;
                            let mut angle: f32;
                            let mut x_next;
                            const DOUBLE_PI: f32 = 2_f32 * std::f32::consts::PI;
                            for n in  0..all_steps {
                                x_next = start + n as f32 * dx;
                                angle = x_next as f32 * DOUBLE_PI / distance;
                                vprevious[n] = angle.sin();
                                info!("Sinusoid: Step: {} - Value: {} ", n , vprevious[n as usize]);
                            }
                            first_ex[..].copy_from_slice(&vprevious[..]);
                            let maxvalue = vprevious.iter().cloned().fold(0./0., f32::max);
                    info!("Max value in array with sinusoid: {}", maxvalue);
                    println!("MAXIMUM VALUE: {}", maxvalue);//??Why not this as usual max value 1 on y axis??
                                    maxvalue},
                    4 => {pt!(format!("{}", ansi_term::Style::new().underline().paint("Прямая под уравнение Бюргерсса")));
                    let alpha = i_parameters.1;
                    let c = i_parameters.2.unwrap_or(0.0);
                    let start = domain.0 as f32;
                    let end= domain.1 as f32;
                    let mut x_next;
                    all_steps = steps;
                    vprevious.resize(all_steps, 0_f32);
                    first_ex.resize(all_steps, 0_f32);
                    second_ex.resize(all_steps, 0_f32);
                    inner_vector.resize(all_steps, 0_f32);
                    //if start.clamp(f32::MIN, f32::MAX)==start && end.clamp(f32::MIN, f32::MAX)== end{
                    //let start_arr: usize = (domain.0 / dx as f32).ceil() as usize;
                    for n in  0..all_steps {
                        x_next = start + n as f32 * dx;
                        vprevious[n] = x_next * alpha + c;
                        info!("Line: Step: {} - Value: {} ", n, vprevious[n as usize]);
                            }
                    first_ex.copy_from_slice(&vprevious[..]);
                    let maxvalue = vprevious.iter().cloned().fold(0./0., f32::max);
                    info!("Max value in array with lines: {}", maxvalue);
                    println!("MAXIMUM VALUE: {}", maxvalue);//??Why not this as usual max value 1 on y axis??
                                    maxvalue},
            other => {println!("{} {}", ansi_term::Colour::Red.underline().paint("Options of initial conditions can be only [0...1] found:" ), other);
            0_f32}//We found max velocity for BURGER task!)
        };
        fsmax}
    _ => panic!("Initial equation condition incorrect")}; 
    //That's all types
        /*.iter()
        .max_by(|x, y| x.abs().partial_cmp(&y.abs()).unwrap())
        .filter(|_| ts.len() >= 2).expect("Less than two elements");*/
//________________________________Some precycle clarification_______________________//        
println!("Enter how to measure 'time' in main cycle: true / enter");
    let mut blob: String = "false".to_string();
    scanline!(blob);
    let mut choose_output: bool;
    if blob.to_uppercase() == "TRUE" {
        choose_output = true;}
    else {choose_output = false;}
    let ELEMENTS_PER_RAW_PYARRAY: usize = ((all_steps as f32).floor()) as usize;//This will output array with this or less amount of columns
    if steps < 3 {panic!("Please, less than 3 piceses doesn't work.");}
    let print_npy =
    if steps >3 && steps<20 {choose_output= true; steps}
    else if steps== 21 {choose_output= true; 10_usize}
    else {ELEMENTS_PER_RAW_PYARRAY};
    let existing_time = temporary.into_iter().min_by(|a, b|
        a.partial_cmp(&b).unwrap_or(Ordering::Less)).unwrap_or(0_f32);
    println!("Minimum in temporary error vector: {}", &existing_time); 
    let t_max = -1_f32/existing_time;
        //This will store numerical error rate from exact and numerical solution
        let mut differential_errors = OpenOptions::new()
            .read(false)
            .write(true).truncate(true)
            .create(true)
            .open(&new_path_dif).expect("cannot open file differential_errors");
        differential_errors.write_all("t, norm1, norm2\nt,0,0".as_bytes()).expect("write failed");
        if equation==1{
            println!("Existing minimum time of burger: {} and will live: {}",
                existing_time, t_max);
            info!("{}", format!("Existing minimum time of burger: {}", existing_time));
            info!("{}", format!("And so the maximum live time will be: {}", t_max));
        }
time_inst.update_loc(Some(&format!("Iteration on number- {}", file_num)[..]), Some(&temp_fi))?;
//________________________________________________________________________________________________________________________________// 
let sgn_smax = smax as f32 > 0_f32;//        later to switch scheme equation
println!("Maximum velocity: {}", smax);
if d{thread::sleep(time::Duration::from_millis(SLEEP_HIGH));}
let fuu = match &equation{
    0 => veloc,
    1 => &0f32, //Further in main cycle will determine this
    _ => &0_f32};
        let INACURACY_OUTPUT: usize = print_npy % 6;//       time_vector lenght ...
/*Expire Time*/             let mut maxl_time  = chrono::Duration::seconds(time_ev.0 as i64);//       below, to set precision up to 6 characters after commas 
                            let mut maxl_time_ns = maxl_time.num_nanoseconds().unwrap();
/*Period of output*/        let out_time = chrono::Duration::seconds(time_ev.1 as i64);//(((time_ev.1 * 1000_000_000_f32)) as i64)/1000_000_000_i64
                            let out_time_nanos= out_time.num_nanoseconds().unwrap();
/*step on y*/               let dt = match equation {
                                    0 => if a_positive {co * dx/(smax)} else {co * dx/(-smax)},
                                    1 => if sgn_smax {co * dx/(smax)} else {co * dx/(-smax)},
                                    _ => panic!("Not type match")
                                };
                            let height = (time_ev.0 as f64/dt as f64).ceil() as usize;
                            let width = steps + 2_usize;
                            let mut _array = vec![vec![0; width]; height];
                            let mut grid_raw = vec![0 as f64; width * height];
                            // Vector of 'width' elements slices
                            let mut addition_mccorn_array: Vec<_> = grid_raw.as_mut_slice().chunks_mut(width).collect();
                                let addition_mccorn: &mut [&mut [f64]] = addition_mccorn_array.as_mut_slice();
                                //let raw_mcarray = (&mut (&mut addition_mccorn as *mut f64) as *mut *mut f64) ;//as *mut[ *mut [f64]];
                                //let mut prediction = Box::into_raw(Box::new(0_f64)) as *mut f64;
                                //&mut prediction as *mut f64
let mut _massive: [Box<[f64]>; 3] = [
Box::new([0_f64; 1000]),
Box::new([0_f64; 1001]), 
Box::new([0_f64; 1002])
];
/*
let mut m_width;
let mut data_0;
let mut prediction_0: *mut [f64];
let mut first_correction: *mut [f64];
let mut data_1;
let mut prediction_1: *mut [f64];
let mut second_correction: *mut [f64];
let mut pre_choose;
if all_steps == 1000{
    m_width = 1000;
    data_0 = [0_f64; m_width];
    prediction_0 = data_0.as_mut_slice();
    let end_rounded_up = prediction_0.wrapping_offset(m_width);
    pre_choose = 0;}
else{m_width = 1002;
    data_1 = [0_f64; m_width];
    prediction_1 = data_1.as_mut_slice(); 
    let end_rounded_up = prediction_1.wrapping_offset(m_width);
    pre_choose = 1;}
let step = 1;
let mut grid_raw = vec![0 as f64; width * height];
//let mut prediction: *mut [f64] = [0_f64; 1000];
//prediction = addition_mccorn[0] as *mut [f64];//addition_mccorn[0];

if pre_choose {
    first_correction = addition_mccorn[1];}
else{
    second_correction = addition_mccorn[2];}//(*raw_one).as_mut_ptr()[2]
    */
let mut prediction = vec![0_f32; width];
let mut first_correction = vec![0_f32; width];
let mut second_correction = vec![0_f32; width];
//let mut temporal_vec: Vec<f32> = vec![0_f32;steps]; let temporal_vec = & mut Vec::with_capacity(steps); temporal_vec.resize(steps,0_f32);
//Too much to work with cycles time)
        let zero_one = chrono::Duration::zero();
        let size_time = (print_npy + 1) as usize;// *time_ev.0.ceil()
        //save here for numerical and exact output
        let vec_output = vec![vec![0_f32; time_decrease.ceil() as usize * size_time * time_ev.0 as usize / (time_ev.1 as usize) + 2_usize], 
            vec![0_f32; time_decrease.ceil() as usize * size_time * time_ev.0 as usize / time_ev.1 as usize + 2_usize]];
        let mut vector_time: Vec<f32>= if !time_switch {
            vec_output[0].clone()
        }
            else{
                vec![0_f32; (time_ev.0 / (time_ev.1 * dt)).ceil() as usize + 2_usize]
            };
        let mut vector_time_exact = vec_output[1].clone();//***-maybe per second will be *** cycles(experiment)
        //       Measure all time and save interim processed values
        //let mut vector_time = Vec::with_capacity_in(size_time, System);
        let mut _size_per_second = chrono::Duration::nanoseconds(0_i64);//      Measure loop's time to create vector of corresponding lenght
        let _system_time = std::time::SystemTime::now();//I think this time clock might not overflow,
                                                        //but if you interrupt console- instead correct results, will be "errors" 
        let _min = chrono::Duration::nanoseconds(1001);//         This to avoid time substraction/(add) overflow
//________________________________________________________________________________________________________________________________//  
let mut fu_next: f32 = 0_f32;   let mut fu_prev = 0_f32;
/*This is bound type*/ let bound = fiarg.bound_type;
//Boundery debug check will be there*
let mut dtotal_loop = chrono::Duration::zero();
let mut dtotal_loop_nanos = dtotal_loop.num_nanoseconds().unwrap();
let mut y_index: usize = 0;
let mut x_index: usize = 0;
let mut only_one_check = Some(1_i8);
let mut period: usize = 0;
let mut output_periods = Vec::new();
/*
let path_to_exact= PathBuf::from(format!(r".\src\treated_datas_{}\exact_to_python.txt", nf));//format!("")
std::fs::File::open(&path_to_exact).unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create(&path_to_exact).unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
});
use std::fs::OpenOptions;
let mut exact_solution = OpenOptions::new()
    .write(true).create(true).open(&path_to_exact).expect("cannot open file");*/
let smooth_correction: bool = c;
let smooth_intensity = 0.5;
show_shape(all_steps, print_npy, &vprevious, &first_ex, &tr, nf, "This is the time after initializing shape", Some("the_beggining_shape"));
#[allow(unused_assignments)]
if time_switch {//Cycle time #1 : Better glance first on else branch.
    let mut current_time = 0_f32;//will be increased by every cycle_time loop on constant
    //This additional in opposite to else where i doesn't measure cycle time
    let mut cycle_time = chrono::Duration::nanoseconds(0);
    let mut cycle_time_nanos = cycle_time.num_nanoseconds().unwrap();
    let mut now = Instant::now();        
    let mut begin = Instant::now();
    let mut end = Instant::now();
    let mut fp_next: f32;
    let mut fp_prev: f32;
    'main_cycle: while maxl_time > zero_one{//max_time.checked_sub(duration).unwrap_or(0) >  Duration::new(0,1000){
        if d {println!("all_steps: {}", all_steps);}
        time_inst.update_loc(Some(&format!("Iteration on number- {}", nf)[..]), Some(&temp_fi))?;
        info!("{}", format!("Main cycle in previous time: {}", dtotal_loop_nanos));
        //{let ref second = vprevious[1]; //   this equaivalent to second = &vprev...) fn cycle_a(aprev: Vec<f32>,bound: i8, current_time: f32, a_positive)
        //let ref last_iter = vprevious[steps-2];
        //inner_vector[1] = vprevious[steps-1] //Also vector, try to vanish this braces
        begin = Instant::now(); now = Instant::now();
        if (!a_positive && equation==0)||(!sgn_smax && equation==1)  {//f<0
        for k in 0..all_steps-1{//from second to prelast
            //if inner_vector.iter().all(|&x| x<0.00001){
            //let refer: &mut Vec<f32> = &mut vprevious;}
            fu_next = match &equation{
                0=> fuu * vprevious[k+1],
                1=> vprevious[k+1]*vprevious[k+1]/2 as f32,
               _ =>  0_f32};
            fu_prev = match &equation{
                0=> fuu * vprevious[k],
                1=> vprevious[k]*vprevious[k]/2 as f32,
                _ =>  0_f32};
                if c && k!=0{
                    //prediction = prediction.wrapping_offset(step);
                    fp_next =  match &equation{
                        0=> fuu * prediction[k] as f32,
                        1=> prediction[k] * prediction[k]/2 as f32,
                        _ =>  0_f32};
                    //prediction = prediction.wrapping_offset(-2 * step);
                    fp_prev = match &equation{
                        0=> fuu * prediction[k-1] as f32 ,
                        1=> prediction[k-1] * prediction[k-1] /2 as f32,
                        _ =>  0_f32};
                        //prediction = prediction.wrapping_offset(step);
                    prediction[k] =  vprevious[k] - (dt/dx)*(fu_next - fu_prev); 
                    inner_vector[k] = 0.5 * vprevious[k] + prediction[k] - (dt/dx) * (fp_next - fp_prev);
                }
                else {
                    inner_vector[k] = vprevious[k] - (dt/dx)*(fu_next - fu_prev);}
                if smooth_correction && type_of_correction_program{//
                    println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
                    Style::new().foreground(Red).bold().paint("smooth_intensity"),
                    Style::new().foreground(Blue).italic().paint("will be smoothed out with rust function 'smoothZF_rs'."));
                    smoothZF_rs(&mut inner_vector, all_steps, smooth_intensity, &mut first_correction, &mut second_correction);
            }
            else if smooth_correction{
                call_smooth(&mut inner_vector, all_steps, smooth_intensity,
                    &mut first_correction, &mut second_correction);
            }
            if k % print_npy as usize == 0 
                {println!("Array on previous layer {}, fu_next(u) {}\n", vprevious[k], fu_next);
                info!("{}", format!("{} element: with value {}", k, vprevious[k]));}
            }
        }//println!(" fuu<0   \n");  }  
        if (a_positive&&equation==0)||(sgn_smax&&equation==1) { //|| sgnsmax
            for k in 1..all_steps-1{// up to penultimate
                //let refer = &mut vprevious;
                //if inner_vector.iter().all(|&x| x<0.00001){
                 fu_next = match &equation{
                   0=> fuu * vprevious[k],
                   1=> vprevious[k].powi(2)/2 as f32,
                   _ =>  0_f32};
                 fu_prev = match &equation{
                    0=> fuu * vprevious[k-1],
                    1=> vprevious[k-1].powi(2)/2 as f32,
                    _ =>  0_f32};
                    if c {
                        //prediction = prediction.wrapping_offset(step);
                        fp_next =  match &equation{
                            0=> fuu * prediction[k] as f32,
                            1=> prediction[k] * prediction[k]/2 as f32,
                            _ =>  0_f32};
                        //prediction = prediction.wrapping_offset(-2 * step);
                        fp_prev = match &equation{
                            0=> fuu * prediction[k-1] as f32 ,
                            1=> prediction[k-1] * prediction[k-1] /2 as f32,
                            _ =>  0_f32};
                            //prediction = prediction.wrapping_offset(step);
                        prediction[k] =  vprevious[k] - (dt/dx)*(fu_next - fu_prev); 
                        inner_vector[k] = 0.5 * vprevious[k] + prediction[k] - (dt/dx) * (fp_next - fp_prev);
                    }
                    else {
                        inner_vector[k] = vprevious[k] - (dt/dx)*(fu_next - fu_prev);
                    }
                    if smooth_correction && type_of_correction_program{//
                            println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
                            Style::new().foreground(Red).bold().paint("smooth_intensity"),
                            Style::new().foreground(Blue).italic().paint("will be smoothed out with rust function 'smoothZF_rs'."));
                            smoothZF_rs(&mut inner_vector, all_steps, smooth_intensity, &mut first_correction, &mut second_correction);
                    }
                    else if smooth_correction{
                        call_smooth(&mut inner_vector, all_steps, smooth_intensity,
                            &mut first_correction, &mut second_correction);
                    }
                    /*if choose_output== false && k% 2  == 0 {
                        exact_solution.write(&inner_vector[k].to_be_bytes())?;
                    }
                    else if choose_output== true {//This means that there are small amount of pieces in raw
                        exact_solution.write(&inner_vector[k].to_be_bytes())?;
                    }*/
                   //fs::write(format!(r"{}\bound.txt", data_directory.display()), format!(r"{}",&inner_vector[k])).expect("write failed"); 
                    println!("Array  {:.4}, fu_next(u) {:.4}, step {} \n", inner_vector[k], fu_next, k);
                    info!("On step {} value: {:.4} \n", k, inner_vector[k]);
                        //thread::sleep(time::Duration::from_millis(SLEEP_LOW)); 
                        //time_on_sleep_in_main+=SLEEP_LOW;
                    } 
               }//println!(" fuu>0");  } debug!
        //println!("Size of vprevious {}", vprevious.len());
        //let ref second = vprevious[1]; //   this equaivalent to second = &vprev...) fn cycle_a(aprev: Vec<f32>,bound: i8, current_time: f32, a_positive)
        //let ref last_iter = vprevious[steps-2];
        if bound == 0 //   non-reflective condition
        {inner_vector[0]= inner_vector[1];
        println!("Boundary condition established: {}, on dx {} with dt {}", inner_vector[1] == inner_vector[0], dx, dt);
        }
        else 
        {println!("{}-{}", all_steps, inner_vector.len());
        inner_vector[0] = inner_vector[all_steps-1];
        //thread::sleep(time::Duration::from_millis(SLEEP_NORMAL));
        //time_on_sleep_in_main+= SLEEP_NORMAL;
        println!("Bound condition established: {} on dx {} with dt {}", inner_vector[0] == inner_vector[all_steps-1], dx, dt);
    }
        if bound == 0 
            {inner_vector[all_steps-1]= inner_vector[all_steps-2];}//      v[n]= v[n-1]
        else
            {inner_vector[all_steps-1] = inner_vector[1];}
        if inner_vector.iter().all(|&v| v == 0_f32){
                break 'main_cycle;
            }
    end = Instant::now();
//________________________________________________________________________________________________________________________________// 
    cycle_time = chrono::Duration::from_std(end - begin).unwrap();//Duration::nanoseconds(
    cycle_time_nanos = cycle_time.num_nanoseconds().unwrap();
    println!("Duration on cycle: {}", cycle_time);
    println!("Duration on cycle in nanos: {}, millisecs: {}", cycle_time_nanos, cycle_time.num_milliseconds());
    println!("Elapsed: {}", now.elapsed().as_nanos());
    info!("On {} loop engage {} time", y_index, cycle_time_nanos);
    const Y_SPARSE: usize = 1;
    let h = (steps as f32/ print_npy as f32).floor() as usize;
    if d{println!("Steps on write h = {}\n", h);}
//Now let's save datas to create animations further.          
        for k in 0 .. print_npy{//..steps).step_by(steps/11 as usize)
            let on_line = k * h as usize;
            // (next_max_time.num_nanoseconds().unwrap()  - Vec_ttime as i64 * cycle_time.num_nanoseconds().unwrap()).abs()< 2*cycle_time.num_nanoseconds().unwrap()
            if y_index% Y_SPARSE ==0  { //alternatively (ELEMENTS_PER_RAW_PYARRAY - 1_i16)  as u16 == 0_u16
                vector_time[x_index as usize + k as usize] = inner_vector[on_line] as f32;
                if out_time_nanos - dtotal_loop_nanos > 0  {//x_index as i64 * && only_one_check != None
                    println!("Rest on write: {}", out_time_nanos - x_index as i64 * cycle_time_nanos);}
                //else{ out_time_nanos= 2* out_time_nanos;}
                if d{println!("x_index {} ... with value time in vector: {}", x_index + k, vector_time[x_index as usize + k as usize]);}
                //thread::sleep(time::Duration::from_millis(SLEEP_NORMAL));
                //time_on_sleep_in_main+= SLEEP_NORMAL;
                //println!("Получившиеся значения с шагом {} равны {}\n", k, Vector_time[k+x_index as usize]);
            }
        }
            //This will resize over vetor of time into measured on one second!
            println!("DT: {}", dtotal_loop_nanos - chrono::Duration::seconds(1).num_nanoseconds().unwrap());
            //thread::sleep(time::Duration::from_millis(SLEEP_HIGH));
            //time_on_sleep_in_main+= SLEEP_HIGH;
                if dtotal_loop_nanos - chrono::Duration::seconds(1).num_nanoseconds().unwrap() > 0 {
                if let Some(_only_once) = only_one_check{
                    let size_per_second =  dtotal_loop_nanos/cycle_time_nanos;
                    //1sec(+-)/ct = how much cycles process on second
                    //1 ct process 1000(+-) elements, i'm saving print_npy(11) elements
                    //Consequently i need size on time T ...
                    if d{println!("{} {} {}", print_npy, time_ev.0, size_per_second);}
                    vector_time.resize((print_npy as f32 * 2 as f32  * time_ev.0 * size_per_second as f32) as usize, 0_f32);//3_f32-for safity
                    //File::create(format!(r".\src\treated_data_{}\debug.txt", nf))?;
                    //fs::write(format!(r".\src\treated_data_{}\debug.txt", nf), "".as_bytes())?;
                    println!("Resize {}.... {}", vector_time.len(),
                        &dtotal_loop.num_nanoseconds().unwrap());
                        //thread::sleep(time::Duration::from_millis(5000_u64));
                    //thread::sleep(time::Duration::from_millis(50_u64));
                    only_one_check = None;
                }
            //println!("{} ---", x_index);
            //print!("------- {:#?}", Vector_time[x_index as usize..(x_index as usize + additional as usize)]);
        }//Duration::nanoseconds(cycle_time.elapsed().unwrap().as_nanos() as i64); for SystemTime!
        if y_index% Y_SPARSE ==0  {x_index= x_index + print_npy as usize;}
        y_index= y_index + 1;
        //let mut cycle_end: Duration = cycle_time; //_or(Duration::to_std(&zero_one).unwrap().as_nanos()
        //println!("Получившиеся значения с шагом {} равны {}\n", steps-1, inner_vector[steps-1]);
        // Vector_time[vec_index as usize+1] = inner_vector[steps-1 as usize] as f32;
        info!("This time extract cycle_end of one horiz. step(millis) {:?}", cycle_time_nanos / 1000000_i64);
        println!("This time extract cycle_end of one horiz. step(millis) {:?}", cycle_time_nanos / 1000000_i64);
        println!("This time extract cycle_end of one horiz. step(nanoseconds) {:?}", cycle_time_nanos);
        //dur_total_loop.checked_add(&cycle_time);
        if d{println!("dtotal_loop: {:?} dtotal_loop_nanos: {} cycle_time_nanos: {}", dtotal_loop, dtotal_loop_nanos, cycle_time_nanos);}
        //thread::sleep(time::Duration::from_millis(50_u64));
        //cycle_time = zero_one;
        //println!("{}", maxl_time.num_seconds());
        //next_max_time = maxl_time.checked_sub(&dur_total_loop).unwrap_or(zero_one);//_or(Duration::new(0_u64,0));
        if d{println!("maxl_time: {:?}  maxl_time nanoseconds: {}", maxl_time, maxl_time_ns);
        //thread::sleep(time::Duration::from_millis(100_u64));
    }
        if d{println!("duration {:?} time rest {:?} Current time {}" , dtotal_loop, maxl_time, current_time);}
        /*match cycle_time.elapsed(){
            Ok(el) => println!("{}", el.as_nanos());,
            Err(err) => {println!("Error: {:?}", err);zero_one as u128}};*/
        //duration.checked_add(Duration::from_nanos(cycle_end as u64));
        //duration.checked_add(Duration::new(cycle_end as u64,0));
        //dur_total_loop=dur_total_loop + cycle_time;
        //duration = duration + Duration::from_nanos(cycle_end as u64);
        //vprevious.swap_with_slice(&mut inner_vector);//Now inner not interest)
        //inner_vector.fill_with(Default::default());
        vprevious = inner_vector;
       // thread::sleep(time::Duration::from_millis(500_u64)); println!("inner_vector {:?}",vprevious);
        inner_vector = vec![0_f32; all_steps];
        current_time = current_time + dt;//   move up
        dtotal_loop = dtotal_loop.checked_add(&cycle_time).unwrap();
        dtotal_loop_nanos += cycle_time_nanos;
        maxl_time = maxl_time.checked_sub(&cycle_time).unwrap();
        maxl_time_ns-= cycle_time_nanos;
        y_index += 1;//  for output time period
        //thread::sleep(time::Duration::from_millis(500_u64)); println!("inner_vector {:?}\n   vprevious {:?}",inner_vector, vprevious);
       // for k in 0..steps{
            //inner_vector[k] = aprev.get(k).unwrap().clone();
            //if k%100==0 {println!(" inner {}",inner_vector[k]);}
            //aprev = vec![0_f32; steps];
       }
    }
    else{
        //let safety = 0  as usize ;//* time_ev.0 as usize/2_usize all_steps as usize
        //vector_time.resize(all_steps as usize * time_ev.0 as usize / time_ev.1 as usize + safety, 0_f32);
        let mut processed_time= chrono::Duration::nanoseconds(0);
        //let mut processed_time_nanos = processed_time.num_nanoseconds().unwrap();
        let mut current_time_on_dt = 0_f32;//will be increased by every time(dt) loop
        let mut begin= Instant::now(); 
        let mut end = Instant::now();
        let mut vertical_point: f32 = time_ev.1;
        let mut z;            
        let mut fp_next;
        let mut fp_prev;
        while current_time_on_dt < time_ev.0 + 0.1 {
            period+=1;
            println!("Rest time before loop: {}", time_ev.0 - current_time_on_dt);
            if d {println!("all_steps: {}", all_steps);}
            time_inst.update_loc(Some(&format!("Iteration on number- {}", nf)[..]), Some(&temp_fi))?;
            info!("{}", format!("Main cycle in previous time: {}", dtotal_loop_nanos));
            begin = Instant::now();
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
        z= current_time_on_dt * fuu;
        let mut x_next: f32;
        let mut l: f32;
        let mut l_new: usize;
        let mut h = (all_steps as f32/ print_npy as f32).floor() as usize;
        let start: f32= domain.0;//this is integer parameter:left/right boundary in programm
        //println!("{:?}", first_ex);
        for k in 0 .. all_steps{
            x_next = start + k as f32 * dx;
            if equation ==0 {
                l =  k as f32 - (z as f32/h  as f32).floor();
                //println!("l: {}, k: {}", l, k);
                if l>= all_steps as f32{
                    l_new = ((l%all_steps as f32).abs()) as usize;
                    second_ex[k] = first_ex[l_new].clone();
                } 
                else {
                    l_new = if l as i32>=0 {l as usize} else { (all_steps as f32 + l) as usize};
                    second_ex[k] = first_ex[l_new].clone();
                }
            }
            else if equation ==1{
                first_ex[k]= (i_parameters.1*x_next + i_parameters.2.unwrap_or(0_f32))/(i_parameters.1*z+1_f32);
                println!("Exact vector: {}", first_ex[k]);
            }
        }   
        if equation ==0 { first_ex.copy_from_slice(&second_ex[..]);
            println!("Exact: {:?}\n Numeric: {:?}", first_ex, vprevious);}
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
            if (!a_positive && equation==0)||(!sgn_smax && equation==1)  {//f<0
                for k in 0..all_steps-1{//from second to prelast
                    fu_next = match &equation{
                        0=> fuu * vprevious[k+1],
                        1=> vprevious[k+1]*vprevious[k+1]/2 as f32,
                        _ =>  0_f32};
                    fu_prev = match &equation{
                        0=> fuu * vprevious[k],
                        1=> vprevious[k] * vprevious[k]/2 as f32,
                        _ =>  0_f32};
                if c && k!=0 {
                    //*prediction = *prediction.wrapping_offset(step);
                    fp_next =  match &equation{
                        0=> fuu * prediction[k] as f32,
                        1=> prediction[k] * prediction[k]/2 as f32,
                        _ =>  0_f32};
                    //prediction = prediction.wrapping_offset(-2 * step);
                    fp_prev = match &equation{
                        0=> fuu * prediction[k-1] as f32 ,
                        1=> prediction[k-1] * prediction[k-1] /2 as f32,
                        _ =>  0_f32};
                    prediction[k] =  vprevious[k] - (dt/dx)*(fu_next - fu_prev); 
                    inner_vector[k] = 0.5 * vprevious[k] + prediction[k] - (dt/dx) * (fp_next - fp_prev);
                    }
                else {
                    inner_vector[k] = vprevious[k] - (dt/dx)*(fu_next - fu_prev);
                }
                if smooth_correction && type_of_correction_program{//
                    println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
                    Style::new().foreground(Red).bold().paint("smooth_intensity"),
                    Style::new().foreground(Blue).italic().paint("will be smoothed out with rust function 'smoothZF_rs'."));
                    smoothZF_rs(&mut inner_vector, all_steps, smooth_intensity, &mut first_correction, &mut second_correction);
            }
            else if smooth_correction && all_steps>49_usize && all_steps <301_usize {//
                    println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
                        Style::new().foreground(Red).bold().paint("smooth_intensity"),
                        Style::new().foreground(Blue).italic().paint("will be smoothed out with c function 'Smooth_Array_Zhmakin_Fursenko'."));
                    call_smooth(&mut inner_vector, all_steps, smooth_intensity,
                        &mut first_correction, &mut second_correction);
                }
            else if smooth_correction{
                    println!("Steps must be set to default maximum value(200)");
                    panic!("For correction needed another step value!")
                }
                if k % 5 as usize == 0 {
                    println!("Array on next layer {}, fu_next(u) {}\n", inner_vector[k], fu_next);
                    info!("{}", format!("{} element: with value {}", k, inner_vector[k]));
                    }
                }
            }//println!(" fuu<0   \n");  }  
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++//
            if (a_positive&&equation==0)||(sgn_smax&&equation==1) { //|| sgnsmax
                for k in 1..all_steps{// up to penultimate
                     fu_next = match &equation{
                       0=> fuu * vprevious[k],
                       1=> vprevious[k].powi(2)/2 as f32,
                       _ =>  0_f32};
                     fu_prev = match &equation{
                        0=> fuu * vprevious[k-1],
                        1=> vprevious[k-1].powi(2)/2 as f32,
                        _ =>  0_f32};
                        if c {
                            //prediction = prediction.wrapping_offset(step);
                            fp_next =  match &equation{
                                0=> fuu * prediction[k] as f32,
                                1=> prediction[k] * prediction[k]/2 as f32,
                                _ =>  0_f32};
                            //prediction = prediction.wrapping_offset(-2 * step);
                            fp_prev = match &equation{
                                0=> fuu * prediction[k-1] as f32 ,
                                1=> prediction[k-1] * prediction[k-1] /2 as f32,
                                _ =>  0_f32};
                                //prediction = prediction.wrapping_offset(step);
                            prediction[k] =  vprevious[k] - (dt/dx)*(fu_next - fu_prev); 
                            inner_vector[k] = 0.5 * vprevious[k] + prediction[k] - (dt/dx) * (fp_next - fp_prev);
                        }
                        else {
                            inner_vector[k] = vprevious[k] - (dt/dx)*(fu_next - fu_prev);}
                        if smooth_correction && type_of_correction_program{//
                                println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
                                Style::new().foreground(Red).bold().paint("smooth_intensity"),
                                Style::new().foreground(Blue).italic().paint("will be smoothed out with rust function 'smoothZF_rs'."));
                                smoothZF_rs(&mut inner_vector, all_steps, smooth_intensity, &mut first_correction, &mut second_correction);
                        }
                    else if smooth_correction && all_steps>49_usize && all_steps <301_usize{//
                                println!("Now array on next layer with smooth_coef {1}: {0:.2}\n {2}.", smooth_intensity,
                                    Style::new().foreground(Red).bold().paint("smooth_intensity"),
                                    Style::new().foreground(Blue).italic().paint("will be smoothed out with c function 'Smooth_Array_Zhmakin_Fursenko'."));
                                call_smooth(&mut inner_vector, all_steps, smooth_intensity,
                                    &mut first_correction, &mut second_correction);
                            }
                        else if smooth_correction{
                                println!("Steps must be set to default maximum value(200)");
                                panic!("For correction needed another step value!")
                            }
                        if choose_output== false && k% 2  == 0 {
                            //fvec.write(&inner_vector[k].to_be_bytes())?;
                        }
                        else if choose_output== true {//This means that there are small amount of pieces in raw
                            //fvec.write(&inner_vector[k].to_be_bytes())?;
                        }
                       //fs::write(format!(r"{}\bound.txt", data_directory.display()), format!(r"{}",&inner_vector[k])).expect("write failed"); 
                        println!("Array  {:.4}, fu_next(u) {:.4}, step {} \n", inner_vector[k], fu_next, k);
                        if smooth_correction{
                            info!("On step {} value with smooth_correction: {:.4} \n", k, inner_vector[k]);}
                        else{
                            info!("On step {} value: {:.4} \n", k, inner_vector[k]);
                        }
                            //thread::sleep(time::Duration::from_millis(50_u64))
                        }
                   }
            if bound == 0 //   non-reflective condition
            {inner_vector[0]= inner_vector[1];
            if d{println!("Boundary condition established: {}, on dx {} with dt {}", inner_vector[1] == inner_vector[0], dx, dt);}
        }
            else 
            {inner_vector[0] = inner_vector[all_steps-2];
                if d{println!("Bound condition established: {}...{}...{}", inner_vector[0] == inner_vector[all_steps-2], dx, dt);}
            }  
            if bound == 0 
                {inner_vector[all_steps-1]= inner_vector[all_steps-2];}//      v[n]= v[n-1]
            else
                {inner_vector[all_steps-1] = inner_vector[1];}
//______________________________________________________________________________________________//
    if d{println!("Steps on write h = {}\n", h);}
    period+=1_usize;
//Now let's save datas to create animations further.          
    if current_time_on_dt -  (vertical_point/time_decrease).ceil() > -0.001  {
        let mut on_line;
        let mut next_vec_index;
        output_periods.push(period.clone());
        for k in 0 .. print_npy{
            on_line = k * h as usize;
            next_vec_index = x_index as usize + k as usize;
                     //alternatively (ELEMENTS_PER_RAW_PYARRAY - 1_i16)  as u16 == 0_u16
                vector_time[next_vec_index] = inner_vector[on_line] as f32;
                vector_time_exact[next_vec_index] = first_ex[on_line] as f32;
                if d{println!("x_index {}, time in exact_vector: {} & time in vector: {}", next_vec_index, vector_time_exact[next_vec_index],
                    vector_time[next_vec_index]);
                }
                    //thread::sleep(time::Duration::from_millis(400_u64));
                    //println!("Получившиеся значения с шагом {} равны {}\n", k, Vector_time[k+x_index as usize]);
            }
            vector_time[x_index + print_npy] = inner_vector[all_steps-1];
            vector_time_exact[x_index + print_npy] = first_ex[all_steps-1];
            thread::sleep(time::Duration::from_millis(SLEEP_NORMAL));
        }
            if current_time_on_dt - (vertical_point/time_decrease).ceil() > 0.0  {
                x_index= x_index + print_npy as usize;
                vertical_point+= time_ev.1;
            }
            //thread::sleep(time::Duration::from_millis(500_u64));
            if d{
                println!("Current time {} & out timepoint {} & passed period {}", current_time_on_dt, vertical_point, period);}
            //show_shape(all_steps, print_npy, &inner_vector, &first_ex, &tr, nf, "In cycle");
            //vprevious.swap_with_slice(&mut inner_vector);//Now inner not interest)
            //inner_vector.fill_with(Default::default());
            //if c{
                //prediction = prediction.wrapping_offset(-all_steps * step);
                /*unsafe {
                    while prediction != end_rounded_up {
                    //for i in 0..inner_vector.len() {
                        (*prediction) = *x_ptr as f64;
                    }
                    for i in 0..inner_vector.len() {
                        (*prediction)[i] = 0.0;
                    }
                }}
            else {vprevious = inner_vector;}}*/
            vprevious = inner_vector;
            inner_vector = vec![0_f32; all_steps];
            current_time_on_dt += dt;//   move up
            y_index += 1;//output time period
            end = Instant::now();
            processed_time.checked_add(&chrono::Duration::from_std(end - begin).unwrap()).unwrap();
            //Duration::nanoseconds(
        }
    }
    show_shape(all_steps, print_npy, &vector_time, &vector_time_exact, &tr, nf, "This is the time after all processed time.", Some("the_ultimate_shape"));
//vector_time.iter()
//        .filter(|&p| !p.iter().all(|&v| v == 0_f32));
let t_maxx = if equation ==0 {None} else {Some(t_max)};
    save_files(&tr, vector_time, Some(vector_time_exact), (all_steps, Some(domain.0), Some(domain.1), t_maxx), Some(print_npy), nf, Some(output_periods),
        Some(false), Some(true))?; 
}
Ok(())
}

fn show_shape(all_steps: usize, print_npy: usize, numvec: &Vec<f32>, exactvec: &Vec<f32>, dir_from: &PathBuf, nf: usize, desc: & str, time_form: Option<&str>){
let step_by_step = (all_steps  as f32/print_npy as f32).floor() as usize;
let mut next_vec_index =0_usize;
    println!("X , U , U_exact "); 
        //for k in (0..all_steps).step_by(step_by_step){
        let end = print_npy*step_by_step -1_usize;
            for k in 0..print_npy{
            next_vec_index = k * step_by_step; 
            println!("{}  , {:^.5}, {:^.5}", next_vec_index as f32, numvec[next_vec_index], exactvec[next_vec_index]);}
            println!("{}  , {:^.5}, {:^.5}", all_steps as f32, numvec[end], exactvec[end]);

        let pic_path = format!(r"{}\pic_shapes_nf{}{}.txt",  dir_from.display(), nf, time_form.unwrap_or(""));
        let mut pic_file = std::fs::File::create(&pic_path[..]).unwrap_or_else(|error| 
            panic!("Problem creating the file: {:?}", error));
        for k in 0..print_npy{
            pic_file.write_all(format!("{} , {:^.5}, {:^.5}
", k as f32, numvec[k], exactvec[k]).as_bytes()).unwrap();}
            pic_file.write_all(format!("{} , {:^.5}, {:^.5}
", all_steps as f32 -1_f32, numvec[end], exactvec[end]).as_bytes()).unwrap();
            pic_file.write_all(format!("^^^{}\n", desc).as_bytes()).unwrap();
    }

fn create_safe_file(path: &str) -> Result<std::fs::File, std::io::Error>{
let file = std::fs::File::with_options().write(true).open(&path).unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create(&path).unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } 
    else {
        panic!("Problem opening the file: {:?}", error);
    }
    });
Ok(file)
}
fn save_files(dir: &PathBuf, tvector: Vec<f32>, wvector: Option<Vec<f32>>, (steps, left, right, t_max): (usize, Option<f32>, Option<f32>, Option<f32>), elements_per_raw: Option<usize>,
    nf: usize, output_periods: Option<Vec<usize>>, necessity_of_csv: Option<bool>, paraview_format: Option<bool>) -> std::io::Result<()>
{
    use csv::Writer;
    use std::cmp;
    let repeated: String= std::iter::repeat(".").take(20).collect();
    const DEFAULT_ELEMENTS_PER_RAW: usize = 11;
    let raw_size;
    if let Some(elements_per_raw) = elements_per_raw{
    raw_size = elements_per_raw;}
    else{raw_size = DEFAULT_ELEMENTS_PER_RAW;}
    let mut string_raw: String;
    let left = left.unwrap_or(0_f32);
    let right = right.unwrap_or(0_f32);
    let distance = right - left;
    let h = steps as usize/raw_size;
//____________________________//
    let mut next_index;
    let mut x_next: f32;
    let mut on_line: usize;

    let path = env::current_dir().unwrap();
    println!("path {:?} & directory specified {:?} paraview_format: {:?}" , path, dir, paraview_format);
//let mut y_index = 0_usize;
    let mut switch_path = String::new();
    let pypath = format!(r".\{}\to_python_nf{}.txt", dir.display(), nf);
    let expypath = format!(r".\{}\exact_to_python_nf{}.txt", dir.display(), nf);
    let parameters_path = format!(r".\{0}\parameters_nf{1}.txt", dir.display(), nf);
    println!("{} && {} && {}", pypath, expypath, parameters_path);
//First of all: create one/two files in which we will store vector hor. lines
    create_safe_file(&pypath[..])?;
    let mut exact_vector: Vec<f32> = Vec::with_capacity(tvector.len()+1);
    exact_vector = vec!(0_f32; tvector.len());
if let Some(ex) = wvector{
    create_safe_file(&expypath[..])?;
    exact_vector= ex;
}
    //drop(expypath);
//This will create csv like txt files to turn them in paraview window
if paraview_format.unwrap_or(false){
    switch_path = format!(r".\{0}\paraview_datas", dir.display());
    println!("quantity parts size: {}\n paraview path: {}", raw_size, switch_path);
    fs::create_dir_all(&switch_path[..])?;
    let end_of_traverse = (tvector.len() as f32/raw_size  as f32).floor() as usize;
    println!("End of traverse vector with computed values(outer loop): {}", end_of_traverse);
    let mut next_period: usize;
    let mut index_in_periods: usize = 0;
    let mut periods: Vec<usize> = vec![0_usize;  tvector.len()];
    if let Some(periods_) = output_periods{
        periods = periods_;
    }
    println!("{:?}", tvector);
        for y_index in 0.. end_of_traverse{
            let mut any_notnull = tvector[y_index.. y_index + raw_size as usize].iter().any(|&v| v !=0_f32);
            println!("Next cycle {}\nPossible check on non null vector: {}", y_index, any_notnull);
            //Check that vector doesn't contain all zeros
            if any_notnull
            //Iterate over horizontal lines and save in txt
            {
            switch_path = format!(r".\{0}\paraview_datas\x_u_w_{1}_{2}.txt", dir.display(), nf, y_index);
            println!("Now create in paraview csv like txt files with computed exact and numeric values");
            //create_safe_file(&switch_path[..])?;//paraview_txt_file
            let mut my_file = std::fs::File::create(&switch_path[..]).unwrap();//superfluously
            println!("{:?}\n periods.len(): {}", my_file, periods.len());
            if periods.len() !=0{   
                next_period = *periods.get(index_in_periods).unwrap_or(&0_usize);
                //next_period = next_period.unwrap_or(output_periods.get(output_periods.len()- 1));
                println!("{:?}", my_file.write_all("x,exv,numv,passed_period\n".as_bytes()).unwrap());
            }
            else{
                next_period = 0_usize;  
                    my_file.write_all("x,exv,numv\n".as_bytes()).unwrap();
            }  
                for k in 0..raw_size {
                    on_line = h*k;
                    x_next = left + on_line as f32;
                    next_index = k + y_index * raw_size;
                    string_raw = format!(r"{},{},{},{}
", x_next, exact_vector[next_index], tvector[next_index], next_period);
                    my_file.write_all(&string_raw[..].as_bytes()).unwrap();
                    println!("Write after");   
                }
                if y_index != end_of_traverse-1 {   
                    string_raw = format!(r"{},{},{},{}
", steps , exact_vector[raw_size + y_index * raw_size], tvector[raw_size + y_index * raw_size], next_period);
                    my_file.write_all(&string_raw[..].as_bytes()).unwrap();
                }
                else{
                    string_raw = format!(r"{},{},{},{}
", steps , exact_vector[exact_vector.len() -1], tvector[tvector.len() -1], next_period);
                    my_file.write_all(&string_raw[..].as_bytes())?;
                }
                index_in_periods+=1_usize;
            }
        }
    }
    let path_to_read = Path::new(&parameters_path[..]);
    let mut prm= File::with_options().write(true).open(&path_to_read)?;   
    prm.write_all(format!("Printed elements per raw {}\n", raw_size).as_bytes())?;
    if let Some(t_max) = t_max {
    prm.write_all(format!("Maximum live time in burger task: {}\n", t_max).as_bytes())?;
    }
    /*for e in WalkDir::new(".").into_iter().filter_map(|e| e.ok()) {
        if e.metadata().unwrap().is_file() {
            println!("{}", e.path().display());
        }
    }println!("{:?}", repeated);*/
    //close then change
    let necessity_of_csv = necessity_of_csv.unwrap_or(false);//shaded variable
    if necessity_of_csv == true {
        let t = &format!(r".\{0}\csv_nf{1}", dir.display(), nf)[..];
        let new_switch_path: &Path = Path::new(t);
        let mut csv_data_dir = Path::new(new_switch_path);
        let err = fs::create_dir_all(csv_data_dir)?;
        let mut csv_array = vec![vec![0_f32; 2_usize * raw_size];cmp::max(tvector.len(), exact_vector.len())]; //mem::size_of::<f32>() as usize
        let mut cnew_switch_csv_i = csv_data_dir.clone();
        //let mut wtr_inner = Writer::from_path(&cnew_switch_csv_i)?;
        let mut x_index;
        let mut wtr_inner;
        let mut temp_csv;
        for i in 0..(tvector.len() as f32/raw_size  as f32).floor() as usize{
            if tvector[i+1_usize..i+(raw_size-1) as usize].iter().any(|&v| v !=0_f32){
                let csv_i:PathBuf =cnew_switch_csv_i.join(&format!(r"\csv_i{}", i)[..]);
                wtr_inner = Writer::from_path(&csv_i)?;
                for k in 0..raw_size{
                    x_index = k as usize + i * raw_size as usize;
                    csv_array[i].push(tvector[x_index]);
                    csv_array[i + tvector.len()].push(exact_vector[x_index]);
                    let divider_sgn_len = ','.len_utf8() as usize;
                    temp_csv= vec![""; 2_usize * raw_size * divider_sgn_len];
                    let mut temp_slice_vec = csv_array[i].clone().iter().map(|s| s.to_string()).collect::<Vec<String>>();
                    let new_array = temp_slice_vec.into_iter().inspect(|x| println!("about to intersperse: {}", x)).intersperse(",".to_string())
                    .inspect(|x| println!("after intersperse: {}", x)).collect::<Vec<String>>();
            wtr_inner.write_record(&temp_csv).unwrap();
            wtr_inner.flush()?;
                }
            }
        }
    }
    use std::io::SeekFrom;
    use std::io::Seek;
    //let mut csv_array = vec![vec![0_u8; mem::size_of::<f32>() as usize];
    //    raw_size *  mem::size_of::<f32>() as usize]; 
    //if tvector.len().is_nan() || tvector.len() <= exact_vector.len() { exact_vector.len() } else { tvector.len() }
    println!("pypath: {}\nexpypath: {}", pypath, expypath);
    let mut py_file = File::with_options().write(true).open(&pypath)?;
    let mut expy_file =  File::with_options().write(true).open(&expypath)?;
    let new_position_py = py_file.seek(SeekFrom::End(1)).unwrap();
    let new_position_ex = expy_file.seek(SeekFrom::End(1)).unwrap();
    //______________________________________________//
        let mut x_index;
        for i in 0..(tvector.len() as f32/raw_size  as f32).floor() as usize{
            if tvector[i+1_usize..i+(raw_size-1) as usize].iter().any(|&v| v !=0_f32){
                //wtr_inner = Writer::from_path(&cnew_switch_csv_i)?;
                //write!(&mut py_file," ").unwrap();//This will write binary data!
                py_file.write_all(" ".as_bytes())?;
                expy_file.write_all(" ".as_bytes())?;
                for k in 0..raw_size{
                    x_index = k as usize + i * raw_size as usize;//.to_le_bytes()
                    print!("{:.4} , ", tvector[x_index]);
                    if k % (raw_size-1) == 0 
                        {print!("\n");}
                    if k != raw_size-1 {
                        //write!(&mut py_file,"{:.4}", tvector[x_index])?;//also as above!
                        py_file.write_all(format!("{:.4}", tvector[x_index]).as_bytes())?;
                        expy_file.write_all(format!("{:.4}", exact_vector[x_index]).as_bytes())?;
                    }
                    else 
                    {
                        writeln!(&mut py_file, " {:.4}", tvector[x_index])?;
                        println!("end{:?}", repeated);
                        writeln!(&mut expy_file, " {:.4}", exact_vector[x_index])?;
                    }//Maybe ]
            }
            //let s: Box<[f32]> = Box::new(temp_slice_vec);  //This convert slice to vec!
            //let x = s.into_vec();
            //temp_csv.splice(.., .collect().iter().cloned()).collect();//csv_array[i..i+raw_size];
        }
    }
    //writeln!(&mut prm, "     {}", raw_size)?;
    println!("All had been written");
Ok(())
}

fn create_output_dir(fnum: usize, num_files: usize) -> StdResult<( PathBuf, File )>{
//Create file with named fields & Создаем файл с именованными полями
let path = env::current_dir().unwrap();
println!("{} {}", ansi_term::Colour::Cyan.on(ansi_term::Colour::Blue).fg(ansi_term::Colour::Yellow).paint("The current directory is "), path.display());
let new_path = path.join(format!(r"src\treated_datas_{}", fnum));
println!("{} {}", ansi_term::Colour::Cyan.on(ansi_term::Colour::Blue).fg(ansi_term::Colour::Green).paint("new_path is "), new_path.display());
fs::create_dir_all(&new_path).unwrap(); //env::temp_dir();
let temp_fi = new_path.join(format!(r"parameters_nf{}.txt", fnum));
let processed_params =  fs::OpenOptions::new().create(true).write(true)/*.mode(0o770)*/.open(&temp_fi).unwrap_or_else(|error| {
    if error.kind() == ErrorKind::NotFound {
        File::create(&temp_fi).unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        })
    } else {
        panic!("Problem opening the file: {:?}", error);
    }
});
println!("This will be writen later ... {:?} ", processed_params );
thread::sleep(time::Duration::from_secs(1_u64));
let bu = PathBuf::from("src\\unchecked.txt");
let next_pathbuf= if fnum < num_files {temp_fi} else {bu};
Ok((next_pathbuf, processed_params))
}

fn preprocess_text(file: &String)-> StdResult<(Vec<std::string::String>, String)>{
use std::char;
    let file_content = fs::read_to_string(&file)
                .expect("While reading occured an error");
            let crude_data: String = file_content.split("\n ").map(|x| str::to_string(x.trim())).collect();
            println!("{:#?}- unprocessed file with lenght: {}\n", crude_data, crude_data.len());//let mut sep_sgn = String::new();
            let io_sgn = read_string("You can choose the separation sign in the processed file:"); //Какой выбрать знак разделения в обработанном файле
            match io_sgn.1 { //io::stdin().read_line(&mut io_sgn)
                n => {if n<5{
                println!("choose less than than 2 (or several more) separator(s)");
                println!("{} bytes read + 2 for \\n + size(seperator)", n-2);
                    println!("{}", io_sgn.0);
                }
                else if n > 5 && n< 8{
                println!("You choose big sep- {}", io_sgn.0);
                }
                else{println!("To huge sepsign");}}
        //Err(error) => println!("error: {}", error.0 as u8),     >>>>>>>>>>>>>>>>>>>>>
                }
        let rinsed_data: Vec<&str> = crude_data.split("\n").collect();
        println!("Rinsed: {:#?}", &rinsed_data);
        let mut new_init_data = Vec::with_capacity(25);
        let mut rubbish = Vec::with_capacity(25);
        for x in rinsed_data{
            let mut y =  x.trim_matches(char::is_alphabetic)
                .replace(","," ").replace("\r"," ").replace("'","").replace(" ","");//.replace(" ",":");
            let lovely_sgn = '💝';
            let _lh: usize = '💝'.len_utf8();
            let mut b = [0; 4];
            lovely_sgn.encode_utf8(&mut b);
            if y.contains(char::is_numeric) {
            //let num: usize= "💝".chars().count();
                if y.contains('💝') {
                    let r = y.find('💝');
                if let Some(rr)  = r {
                    let (z, zz) = y.split_at_mut(rr);//.chars().next().unwrap()
                    let new_z = z.trim_matches(char::is_alphabetic).replace("'", "").replace("\\", "").replace("\"","");
                    let mut new_zz: &str = &zz[..];// = &zz[rr .. ];
                    new_zz = new_zz.trim_matches(char::is_alphabetic); 
                    //if let Some(rr) =rr {
                    //    z = (&z[rr as usize .. ]).to_string()}
                    rubbish.push(new_zz.to_string());
                    new_init_data.push(new_z.to_string());
                }
            }
                else {
                    y = y.trim_matches(char::is_alphabetic).replace("'", "").replace("\\", "").replace(","," ");
                    new_init_data.push(y);
                }
            }
            else if !y.contains(char::is_numeric) {
                panic!("Expected that in files would be digits.")
            }
               //println!("{:#?}",&y);
            else{
                y = y.trim_matches(char::is_alphabetic).replace("'", "").replace("\\", "").replace(","," ");
                new_init_data.push(y);
                }
            }
            println!("Rb_comments: {:#?}", rubbish);
            //println!("{}",new_init_data.len());
           /*let y = x.retain(|c| c !=',').as_str();
            init[0].push_str(y);*/
        Ok((new_init_data, io_sgn.0))
}
type StdtResult<T> = std::result::Result<Vec<T>, Box<dyn SError>>;

//Обработка аргументов командной строки ref &
pub fn run<'a>(argumento: &'a Argumento)-> Result<(), Box<dyn SError>>
//dyn «динамический» объект типаж 
{
    let mut contents;
    //let args= & argumento;
    //let quant_f = if argumento.filename.len() < 3 {argumento.filename.len()} else{3};
     //(0..quant_f).map(|i| {
        //let aa= &args;
        for file in argumento.filenames.iter(){
            println!("Next file will be: {}", file);
            contents = fs::read_to_string(file)
                    .expect("Something wrong");
println!("With text content in {}:\n{}", file, &contents);}
    //});
    Ok(())
}
fn process_clfiles<'a>(_datas: FileParametres, new_path_obj: &'a mut Vec<PathBuf>, num_files: Option<usize>, db: &bool) 
    -> StdtResult<FileParametres>
    {
        let db = *db;//unsafe{db as *const bool};
        //Creating from parsed arguments in command line Struct Argumento
        let args: Vec<String> = env::args().collect();
            let argumento = Argumento::new(&args).unwrap_or_else(|err| {
                eprintln!("{} {}", Style::new().foreground(Red).bold().paint("Problem parsing arguments: {}"), err);
                process::exit(1);
            });
        //
            if let Err(e) = run(&argumento.clone()) {
                eprintln!("{}", Style::new().foreground(Red).bold().paint(format!("Application error: {}", e)));
                process::exit(1);
            }
//Some prerequisites for processing input files
if let Some(num_files)= num_files{
if num_files==0 {panic!()} else if num_files>5 { pt!("I hope only on less then 5 files)");
        panic!()};}//process::exit()
let num_files= num_files.unwrap();//hide earlier veriable
if db {println!(" {}" , num_files);}
let mut fiter = argumento.filenames.chunks(num_files);
//Process every chunk of 2,3 etc parts in threads
let fp= FileParametres::first_initializing().unwrap();
//let mut vec_of_processes= vec![fp; argumento.filename.len()];
//let mut vec_of_processes= vec![PathBuf::new(); argumento.filename.len()];
let files_vec: Arc<Mutex<Vec<FileParametres>>> = Arc::new(Mutex::new(Vec::with_capacity(num_files as usize *2)));
let mut paths_buf: Vec<PathBuf>= Vec::<PathBuf>::new();
while let Some(next_fvec)= fiter.next(){//divide by num_files(3) per cycle
    let mut iterable= next_fvec.iter();
    let next_elem= iterable.next().unwrap();//above checked- Some exist
    if db {println!("Next file in bundle: {:?} - Current file: {:?}", fiter.next(), next_elem);}
    let clone_arg1 = next_elem.clone();//check above- has at least value one
    let mut clone_arg2: PathBuf= PathBuf::new();
    let mut clone_arg3: PathBuf= PathBuf::new();
    let mut clone_arg4: PathBuf= PathBuf::new();
    let mut clone_arg5: PathBuf= PathBuf::new();
    match num_files{
    1=> {let bw= &mut paths_buf;
        bw.push(PathBuf::from(clone_arg1));
        drop(clone_arg2);drop(clone_arg3);drop(clone_arg4);drop(clone_arg5);}
    2=> {drop(clone_arg3);drop(clone_arg4);drop(clone_arg5);
        let bw= &mut paths_buf;
        bw.push(PathBuf::from(clone_arg1));
        if let Some(clone_2) = iterable.next(){
        clone_arg2= PathBuf::from(clone_2);
        bw.push(clone_arg2);}},
    3=> {drop(clone_arg5);drop(clone_arg4);
        let bw= &mut paths_buf;
        bw.push(PathBuf::from(clone_arg1));
            if let Some(clone_2) = iterable.next(){
                clone_arg2= PathBuf::from(clone_2);
                bw.push(clone_arg2);}
            if let Some(clone_3) = iterable.next(){
                clone_arg3= PathBuf::from(clone_3);
                bw.push(clone_arg3);}
            }
    4=> {drop(clone_arg5);
    let bw= &mut paths_buf;
    bw.push(PathBuf::from(clone_arg1));
        if let Some(clone_2) = iterable.next(){
            clone_arg2= PathBuf::from(clone_2);
            bw.push(clone_arg2);}
        if let Some(clone_3) = iterable.next(){
            clone_arg3= PathBuf::from(clone_3);
            bw.push(clone_arg3);}
        if let Some(clone_4) = iterable.next(){
            clone_arg4= PathBuf::from(clone_4);
            bw.push(clone_arg4);}}
    5=> {pt!("5 files you choose(max)");
    let bw= &mut paths_buf;
    bw.push(PathBuf::from(clone_arg1));
    if let Some(clone_2) = iterable.next(){
        clone_arg2= PathBuf::from(clone_2);
        bw.push(clone_arg2);}
    if let Some(clone_3) = iterable.next(){
        clone_arg3= PathBuf::from(clone_3);
        bw.push(clone_arg3);}
    if let Some(clone_4) = iterable.next(){
        clone_arg4= PathBuf::from(clone_4);
        bw.push(clone_arg4);}
        if let Some(clone_5) = iterable.next(){
            clone_arg5= PathBuf::from(clone_5);
            bw.push(clone_arg5);}},
    _ => (),}
for el in paths_buf.iter(){
    let npb= el.clone();
    new_path_obj.push(npb);
}
let temp_vec= paths_buf.clone();//Clone all vector!
//let mut borrowed_path= paths_buf.clone();
if db{println!("Paths_buf: {:?} - Temp_vec: {:?}", &paths_buf , &temp_vec);}
let paths_hs: HashSet<PathBuf> = temp_vec.into_iter().collect();//remain unique file names
//else if from beggin create HashSet, I would need to check on insert returned value every if let ...
//___________________________________________________
//fetch and modify file data _ извлекаем и изменяем данные файла
let arc_new_path_obj=  Arc::new(Mutex::new(paths_hs));//.clone()
let files_vec = Arc::clone(&files_vec);
//let mut threads = Vec::with_capacity(argumento.filenames.len());
crossbeam::scope(|spawner| {
    spawner.builder()
        .spawn(|_| println!("{}", ansi_term::Colour::Green.dimmed().on(ansi_term::Colour::Blue).paint("A child thread is running in place processing files")))
        .unwrap();
    //let mut files_vec_ref= &mut files_vec;
    for (fi, file) in next_fvec.into_iter().enumerate() {
        let files_vecs=  Arc::clone(&files_vec);
        let fnames= Arc::clone(&arc_new_path_obj);
        let process_handle = spawner.spawn(move |_| { 
    //let io_sgn: String; let mut new_init_data: Vec<String>;
let  (new_init_data, io_sgn) =  preprocess_text(file).unwrap();
        if db {println!("New updated vector\n{:#?}", &new_init_data);}
        let (x_min,x_max) = parse_pair::<f32>(new_init_data[1].as_str(), ':').expect("Second argument margin_domain must be tuple of pair");
        let (i1,i2,i3) = parse_three::<f32>(new_init_data[5].as_str(), ':').expect("Forth argument is init_conditions, must be three digits here");
        let (t1,t2) = parse_pair::<f32>(new_init_data[2].as_str(), ':').expect("3d argument is time,also three digits");
        if db {println!("Domain{:?}, Time{:?}, Initial conditions{:?}", (x_min,x_max), (t1,t2), (i1,i2,i3));}
//I had passed several files, so i need several new files, where will store treated datas
//Создаем файл с именованными полями
//let mut temp_directory = env::temp_dir();
//temp_directory.push("/src");
let (new_buf , mut processed_params)= create_output_dir(fi, num_files).expect("In creating output files error ");
        let pb= new_buf.clone();
        let boo= fnames.lock().unwrap().insert(pb);
    if boo {
    let err= write!(&mut processed_params, "Equation type:{data1}  {sep} 
        Optional argument(velocity): {dataadd}  {sep} 
        Margin domain: {data3:?} {sep} 
        Time evaluation period: {data4:?} {sep} 
        Boundary type: {data5}  {sep}  
        Initial type: {data6}  {sep}  
        Initial conditions: {data7:?} {sep} 
        Quantity split nodes: {data8:?} {sep} 
        Courant number: {data9}  ", data1 = new_init_data[0], data3 = (x_min,x_max), data4 =  (t1,t2),//parse_pair(&init[2..4],","),
        data5 = new_init_data[3], data6 = new_init_data[4], data7 =(i1,i2,Some(i3)),// parse_three(String::as_str(String::from(init[6..8])),","),  
        data8 = new_init_data[6], data9 = new_init_data[7], dataadd =  new_init_data[8], sep = io_sgn);
        println!("{:?} ", err );
        /*let err= processed_params.write_all((format!("equation_type:{data1}  {sep} 
        add_arg: {dataadd}  {sep} 
        margin_domain: {data3:?} {sep} 
        time_eval_period_stage: {data4:?} {sep} 
        bound_type: {data5}  {sep}  
        init_type: {data6}  {sep}  
        init_conditions: {data7:?} {sep} 
        quantity_split_nodes: {data8:?} {sep} 
        n_corant: {data9}  ",data1 = new_init_data[0], data3 = (x_min,x_max), data4 =  (t1,t2),//parse_pair(&init[2..4],","),
        data5 = new_init_data[3], data6 = new_init_data[4], data7 =(i1,i2,Some(i3)),// parse_three(String::as_str(String::from(init[6..8])),","),  
        data8 = new_init_data[6], data9 = new_init_data[7], dataadd =  new_init_data[8], sep = io_sgn)).as_bytes());
        println!("{:?} ", err );*/
        let all_datas: FileParametres;
        all_datas = FileParametres::new(new_init_data[0].to_string(), (x_min,x_max),
        (t1, t2), new_init_data[3].to_string(), new_init_data[4].to_string(), (i1, i2, i3, 0_f32),
        new_init_data[6].to_string(), new_init_data[7].to_string(),
        //Here I pass additional arguments!If not 0=> will be BURGER type, if !=0, then type TRANSFER
        (TypeTsk::TRANSFER{a: new_init_data[8].trim().parse().unwrap_or(0_f32)}, 0_i8, false));
        if db{println!("{}{:#?}\n",ansi_term::Colour::Cyan.on(ansi_term::Colour::Green).paint("From file: "), all_datas);}
        //then push all in earlier created vector for storing processed files
        files_vecs.lock().unwrap().push(all_datas);
    }
    else{println!("{}", ansi_term::Colour::Cyan.on(ansi_term::Colour::Blue).
        fg(ansi_term::Colour::Yellow).paint("This file was already processed"));
        }
    });
    let message_from_thread="The child thread ID: ".to_string();
    let len_dots= message_from_thread.len();
    println!("{m:?} {0:?}", process_handle.thread().id(), m= message_from_thread);
    let repeated: String= std::iter::repeat(".").take(len_dots).collect();
    println!("{:?}", repeated);
        }//Enum fi files!    //assert!(res.is_ok());
    }).unwrap();//Crossbeam!
/*threads.into_iter().for_each(|thread| {
    println!("{m:?} {0:?}", thread.thread().id(), m= message_from_thread);
        thread
            .join();
            //.expect("The thread creating or execution failed !")
    });*/
};//Process all files...
let result = files_vec.lock().unwrap().to_vec().clone();
drop(files_vec);
println!("Processed: {:#?}", result);
Ok(result)  
}     

use std::io::Write;//flush

fn read_string(comment:&str) -> (String, u8) {
    print!("{}", comment);
    io::stdout().flush().expect("flush");
    const ilen: u8 = 20;
    let mut string: String = String::with_capacity(ilen as usize);
    let iolen:u8 = io::stdin().read_line(&mut string).ok().expect("Error read line!") as u8;
    println!("You had written {} bytes", iolen);
        return (String::from(string.trim()), iolen);
    }
        //let mut uniques = HashSet::<char>::new();
        //new_init.split("").collect::<Vec<&str>>();//.chars().collect::<Vec<chars>>();
        //new_init.retain(|e| uniques.insert(e.clone()));
                //let pair = String::from(&init[2]).retain(|c| c !=',');
        //let triple = String::from(&init[5]).retain(|c| c !=',');
        //println!("{:?}",init);
//Для сканирования входных данных
  
//Ищем несколько разделителей
fn parse_pair<T: FromStr>(s : &str, separator :char) -> Option<(T,T)>{
    match s.find(separator){
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])){
                (Ok(l),Ok(r)) => Some((l, r)),
                _ => None
            }
    }
}}
fn parse_three<T: FromStr>(s : &str, separator :char) -> Option<(T,T,T)>{
    let width = separator.len_utf8();
    match s.find(separator){
        None => None,
        Some(index) => {
            match s[index+width..].find(separator){//1ая ветка
           /* None => match (T::from_str(&s[..index]), T::from_str(&s[index+1..])){
            (Ok(_l),Ok(_r)) => None,  //Some((l, r,None)),
            _ => None*/
            None => None,
            Some(indexx) =>{//вторая ветка
            let indexx = indexx + index + width;
            match (
                T::from_str(&s[..index]),
                T::from_str(&s[index+width..indexx]),
                T::from_str(&s[indexx+width..])){
                (Ok(l),Ok(r),Ok(c)) =>Some((l, r,c)),
                _ => None
                }
            }
        }
    }
}}
//From rust cookbook!
use std::io::{Error};
fn wf(_path: Option<&Path>) -> Result<(), Error> {
    let current_dir = env::current_dir()?;
    println!(
        "Let's get access to current dir)\nEntries modified in the last 1 hour in {:?}:",
        current_dir);
    for entry in fs::read_dir(current_dir)? {
        let entry = entry?;
        let path = entry.path();

        let metadata = fs::metadata(&path)?;
        let last_modified = metadata.modified()?.elapsed().unwrap().as_secs();

        if last_modified < 1 * 3600 && metadata.is_file() && path.ends_with(".rs") || path.ends_with("txt"){
            println!(
                "Last modified: {:?} seconds,
                is read only: {:?},
                size: {:?} bytes,
                filename: {:?}",
                last_modified,
                metadata.permissions().readonly(),
                metadata.len(),
                path.file_name().ok_or("No filename").expect("File wf error"),
            );
        }
    let path_to_read = Path::new("save_some_statistic.txt");
    let stdout_handle = Handle::stdout()?;
    let handle = Handle::from_path(path_to_read)?;

    if stdout_handle == handle {
        return Err(Error::new(
            ErrorKind::Other,
            "You are reading and writing to the same file",
        ));//"You are reading and writing to the same file"
    } else {
        
        let file = File::open(&path_to_read)?;
        let file = BufReader::new(file);
        for (num, line) in file.lines().enumerate() {
            println!("{} : {}", num, line?.to_uppercase());
        }
    }
    }    Ok(())
}
// =================================================================
pub fn find(haystack: &str, needle: char) -> Option<usize> {
    for (offset, c) in haystack.char_indices() {
        if c == needle {
            return Some(offset);
        }
    }
    None
}
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
let mut results = Vec::new();
for line in contents.lines() {
    if line.contains(query) {
        results.push(line);
        }
    } 
    results
}
// =================================================================

#[derive(Debug, Clone)]
pub enum ErrInTransferTask{
    DebOptim(i32),
    FileParams(i32),
    ExecMain(String)
}

use std::io::Write;
fn main()-> Result<(), ErrInTransferTask>{
let mut search_error: ErrInTransferTask;//will find error in separate part of program
let mut output_path = PathBuf::new();//Will write results into


    Ok(())
}
fn count1_norm(prev: Vec<f32>,current: Vec<f32>, temp: Vec<f32>){
    
}

fn test<T: AsRef<str>>(inp: &[T]) {
    for x in inp { print!("{} ", x.as_ref()) }
    println!("");
}
//Let's create struct to save vector with initial values and maximal velocity in it.
use std::fmt::{Debug, Display};
#[derive(Builder, Debug, PartialEq)]
struct IVdata<T: Display>{
    ivector: Vec<T>,
#[builder(default = "100")]
    ivecs: usize 
}
impl<T: Display> IVdata<T> 
{/*
    fn rung<U: Sized, V: Debug> (c:U, w:U, h: U, vec_prev: V, dx: U, debug_q: Option<i16>, domain: Option<(i16, i16)>)
    where U: Debug
    {
        println!("Ступенька под уравнеие переноса)"); 
        //We can't access values in vector by f32, so we count steps from left bound
                        let dip = (w / dx) as u32; //Количество кусочков внутри, dx=0.01,0.1...;
                        let start = ((c-w/ 2 as U)/dx); // as f32 | Two times unwrap the same!
                        if start >  domain.unwrap_or((0_i16,0_i16)).0 as f32 || start< domain.unwrap_or((0_i16,0_i16)).0  {
                            println!("Левая|правая точка ступенька вне заданной области");
                            panic!("Out of domain!");} 
                        for n in 0..dip{
                            vec_prev[start as usize + n as usize] = h.max(-h) as f32;
                            if n%25==0 {println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start , vec_prev[n as usize+start as usize]);}}}
    }*/
}
//For simplicity without default)
#[warn(unused_macros)]
macro_rules! rung {
    ($a: expr, $b: expr, $c: expr, $d: expr,$step: expr, $e: expr) => {
        rung($a, $b, $c, $d, $step, $e)
    };
    ($a: expr, $b: expr, $c: expr, $d: expr, $step: expr) => {
        rung($a, $b, $c, $d, $step, step*100)
    };
}

/*
for entry in WalkDir::new("foo").min_depth(1).max_depth(3) {
        println!("{}", entry?.path().display());
    }
    */
/*
pub fn pt<'a, S: AsRef<str>>(data: S, text: Option<S>)
where S: Into<Cow<'a, str>>{
    println!("{}, {:>?}", data.into(), text.as_ref().map(|r| r.as_ref().to_string()));
}
//*****************************************************************************************************************************
}//Some(SelectSpecTypePrp::None),/**/
#[derive(Debug, Clone)]
pub enum ErrInTransferTask{
    DebOptim(i32),
    FileParams(i32),
    ExecMain(String)
}

use std::io::Write;
fn main()-> Result<(), ErrInTransferTask>{
let mut search_error: ErrInTransferTask;//will find error in separate part of program
let mut output_path = PathBuf::new();//Will write results into


    Ok(())
}
fn count1_norm(prev: Vec<f32>,current: Vec<f32>, temp: Vec<f32>){
    
}
fn test<T: AsRef<str>>(inp: &[T]) {
    for x in inp { print!("{} ", x.as_ref()) }
    println!("");
}
//Let's create struct to save vector with initial values and maximal velocity in it.
use std::fmt::{Debug, Display};
#[derive(Builder, Debug, PartialEq)]
struct IVdata<T: Display>{
    ivector: Vec<T>,
#[builder(default = "100")]
    ivecs: usize 
}
impl<T: Display> IVdata<T> 
{/*
    fn rung<U: Sized, V: Debug> (c:U, w:U, h: U, vec_prev: V, dx: U, debug_q: Option<i16>, domain: Option<(i16, i16)>)
    where U: Debug
    {
        println!("Ступенька под уравнеие переноса)"); 
        //We can't access values in vector by f32, so we count steps from left bound
                        let dip = (w / dx) as u32; //Количество кусочков внутри, dx=0.01,0.1...;
                        let start = ((c-w/ 2 as U)/dx); // as f32 | Two times unwrap the same!
                        if start >  domain.unwrap_or((0_i16,0_i16)).0 as f32 || start< domain.unwrap_or((0_i16,0_i16)).0  {
                            println!("Левая|правая точка ступенька вне заданной области");
                            panic!("Out of domain!");} 
                        for n in 0..dip{
                            vec_prev[start as usize + n as usize] = h.max(-h) as f32;
                            if n%25==0 {println!("Получившиеся значения с шагом {} равны {}\n",n as f32 + start , vec_prev[n as usize+start as usize]);}}}
    }*/
}

/*
#[cfg(test)]
mod tests {
use super::*;
fn try_parse(){
    let opt = DebOpt::from_args();
    println!("{:?}", opt);
    }
fn main(){
    try_parse()
    }
}*/
