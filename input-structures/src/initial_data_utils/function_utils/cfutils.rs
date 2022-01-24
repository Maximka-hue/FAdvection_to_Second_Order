//Errors in dark red, cyan for info
#![feature(trace_macros)]
#[macro_use]
pub use std::borrow::Cow;
#[macro_use]
use crate::initial_data_utils::function_utils::print_macros::macro_lrls::{pt};

//use print_macros::*;
use std::borrow::Borrow;
use std::env;
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
use std::fmt::Debug;
use std::fmt::Formatter;
use ansi_term::Colour::*;
use tutil::crayon::{Style as CrStyle, Color::{Red, Blue}};
use text_colorizer::*;
use termion;

//use dao_ansi::color::kinds::{ForegroundColor, BackgroundColor, PrimaryColor};
use better_term::{flush_styles, rainbowify};
/* Regex */
use lazy_static::lazy_static;
use regex::Regex;
use regex::{RegexSet};

pub const  SLEEP_PASS: u16 = 0;
pub const  SLEEP_LOW: u16 = 100;
pub const  SLEEP_NORMAL: u16 = SLEEP_LOW * 2;
pub const  SLEEP_HIGH: u16 = SLEEP_NORMAL * 2;
pub const  SLEEP_MAX: u16 = SLEEP_HIGH * 2; 
pub const  ALL_TIMES: [u16; 5] = [SLEEP_PASS, SLEEP_LOW, SLEEP_NORMAL, SLEEP_HIGH, SLEEP_MAX];

pub const  ARGUMENTO_DBGOUT: bool = true;

// Note the use of braces rather than parentheses.
custom_error!{pub ArgumentParseError
    AmountOfFiles{error_description: String, code: u8} = "{error_description}\n   error code {code}.",
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


#[derive(Debug, Clone)]
pub struct Argumento{
    pub query : String,
    pub filenames : Vec<String>,
    pub case_sensitive: bool,
}

impl Argumento {
    pub fn new(args: &[String]) -> Result<Argumento, ArgumentParseError>  {
        //trace_macros!(true);
        if args.len() < 3 {
            return Err(ArgumentParseError::AmountOfFiles{
                error_description: format!(
                    "{}parsing args: not enough arguments:
                    \nThis program expect name main.rs + other txts 
                    \n\r containing info of initial values", termion::color::Bg(termion::color::Red)),
                code: 1})
                }
    let mut args_vec: Vec<String> = Vec::with_capacity(args[2..].len() as usize); 
        for argument in env::args().skip(2) { //skip name of programm 
            if argument == "--help" {
                cyan!("You passed --help as one of the arguments!");
            }
            else if argument.ends_with(".txt"){
                args_vec.push(argument);
                //pt("argument", None);
                pt!("Arguments as supposed determining files.txt", "impl", &args_vec, PrintStyle::Debug);// print name of file first time...
            }
/*Very important!*/else if argument.starts_with("--") | argument.starts_with("-"){
                continue
            }
            else{
                pt!("Now support text files only", "debug");
            }
        }
        cyan!("Vector of passed arguents");
        if ARGUMENTO_DBGOUT{
            pt!("Debug check for txt files", "dbg", &args_vec, PrintStyle::Debug);// below(pt/ptc is only convenient print)
        }
        let query = args[1].clone();
        println!("args[1]: ");
        pt!(&query, "impl");
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

