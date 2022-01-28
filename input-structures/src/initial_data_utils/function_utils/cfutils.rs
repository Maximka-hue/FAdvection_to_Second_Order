//Errors in dark red, cyan for info
#![feature(trace_macros)]
#[macro_use]
pub use std::borrow::Cow;
use crate::initial_data_utils::function_utils::print_macros::macro_lrls::{pt};
extern crate os_type;

//use print_macros::*;
use std::borrow::Borrow;
use std::{io::{self, Write},{time::Duration}, fs, env};
use named_tuple::named_tuple;
use std::path::{Path, PathBuf};
extern crate custom_error;
use custom_error::custom_error;
#[path="./custom_colours.rs"]
pub mod custom_colours;
pub use custom_colours::*;
/* Stylisation */
pub use colour::*;
pub use tcprint::*;
pub use colorify::*;
pub use simple_colors::{white, red, printlnc, Style as SimStyle,Color as SimColor};
use std::fmt::{Debug, Formatter};
use ansi_term::Colour::*;
use tutil::crayon::{Style as CrStyle, Color::{Red, Blue}};
use text_colorizer::*;
use termion;


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

pub fn op_sys(){
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


    #[derive(Debug, Clone)]
    pub struct Argumento{
        pub query : String,
        pub filenames : Vec<String>,
        pub case_sensitive: bool,
    }
    
    impl Argumento {
        pub fn new(args: &[String]) -> Result<Argumento, ArgumentParseFilesError>  {
            //trace_macros!(true);
            if args.len() < 3 {
                return Err(ArgumentParseFilesError::AmountOfFiles{
                    error_description: format!(
                        "{}parsing args: not enough arguments:
                        \nThis program expect name main.rs + other txts 
                        \n\r containing info of initial values", termion::color::Bg(termion::color::Red)),
                    code: 1})
                    }
        let mut args_vec: Vec<String> = Vec::with_capacity(args[2..].len() as usize); 
        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();
        for argument in env::args().skip(2) { //skip name of programm 
            if argument == "--help" {
                cyan!("You passed --help as one of the arguments!");
            }
            else if argument.ends_with(".txt"){
                args_vec.push(argument);
                    if ARGUMENTO_DBGOUT{// print name of file first time...
                    pt!("Arguments as supposed determining files.txt", "impl", &args_vec, PrintStyle::Debug);}
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
                pt!("Debug check for txt files", "dbg", &args_vec, PrintStyle::Debug);// below(pt/ptc is only convenient print)
            }
            let query = args[1].clone();
            println!("args[1]: {}", case_sensitive);
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

    fn preprocess_text(file: &String)-> Result<(Vec<std::string::String>, String), ()>{
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
use std::str::FromStr;
///These functions search delimeters [first from book Jim Blandy and latter my improved version]
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
    }
}
///Some almost usefulness stuff 
fn goodbye() -> String {
    "さようなら".to_string()
}
/*
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
