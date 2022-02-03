//Errors in dark red, cyan for info
#![feature(trace_macros)]
pub use std::borrow::Cow;
use crate::initial_data_utils::function_utils::print_macros::macro_lrls::{pt};
extern crate os_type;
use os_type::{ OSType};
extern crate path_clean;
use handle::Handle;
use log::info;
use path_clean::PathClean;
//use print_macros::*;
use std::borrow::Borrow;
use std::{thread, io::{self, SeekFrom, Seek, ErrorKind, BufRead, BufReader, Write, Error},
    path::{Path, PathBuf},{time::{Instant, Duration}}, fs::{self, OpenOptions, File}, env};
use named_tuple::named_tuple;
//use std::path::{Path, PathBuf};
extern crate custom_error;
use custom_error::custom_error;
#[path="./custom_colours.rs"]
pub mod custom_colours;
use walkdir::{WalkDir, DirEntry};
pub use custom_colours::*;
/* Stylisation */
#[warn(unused_imports)]
pub use colour::*;
#[warn(unused_imports)]
pub use tcprint::*;
pub use colorify::*;
pub use simple_colors::{white, red, printlnc, Style as SimStyle,Color as SimColor};
use std::fmt::{Debug, Formatter};
#[warn(unused_imports)]
use text_colorizer::*;
use termion;
use std::sync::{Arc, Mutex};
use std::error::Error as SError;//**** 
type StdResult<T> = std::result::Result<T, Box<dyn SError>>;

//use dao_ansi::color::kinds::{ForegroundColor, BackgroundColor, PrimaryColor};
use better_term::{flush_styles, rainbowify};
/* Regex 
use lazy_static::lazy_static;
use regex::Regex;
use regex::{RegexSet};*/

pub const  SLEEP_PASS: u16 = 0;
pub const  SLEEP_LOW: u16 = 100;
pub const  SLEEP_NORMAL: u16 = SLEEP_LOW * 2;
pub const  SLEEP_HIGH: u16 = SLEEP_NORMAL * 2;
pub const  SLEEP_MAX: u16 = SLEEP_HIGH * 2; 
pub const  ALL_TIMES: [u16; 5] = [SLEEP_PASS, SLEEP_LOW, SLEEP_NORMAL, SLEEP_HIGH, SLEEP_MAX];

pub const  ARGUMENTO_DBGOUT: bool = true;

pub fn op_sys() -> OSType {
    let os = os_type::current_platform();
    println!("Type: {:?}", os.os_type);
    println!("Version: {}", os.version);
    match os_type::current_platform().os_type {
        os_type::OSType::OSX => {
            println!("This is probably an apple laptop!");
            println!("cargo:rustc-link-lib=framework=CoreFoundation");
        }
        os_type::OSType::Ubuntu => {
            println!("This is running Ubuntu Linux!");
        }
        _ => {
            if cfg!(windows) {
                println!("this is windows");
            }
            else{
                println!("Unknown Operating System");}
            }
        }
        os.os_type
    }

pub fn approx_equal (a: f64, b: f64, dp: u8) -> bool {
        let p = 10f64.powi(-(dp as i32));
        (a-b).abs() < p
    }
// Note the use of braces rather than parentheses.
custom_error!{pub ArgumentParseFilesError
    AmountOfFiles{error_description: String, code: u8} = "Problem with file quantities{error_description}\n   error code {code}.",
    FileInitialization{error_description: String, code: u8} = "Problem with Initialization{error_description}\n   error code {code}.",
    FileBuilder{error_description: String, code: u8} = "Problem with Builder{error_description}\n   error code {code}.",
    DebugFormat            = "Debug implementation error"
}

