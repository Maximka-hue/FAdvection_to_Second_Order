//This lib will implement initial interaction in programm(command-line, basic functions, etc.)
#[warn(unused_imports)]
#[macro_use] 
extern crate tcprint;
extern crate colorify;
#[macro_use]
extern crate colour;
extern crate colored;
use colored::Colorize;
pub mod initial_data_utils;
pub use crate::initial_data_utils::{PathBuf,Path, function_utils::cfutils::{self, Argumento, op_sys, parse_positive_int}};
pub use crate::initial_data_utils::initial_input_structures::{TaskType, TaskTypeCs, FileParametres, FileParametresBuilder, initial_information_of_advection};
use crate::initial_data_utils::function_utils::print_macros::macro_lrls;
use rustils::parse::boolean::str_to_bool;
//use std::time::{Instant};
//use chrono::{Local};
use tutil::crayon::Style;
use tutil::crayon::Color::*;
extern crate rand;
use rand::{prelude::*, Rng, SeedableRng};
pub use structopt::StructOpt;
extern crate clap;
use clap::{ColorChoice, Arg, ArgGroup, App};
use walkdir::{DirEntry};
use std::time::Duration;
use std::{env, error::Error};
use std::time::Instant as SInstant;

pub const MY_ARGUMENT_PROCESS: bool = true;
pub const ARGUMENTS_PRINT: bool = true;

type MyResult<T> = Result<T, Box<dyn Error>>;
pub fn advection_input() -> MyResult<(Argumento, MyConfiguration)>{
    let start = SInstant::now();
    let clap_arguments = App::new("Advection").color(ColorChoice::Always)
    .version("0.1")
    .author("Maxim <mmmaximus1403@gmail.com>")
    .about("Does awesome things")
    .arg(Arg::new("SWITCH_TIME")
        .short('s')
        .default_value("false")
        .long("switch_time")
        .help("Sets option for taking real-time or dt on every iteration in main.rs"))
    .arg(Arg::new("debug")
        .short('d')
        .min_values(1)
        .help("Sets the level of debugging information"))
    .arg(Arg::new("CORRECTION")
        .short('c')
        .long("correction")
        .required(false)
        .help("Sets the input file to use"))
    .arg(Arg::new("amount-of-files")
        .short('q')
        .long("fquantity")
        .takes_value(true)
        //.map(parse_positive_int)
        //.map_err(|e| format!("illegal amount of files number -- {}", e))?
        .default_value("6")
        .help("Sets how many files will be processed[default MAXIMUM_FILES_TO_EXPECT=6]"))
    .group(ArgGroup::with_name("output-style")
            .args(&["cli-files", "in-file", "from-directory"])
            .required(true))//Only one of them!
    .arg(Arg::new("path-to-files")
        .short('f')
        .long("file-paths")
        .multiple_occurrences(true)
        .conflicts_with("in-file")
        .help("Gives your own path to main programm")
        .takes_value(true)
        .required(false)
        .requires("cli-files"))
    .arg(Arg::new("dir-to-files")
        .long("dir-path")        
        .takes_value(true)
        .required(false))
        .get_matches();
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
        _ => unreachable!(),
    };
    let switch_time = clap_arguments.value_of("SWITCH_TIME").unwrap_or("false");
    let debug = clap_arguments.value_of("debug").unwrap_or("false");
    let correction = clap_arguments.value_of("CORRECTION").unwrap_or("false");
    let amf = clap_arguments.value_of("amount-of-files").unwrap();
    let out_style = clap_arguments.value_of("output-style").unwrap();
    if ARGUMENTS_PRINT{
        format!("stdout?{}-fileout?{}-from_directory?{}", outcli, from_files, from_directory);
        println!("Value for SWITCH_TIME: {}", switch_time);
    }
    let mut files_str: Vec<String> = Vec::new();
    let mut files_buf: Vec<PathBuf> = Vec::new();
    if outcli {
    files_str = clap_arguments.values_of("path-to-files").clone().unwrap().map(|strs| String::from(strs)).collect::<Vec<String>>();
    files_buf = files_str.clone().into_iter().map(|strin| Path::new(&strin[..]).to_path_buf()).collect();
    println!("{}", "Files collected from terminal: ".italic().yellow());
    for fi in &files_str{
        println!("{}", fi);
        }
    }
    let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
    cyan!("{}", case_sensitive);
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
            debug: str_to_bool(debug),
            amf: str_to_bool(amf),
            correction: str_to_bool(correction),
            out_style: str_to_bool(out_style),
    }} else{MyConfiguration {//this variable suitable for both[from language point]
        search_path: None,
        searched_files: files_buf,
        debug: str_to_bool(debug),
        amf: str_to_bool(amf),
        correction: str_to_bool(correction),
        out_style: str_to_bool(out_style),}};
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
    amf: bool, 
    correction: bool, 
    out_style: bool, 
}

impl MyConfiguration {

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
pub fn is_dir_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

    // add setters here
/*
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
        let err= processed_params.write_all((format!("equation_type:{data1}  {sep} 
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
        println!("{:?} ", err );
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/
