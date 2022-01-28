//This lib will implement initial interaction in programm(command-line, basic functions, etc.)
#[warn(unused_imports)]
#[macro_use] 
extern crate tcprint;
#[macro_use]
extern crate colorify;
#[macro_use]
extern crate colour;
pub mod initial_data_utils;
pub use crate::initial_data_utils::{PathBuf, function_utils::cfutils::{self, op_sys}};
pub use crate::initial_data_utils::initial_input_structures::{TaskType, TaskTypeCs, FileParametres,FileParametresBuilder, initial_information_of_advection};
pub use crate::initial_data_utils::function_utils::print_macros::macro_lrls;
//use std::time::{Instant};
//use chrono::{Local};
extern crate rand;
use rand::{prelude::*, Rng, SeedableRng};
pub use structopt::StructOpt;
extern crate clap;
use clap::{Arg,ArgGroup, App, SubCommand};
use walkdir::{DirEntry, WalkDir};
use chrono::Duration;
use std::error::Error;

pub const MY_ARGUMENT_PROCESS: bool = true;
pub const ARGUMENTS_PRINT: bool = true;

type MyResult<T> = Result<T, Box<dyn Error>>;
fn advection_input(arg_config: MyConfiguration, def_pathbufs: Vec<PathBuf>) -> MyResult<MyConfiguration>{
    let matches = App::new("Advection")
    .version("0.1")
    .author("Maxim <mmmaximus1403@gmail.com>")
    .about("Does awesome things")
    .arg(Arg::with_name("SWITCH_TIME")
        .short('s')
        .default_value("false")
        .long("switch_time")
        .help("Sets option for taking real-time or dt on every iteration in main.rs"))
    .arg(Arg::with_name("debug")
        .short('d')
        .multiple(true)
        .help("Sets the level of debugging information"))
    .arg(Arg::with_name("amount-of-files")
        .short('q')
        .takes_value(true)
        .default_value("6")
        .help("Sets how many files will be processed[default MAXIMUM_FILES_TO_EXPECT=6]"))
    .arg(Arg::with_name("CORRECTION")
        .short('c')
        .long("correction")
        .help("Sets the input file to use"))
    .arg_group(ArgGroup::with_name("output-style")
            .add_all(vec!["output", "in-file"])
            .required(true))
    .arg(Arg::with_name("Path_to_files")
        .short('f')
        .long("file-paths")
        .multiple(true)
        //.conflicts_with("lines")
        .help("Gives your own path to main")
        .takes_value(true))
        .get_matches();
    let switch_time = matches.value_of("switch_time").unwrap_or("default.conf");
    if ARGUMENTS_PRINT{
        println!("Value for SWITCH_TIME: {}", switch_time);}
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
#[derive(Default, Debug, PartialEq)]
struct MyConfiguration {
    // Option defaults to None
    output: Option<PathBuf>,
    // Vecs default to empty vector
    search_path: Vec<PathBuf>,
    // Duration defaults to zero time
    timeout: Duration,
    // bool defaults to false
    check: bool,
}
impl MyConfiguration {
    // add setters here
}
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

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
*/