// Foreground Color:前景色（文本颜色）Black Red Green Magenta Cyan Reset
// Background Color:背景颜色 White
pub enum PrintStyle {
    Time,
    Loop,
    Debug,
    Impl,
    Dft,
}
impl simple_colors::custom::Style for PrintStyle {
    fn get_style_code(&self) -> String {
        match self {
            // Style1 will be bold and light blue
            PrintStyle::Time => "\x1b[1m\x1b[94m".to_string(),
            // Style2 will be bold and red
            PrintStyle::Loop =>
                format!(
                    "{}{}",
                    SimStyle::Italic.get_style_code(),
                    SimColor::Magenta.get_style_code()
                ),
                PrintStyle::Debug =>
                format!(
                    "{}{}",
                    SimStyle::Bold.get_style_code(),
                    SimColor::Red.get_style_code()
                ),
                PrintStyle::Impl =>
                format!(
                    "{}{}",
                    SimStyle::Underlined.get_style_code(),
                    SimColor::Cyan.get_style_code()
                ),
                PrintStyle::Dft =>
                format!(
                    "{}{}",
                    SimStyle::Dark.get_style_code(),
                    SimColor::White.get_style_code()
                ),
        }
    }
}
impl PrintStyle{
    pub fn string_value(&self) -> String{
        let value = 
            match self {
            PrintStyle::Time => "Time",
            PrintStyle::Loop => "Loop",
            PrintStyle::Debug => "Debug",
            PrintStyle::Impl => "Impl",
            PrintStyle::Dft => "Dft",
        };
        value.to_string()
    }
}
impl Debug for PrintStyle {
    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        write!(f, "PrintStyle: {:?}", self.string_value())
    }
}

//_____________________________________________________//
named_tuple!(
    #[derive(Clone, Debug, Default, PartialEq)]
    pub struct ChooseSleepTime<'a> {
    pub reason: &'a str,
    pub flag_and_time: (Vec<bool>, Vec<u32>),
    time: [u16; ALL_TIMES.len()],
});

impl ChooseSleepTime<'static>{
    pub fn add_default_time<'a>() -> ChooseSleepTime<'a>{
    //let StartSleep: ChooseSleepTime =
        ChooseSleepTime::new("", (Vec::<bool>::new(), Vec::new()), ALL_TIMES)
    }
    pub fn get_time(&self, get_t: u64) -> u16{
        if get_t< 6{
            match get_t{
                0 => self.time()[0],
                1 => self.time()[1],
                _ => 0_u16,
            }
        }
        else{
            0_u16
        }
    }
    pub fn add_duration(self, add_this: u64, calculated: Option<bool>){
        if let Some(is_calculate) = calculated{
            if is_calculate { 
                std::thread::sleep(Duration::from_millis(add_this as u64));}
        }
        else{
            match add_this{
                0 =>  std::thread::sleep(Duration::from_millis(self.get_time(0_u64) as u64)),
                1 => std::thread::sleep(Duration::from_millis(self.get_time(0_u64) as u64)),
                _ => println!("Something wrong with sleeping")
            }
        }
    }
    /*
    pub fn add_duration(self, add_this: u32, calculated: Option<bool>){
        let to_add_bool: &mut Vec<bool> = &mut self.flag_and_time().0;
        let to_add_time: &mut Vec<u32> = &mut self.flag_and_time().1;
        if let Some(is_calculate) = calculated{
            if is_calculate { 
                std::thread::sleep(Duration::from_millis(add_this as u64));
                //to_add.0.push(true);
                to_add_time.push(add_this);}
            self.flag_and_time().1.push(add_this);
        }
        else{
            match add_this {
                0 => {std::thread::sleep(Duration::from_millis(SLEEP_PASS as u64)); println!("Nothing to add")},
                1 => {std::thread::sleep(Duration::from_millis(SLEEP_LOW  as u64));
                    //to_add.0.push(false);
                    to_add_time.push(SLEEP_LOW as u32);},
                2 => {std::thread::sleep(Duration::from_millis(SLEEP_NORMAL  as u64));
                    //to_add.0.push(false);
                    to_add_time.push(SLEEP_NORMAL as u32);},
                _ => println!("Incorrect input"),
            }
        }
    }*/
}

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
pub fn create_output_dir(mut fnum: usize, num_files: usize, should_sleep: bool, init_dir: Option<&String>) -> StdResult<(usize, PathBuf, String, File )> {
    //Создаем файл с именованными полями
    //let mut temp_directory = env::temp_dir();
    //temp_directory.push("/src");
    let init_dir = Some(PathBuf::from(init_dir.unwrap()));
    let path = if let Some(dir) = init_dir{
            dir}
        else{env::current_dir().unwrap()};
    println!("\n{} {}\n", ansi_term::Colour::Cyan.on(ansi_term::Colour::Blue).fg(ansi_term::Colour::Yellow).paint("The current directory is "), path.display());
    let new_path = path.join(format!(r"treated_datas_{}", fnum));
    println!("{} {}\n", ansi_term::Colour::Cyan.on(ansi_term::Colour::Blue).fg(ansi_term::Colour::Green).paint("new_path is "), new_path.display());
    fs::create_dir_all(&new_path).unwrap();
    let parameter_file = new_path.join(format!(r"parameters_nf{}.txt", fnum));
    fnum +=1;
    let processed_params =  fs::OpenOptions::new().create(true).write(true)/*.mode(0o770)*/.open(&parameter_file).unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create(&parameter_file).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    println!("This will be writen later ... \n{:?} ", processed_params );
    if should_sleep{
        thread::sleep(Duration::from_secs(1_u64));}
    let bu = PathBuf::from("src\\unchecked.txt");
    let next_pathbuf= if fnum < num_files {parameter_file} else {bu};
    let next_str = next_pathbuf.clone().into_iter().map(|h| String::from(h.to_string_lossy())).collect();
    Ok((fnum, next_pathbuf, next_str, processed_params))
}
    

