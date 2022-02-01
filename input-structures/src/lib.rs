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
pub use crate::initial_data_utils::{PathBuf,Path, function_utils::{ cfutils::{self, Argumento, 
    run, parse_pair, parse_three, op_sys, parse_positive_int, create_output_dir}}};
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
use rand::{prelude::*, Rng, SeedableRng};
pub use structopt::StructOpt;
use clap::{ ColorChoice, Arg, ArgGroup, App};
use clap::{app_from_crate, arg, crate_name};
use walkdir::{DirEntry};
use std::time::Duration;
use std::{io::Write, fs::read_to_string, env, error::Error};
use std::time::Instant as SInstant;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;
use log::info;

pub const MY_ARGUMENT_PROCESS: bool = true;
pub const ARGUMENTS_PRINT: bool = true;
pub const PROCESS_DETAIL: bool = true;

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
type StdtResult<T> = std::result::Result<(Vec<T>, Vec<String>), Box<dyn Error>>;
pub fn process_files<'a>(new_path_obj: &'a mut Vec<PathBuf>, num_files: Option<usize>, db: Option<bool>, should_sleep: Option<bool>, init_dir: Option<String>) 
-> StdtResult<FileParametres>
{
    use std::fs::File;
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
            add_arg: {dataadd}  {sep} 
            margin_domain: {data3:?} {sep} 
            time_eval_period_stage: {data4:?} {sep} 
            bound_type: {data5}  {sep}  
            init_type: {data6}  {sep}  
            init_conditions: {data7:?} {sep} 
            quantity_split_nodes: {data8:?} {sep} 
            n_corant: {data9}  ",data1 = new_init_data[0], data3 = (x_min,x_max), data4 =  (t1,t2),//parse_pair(&init[2..4],","),
            data5 = new_init_data[3], data6 = new_init_data[4], data7 =(i1,i2,Some(i3)),// parse_three(String::as_str(String::from(init[6..8])),","),  
            data8 = new_init_data[6], data9 = new_init_data[7], dataadd =  new_init_data[8], sep = ',')).as_bytes());
            if additional_print{
                println!("{:?} ", err );}
            let all_datas =  FileParametres::new(new_init_data[0].parse::<i8>().unwrap(), (x_min,x_max),
                (t1, t2, false), new_init_data[3].parse::<i8>().unwrap(), new_init_data[4].parse::<i8>().unwrap(), (i1, i2, i3, 0_f64),
                new_init_data[6].parse::<f64>().unwrap(), new_init_data[7].parse::<f64>().unwrap(),
            //Here I pass additional arguments!If not 0=> will be BURGER type, if !=0, then type TRANSFER
                (TaskType::Transfer{a: new_init_data[8].trim().parse().unwrap_or(0_f64)}, 0_i8, false)).unwrap();
        if additional_print{
            println!("{}{:#?}\n",ansi_term::Colour::Cyan.on(ansi_term::Colour::Green).paint("From file: "), all_datas);}
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
        let message_from_thread="The child thread ID: ".to_string();
        let len_dots= message_from_thread.len();
        //println!("{m:?} {0:?}", &files_vec, m= message_from_thread);
        let repeated: String= std::iter::repeat(".").take(len_dots).collect();
        println!("{:?}", repeated);
    return});
let result = files_vec.lock().unwrap().to_vec().clone();
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
pub fn main_initialization(steps: usize, debug_init: bool){
    use std::time::Instant;
    let init_t  = std::time::Instant::now();
    println!("{}", Style::new().foreground(Blue).italic().paint("Constructing array \nfor saving values of function"));
    let mut vprevious = vec![0_f64; steps as usize + 2 as usize];
    if debug_init{
        println!("Size {} steps {}\n", vprevious.len(), steps as f32);
        assert!(vprevious.len() == steps+2);
        let values_all_same = vprevious.iter()/*.inspect(|val| println!("Inspect on size now-{}",val))*/.all(|& x| x == vprevious[0]);
        println!("All array's dtypes values the same?{}", values_all_same);
    }
    let mut inner_vector = vec![0_f32; steps as usize + 2 as usize]; // As next time step to vprevious
    if debug_init {
        println!("{}: {} # {} ", Style::new().foreground(Blue).italic().paint("Size of inner and previous arrays"), inner_vector.len(), vprevious.len());
        info!("{}== {}?", inner_vector.len(), vprevious.len());
        //They will be exchanging values in main loop.
        std::thread::sleep(std::time::Duration::from_millis(300_u64));
    }
    let mut exact_solvec = vec![vec![0_f32; steps + 2], vec![0_f32;steps + 2], vec![0_f32;steps + 2]];//vec![vec![0_f32;steps + 2], vec![0_f32; steps + 2], vec![0_f32;steps + 2]];
    if debug_init{let all_same_length = exact_solvec.iter().all(|ref v| v.len() == exact_solvec[0].len());
        if all_same_length {
            println!("They're all the same");
        } else {
            println!("They are not the same");
        }
    }
    let elapsed_in = init_t.elapsed();
    if debug_init{
    println!("Elapsed for initialization: {:.2?}", elapsed_in);}
    let new_now = std::time::Instant::now();
    println!("Main initialization: {:?} < {:?}", elapsed_in, new_now.duration_since(init_t));
    info!("Start in determining initial shape");
    let mut first_ex = exact_solvec[0].clone();
    let mut second_ex = exact_solvec[1].clone();
    let mut temporary = exact_solvec[2].clone();
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