pub fn write_at_end<W: Write + Seek>(writer: &mut W, amount_of_files: usize) -> io::Result<()> {
        writer.seek(SeekFrom::End(0))?;
        for i in 0..(amount_of_files+1_usize) {
            writer.write("\n{i}".as_bytes())?;
        }
        // all went well
        Ok(())
    }
fn read_file_or_stdin() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "-";
    // These must live longer than `readable`, and thus are declared first:
    let (mut stdin_read, mut file_read);
    
    // We need to ascribe the type to get dynamic dispatch.
    let readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };
    
    // Read from `readable` here.
    
    Ok(())
    }
type MyResult<T> = Result<T, Box<dyn std::error::Error>>;
pub fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}    
// --------------------------------------------------
pub fn parse_positive_int(val: &str) -> MyResult<usize> {
    match val.parse() {
        Ok(n) if n > 0 => Ok(n),
        _ => Err(From::from(val)),
    }
}

pub fn traverse_not_hidden_files(PATH_DEBUG_INFO: bool, MAXIMUM_FILES_TO_EXPECT: usize, input_fpath: &PathBuf) -> Vec<PathBuf> {
    let mut all_txt: Vec<PathBuf> = Vec::new();
    let walker = WalkDir::new(&input_fpath).into_iter();
    for entry in walker.filter_entry(|e| !is_dir_hidden(e)) {
        all_txt.push(PathBuf::from(entry.unwrap().path().clone()));
        }
        //First is directory itself
        let all_txt = all_txt[1..MAXIMUM_FILES_TO_EXPECT+1usize].to_vec();
        if PATH_DEBUG_INFO{ 
        for path_txt in &all_txt{
            println!("{}", path_txt.display());}
        }
        all_txt
}
pub fn is_dir_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
         .to_str()
         .map(|s| s.starts_with("."))
         .unwrap_or(false)
}

    // --------------------------------------------------
#[derive(Debug, Clone)]
pub struct Argumento{
    pub query: String,
    pub filenames: Vec<String>,
    pub case_sensitive: bool,
}
    
impl Argumento {
    pub fn new(args: &[String]) -> Result<Argumento, ArgumentParseFilesError>  {
        if args.len() < 3 {//trace_macros!(true);
            //I determine the lowest limit of txt args[at least one]
            return Err(ArgumentParseFilesError::AmountOfFiles{
                error_description: format!(
                    "{}parsing args: not enough arguments:
                    \nThis program expect name main.rs + other txts 
                    \n\r containing info of initial values", termion::color::Bg(termion::color::Red)),
                    code: 1})
                }
        //Skipping the first argument as it's program name [cargo run main.rs ...]
        let mut args_vec: Vec<String> = Vec::with_capacity(args[2..].len() as usize); 
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        cyan!("{}", case_sensitive);
        for argument in env::args().skip(2) { //skip name of programm 
            if argument == "--help" {
                cyan!("You passed --help as one of the arguments!");
            }//below(pt/ptc is only convenient print)
            else if argument.ends_with(".txt"){
                args_vec.push(argument);
                    if ARGUMENTO_DBGOUT{// print name of file first time...
                    pt!("Arguments as supposed determining files.txt", "impl", &args_vec, PrintStyle::Debug);
                }
            }
/*Very important!*/else if argument.starts_with("--") | argument.starts_with("-"){
                    continue
            }
            else{
                if ARGUMENTO_DBGOUT{
                pt!("Now support text files only", "debug");}
            }
        }
        cyan!("Vector of passed arguents");
        if ARGUMENTO_DBGOUT{
            pt!("Debug check for txt files", "dbg", &args_vec, PrintStyle::Debug);
        }
        let query = args[1].clone();
        println!("args[1]: {}", query);
        pt!(&query, "impl");
        let mut vec_ap: Vec<String> = Vec::with_capacity(5*4);
        for f in args_vec.into_iter(){
            let filename = f.clone();
            vec_ap.push(filename);
    }
    Ok(Argumento{query,
        filenames: vec_ap,
        case_sensitive})
    }
}

pub fn preprocess_text(file: &String)-> Result<(Vec<std::string::String>, String), ()>{
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
    fn read_string(comment:&str) -> (String, u8) {
        print!("{}", comment);
        io::stdout().flush().expect("flush");
        const ilen: u8 = 20;
        let mut string: String = String::with_capacity(ilen as usize);
        let iolen:u8 = io::stdin().read_line(&mut string).ok().expect("Error read line!") as u8;
        println!("You had written {} bytes", iolen);
            return (String::from(string.trim()), iolen);
    }
pub fn show_shape(all_steps: usize, print_npy: usize, numvec: &Vec<f64>, exactvec: &Vec<f64>, 
                calculation_path: &PathBuf, nf: usize, desc: &str, time_form: Option<&str>, deb_my: bool){
    //Will be less than (print_npy - 1) * step_by_step
        let step_by_step = (all_steps  as f64 /print_npy as f64).floor() as usize;
        let mut next_vec_index = 0_usize;
        println!("X, U, U_exact ");
        let end = print_npy * step_by_step - 1_usize;
            for k in 0..print_npy {
                next_vec_index = k * step_by_step; 
                if deb_my{
                    println!("{}  , {:^.5}, {:^.5}", next_vec_index as f32, numvec[next_vec_index], exactvec[next_vec_index]);
                }
            }
            info!("{}  , {:^.5}, {:^.5}", all_steps as f32, numvec[end], exactvec[end]);
            let pic_path = &calculation_path.join(format!(r"pic_shapes_file_num{}_{}.txt", nf, time_form.unwrap_or("")));
            let mut pic_file = OpenOptions::new()
                .write(true).create(true).open(&pic_path).expect("cannot open file");
            for k in 0..print_npy {
                pic_file.write_all(format!("{} , {:^.5}, {:^.5}
                    ",k as f64, numvec[k], exactvec[k]).as_bytes()).unwrap();
            }
            pic_file.write_all(format!("{} , {:^.5}, {:^.5} \n
                ",end as f64 - 1_f64, numvec[end], exactvec[end]).as_bytes()).unwrap();
            pic_file.write_all(format!("^^^{}\n", desc).as_bytes()).unwrap();
            }

use std::str::FromStr;
///These functions search delimeters [first from book Jim Blandy and latter my improved version]
//Ищем несколько разделителей
pub fn parse_pair<T: FromStr>(s : &str, separator :char) -> Option<(T,T)>{
    match s.find(separator){
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index+1..])){
                (Ok(l),Ok(r)) => Some((l, r)),
                _ => None
            }
    }
}}
pub fn parse_three<T: FromStr>(s : &str, separator :char) -> Option<(T,T,T)>{
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
    }
}
//________________________Additional+++++++++++++++++++++++++++++++++++++
pub fn absolute_path(path: impl AsRef<Path>) -> io::Result<PathBuf> {
    let path = path.as_ref();

    let absolute_path = if path.is_absolute() {
        path.to_path_buf()
    } else {
        env::current_dir()?.join(path)
    }.clean();

    Ok(absolute_path)
}

/*
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
///Some almost usefulness stuff 
fn goodbye() -> String {
    "さようなら".to_string()
}

#[allow(missing_docs)]
#[macro_export]
#[warn(unused_macros)]
macro_rules! pt {
    ($a: expr) => {
        pt($a, None, None)
    };
    ($a: expr, $($b: expr),+) =>{
        pt($a ,..., $b)
    };
}
*/

