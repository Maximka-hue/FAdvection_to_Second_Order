//Errors in dark red, cyan for info
pub use std::borrow::Cow;
use crate::initial_data_utils::function_utils::print_macros::macro_lrls::{pt};
extern crate os_type;
use os_type::{ OSType};
extern crate path_clean;
use handle::Handle;
use log::{info, warn};
use path_clean::PathClean;
use csv::Writer;
use std::{thread, cmp, str::FromStr, 
    io::{self, SeekFrom, Seek, ErrorKind, BufRead, BufReader, Write},
    path::{Path, PathBuf}, {time::{ Duration}}, fs::{self, OpenOptions, File}, env};
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
use std::error::Error as SError;//**** 
type StdResult<T> = std::result::Result<T, Box<dyn SError>>;
use glob;
//use dao_ansi::color::kinds::{ForegroundColor, BackgroundColor, PrimaryColor};


pub const  SLEEP_PASS: u16 = 0;
pub const  SLEEP_LOW: u16 = 100;
pub const  SLEEP_NORMAL: u16 = SLEEP_LOW * 2;
pub const  SLEEP_HIGH: u16 = SLEEP_NORMAL * 2;
pub const  SLEEP_MAX: u16 = SLEEP_HIGH * 2; 
pub const  ALL_TIMES: [u16; 5] = [SLEEP_PASS, SLEEP_LOW, SLEEP_NORMAL, SLEEP_HIGH, SLEEP_MAX];

pub const  ARGUMENTO_DBGOUT: bool = true;
pub const DEFAULT_ELEMENTS_PER_RAW: usize = 11;
pub const IS_CHOSEN_WRITE_IN_MAIN_CYCLE: bool = true;

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

// Foreground Color:???????????????????????????Black Red Green Magenta Cyan Reset
// Background Color:???????????? White
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

//?????????????????? ???????????????????? ?????????????????? ???????????? ref &
pub fn run<'a>(argumento: &'a Argumento)-> Result<(), Box<dyn SError>>
//dyn ???????????????????????????? ???????????? ?????????? 
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
    //?????????????? ???????? ?? ???????????????????????? ????????????
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
    let processed_params =  File::create(&parameter_file).unwrap_or_else(|error| {
    //Not thread safe! fs::OpenOptions::new().create(true).write(true)/*.mode(0o770)*/.open(&parameter_file).unwrap_or_else(|error| {
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
        thread::sleep(Duration::from_secs(1_u64));
    }
    //This determine that file amount greater than fnum.
    let bu = PathBuf::from("src\\fi_greater_fnum.txt");
    let next_pathbuf= if fnum < num_files {parameter_file} else {bu};
    let next_str = next_pathbuf.clone().into_iter().map(|h| /*{let first_psgn = "/".to_owned();
    first_psgn.push_str(&String::from(h.to_string_lossy())[..]);
    first_psgn.push_str("/")}*/
    format!("{}{}{}",  "/", &String::from(h.to_string_lossy())[..],  "/")).collect();
    Ok((fnum, next_pathbuf, next_str, processed_params))
}
    

pub fn write_at_end<W: Write + Seek>(writer: &mut W, amount_of_files: usize) -> io::Result<()> {
        writer.seek(SeekFrom::End(0))?;
        for ii in 0..(amount_of_files+1_usize) {
            writer.write("\n{ii}".as_bytes())?;
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

pub fn traverse_not_hidden_files(path_debug_info: bool, maximum_files_to_expect: usize, input_fpath: &PathBuf) -> Vec<PathBuf> {
    let mut all_txt: Vec<PathBuf> = Vec::new();
    let walker = WalkDir::new(&input_fpath).into_iter();
    for entry in walker.filter_entry(|e| !is_dir_hidden(e)) {
        all_txt.push(PathBuf::from(entry.unwrap().path().clone()));
        }
        //First is directory itself
        let all_txt = all_txt[1..maximum_files_to_expect+1usize].to_vec();
        if path_debug_info{ 
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
        println!("{}", "args[1]: {query}".bold().italic());
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
            let io_sgn = read_string("You can choose the separation sign in the processed file:"); //?????????? ?????????????? ???????? ???????????????????? ?? ???????????????????????? ??????????
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
            println!("{} {rinsed_data:?}", "Rinsed: ".bold().italic());
            let mut new_init_data = Vec::with_capacity(25);
            let mut rubbish = Vec::with_capacity(25);
            for x in rinsed_data{
            let mut y =  x.trim_matches(char::is_alphabetic)
                .replace(","," ").replace("\r"," ").replace("'","").replace(" ","");//.replace(" ",":");
            let lovely_sgn = '????';
            let _lh: usize = '????'.len_utf8();
            let mut b = [0; 4];
            lovely_sgn.encode_utf8(&mut b);
            if y.contains(char::is_numeric) {
            //let num: usize= "????".chars().count();
                if y.contains('????') {
                    let r = y.find('????');
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
            println!("{} {rubbish:#?}", "Rb_comments: ".bold().italic());
                    //println!("{}",new_init_data.len());
                   /*let y = x.retain(|c| c !=',').as_str();
                    init[0].push_str(y);*/
        Ok((new_init_data, io_sgn.0))
}
    fn read_string(comment:&str) -> (String, u8) {
        print!("{}", comment);
        io::stdout().flush().expect("flush");
        const ILEN: usize = 20;
        let mut string: String = String::with_capacity(ILEN);
        let iolen:u8 = io::stdin().read_line(&mut string).ok().expect("Error read line!") as u8;
        println!("You had written {} bytes", iolen);
            return (String::from(string.trim()), iolen);
    }
pub fn show_shape(all_steps: usize, print_npy: usize, numvec: &Vec<f64>, exactvec: &Vec<f64>, 
                calculation_path: &PathBuf, nf: usize, desc: &str, time_form: Option<&str>, deb_my: bool){
    //Will be less than (print_npy - 1) * step_by_step
        let step_by_step = (all_steps  as f64 /print_npy as f64).floor() as usize;
        let mut next_vec_index: usize;
        println!("X, U, U_exact");
        let end = print_npy * step_by_step - 1_usize;
            for k in 0..print_npy {
                next_vec_index = k * step_by_step; 
                if deb_my{
                    println!("{}, {:^.5}, {:^.5}", next_vec_index, numvec[next_vec_index], exactvec[next_vec_index]);
                }
            }
        info!("All_steps: {}, {:^.5}, {:^.5}", all_steps, numvec[end], exactvec[end]);
        let pic_path = calculation_path.join(format!(r"pic_shapes_file_num{}_{}.txt", nf, time_form.unwrap_or("")));
        let mut pic_file = create_safe_file(None, Some(&pic_path), false).expect("cannot open file");
        for k in 0..print_npy {
            next_vec_index = k * step_by_step;
            pic_file.write_all(format!("{} , {:.5}, {:.5}\n",
            next_vec_index, numvec[next_vec_index], exactvec[next_vec_index]).as_bytes()).unwrap();
        }
        pic_file.write_all(format!("End: {} , {:.5}, {:.5}\n", end, numvec[end], exactvec[end]).as_bytes()).unwrap();
        pic_file.write_all(format!("^^^{}\n", desc).as_bytes()).unwrap();
}
pub fn create_safe_file(ppath_str: Option<&str>, path_buf: Option<&PathBuf>, only_create_once: bool) -> Result<File, std::io::Error>{
        let path_to_open = if let Some(path_str) = ppath_str{
            path_str.to_string()
        }
        else{
            String::from(path_buf.unwrap().to_string_lossy())
        };
        if only_create_once{
        Ok(std::fs::File::open(&path_to_open).unwrap_or_else(|error| {
            if error.kind() == ErrorKind::NotFound {
                File::create(&path_to_open).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            }
            else {
                panic!("Problem opening the file: {:?}", error);
            } 
        }))
    }
    else{
        Ok(std::fs::File::create(&path_to_open).unwrap_or_else(|error| {
            panic!("Problem creating the file: {:?}", error);
        }))
    } 
}
pub fn create_safe_file_with_options(path: PathBuf, create: bool) -> Result<std::fs::File, std::io::Error>{
    let file = if create {
        OpenOptions::new().create(true).write(true).open(&path).unwrap_or_else(|error| {//File::with_options()
        if error.kind() == ErrorKind::NotFound {
            File::create(&path).unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } 
        else {
            panic!("Problem opening the file: {:?}", error);
        }
    })
}
    else{
        OpenOptions::new().write(true).open(&path).unwrap_or_else(|error| {//File::with_options()
            if error.kind() == ErrorKind::NotFound {
                File::create(&path).unwrap_or_else(|error| {
                    panic!("Problem creating the file: {:?}", error);
                })
            } 
            else {
                panic!("Problem opening the file: {:?}", error);
            }
        })
    };
    Ok(file)
}
fn walk(dir: &Path, cb: &dyn Fn(&PathBuf), recurse: bool) ->
io::Result<()> {
    for entry in dir.read_dir()? {
        let entry = entry?;
        let path = entry.path();
        if recurse && path.is_dir() {
            walk(&path, cb, true)?;
        }
        cb(&path);
    }
    Ok(())
}
type GenericError = Box<dyn SError + Send + Sync + 'static>;
fn walk_glob(pattern: &str, cb: &dyn Fn(&PathBuf)) -> Result<(),
    GenericError> {
        for entry in glob::glob(pattern)? {
            cb(&entry?);
        }
        Ok(())
    }
pub fn save_files(dir: &PathBuf, tvector: Vec<f64>, wvector: Option<Vec<f64>>, (steps, left, right): (usize, Option<f64>, Option<f64>), 
    elements_per_raw: Option<usize>, nf: usize, output_periods: usize, necessity_of_csv: Option<bool>, paraview_format: Option<bool>, my_deb: Option<bool>) 
    -> std::io::Result<()>{
        let my_deb = if let Some(debug) = my_deb {
            debug
        }
        else{
            false
        };
//Define variables +++++++++++++++++++++++++++++++++++++++++++++++
        let repeated_dbg: String= std::iter::repeat(".").take(20).collect();
        let raw_size: usize= if let Some(elements_per_raw) = elements_per_raw{
            elements_per_raw
        }
        else{
            DEFAULT_ELEMENTS_PER_RAW
        };
        let mut string_raw: String;
        let left = left.unwrap_or(0.0);
        let right = right.unwrap_or(0.0);
        let distance = right - left;
        let h_fl: f64 = steps as f64/raw_size as f64; 
        let h = (steps/raw_size) as usize;
        println!("h_fl - h: {} ^ {} = {}",h_fl, h, h_fl - h as f64);
        let mut next_index: usize;
        let mut x_next: f64;
        let mut on_line: usize;
        println!("directory specified {:?} paraview_format: {:?}" , dir, paraview_format);
        let pypath = dir.join(format!("to_python_{}.txt", nf));//.join(format!(r"\{}", dir.display()))
        let expypath = dir.join(format!("exact_to_python_{}.txt", nf));
        let newly_treated = dir.join(format!("treated_datas_{}", nf));
        let parameters_path = dir.join(format!("parameters_{}.txt", nf));
        println!("{:?}  ^ {:?} ^ \n{:?}", format!("PyPaths for graphics: \n{:?}", pypath), expypath, parameters_path);
        let mut exact_vector: Vec<f64> = Vec::with_capacity(tvector.len()+1);
        let py_path = create_safe_file_with_options(pypath, true)?;
        if let Some(ex) = wvector{
            create_safe_file_with_options(expypath, true)?;
            exact_vector= ex;
        }

//This will create csv like txt files to turn them in paraview window ------------------------------------
        if paraview_format.unwrap_or(false){
            let switch_path_paraview = dir.join("paraview_datas");
            println!("quantity parts size: {}\n paraview path: {:?}\n Is it directory? {}", raw_size, switch_path_paraview, switch_path_paraview.is_dir());
            let end_of_traverse_exact = tvector.len() as f64 / output_periods  as f64;
            let end_of_traverse = (tvector.len() as f64 / output_periods  as f64).floor() as usize;
            println!("What will be print step? - {}", end_of_traverse_exact - end_of_traverse as f64);
            let new_paraview_dir_to_create = newly_treated.join("paraview_datas");
            let new_paraview_dir = newly_treated.join("paraview_datas");
            fs::create_dir_all(new_paraview_dir_to_create)?;
//How it write? 
            for y_index in 0.. end_of_traverse {
                //Check that vector doesn't contain all zeros
                //println!("{:?} \n {:?}\n^", tvector, exact_vector);
                if my_deb {
                    println!("y_index {}, Condition on write, Any of elements!=0: {}",
                    y_index, tvector[y_index..y_index+(raw_size-1) as usize].iter().any(|&v| !approx_equal(v, 0.0, 3)));
                }
                if tvector[y_index..y_index+(raw_size-1) as usize].iter().any(|&v| !approx_equal(v, 0.0, 3)){
                    let updating_x_u_w = new_paraview_dir.join(format!("x_u_w_{0}_{1}.txt", nf, y_index));
                    let path = Path::new(&new_paraview_dir);
                    if my_deb{
                        println!("switch_path_x_u_w : {:?}", updating_x_u_w);
                    }
                    println!("Listing '{}'", path.display());
                    let mut paraview_file_x_u_w = create_safe_file(None, Some(&updating_x_u_w), false)?;//superfluously
                    println!("{:?}" , paraview_file_x_u_w);
                    paraview_file_x_u_w.write_all("x, exv, numv\n".as_bytes())?;
                    for k in 0..raw_size {
                        on_line = h*k;
                        x_next = left + on_line as f64;
                        next_index = k + y_index * raw_size;
                        string_raw = format!(r"{:.6}, {:.6}, {:.6}{}",
                            x_next, exact_vector[next_index], tvector[next_index], "\n");
                        paraview_file_x_u_w.write_all(&string_raw[..].as_bytes())?;
                    }
                    if y_index != end_of_traverse-1{
                        string_raw = format!(r"{:.6}, {:.6}, {:.6}{}", 
                            steps , exact_vector[raw_size + y_index * raw_size], tvector[raw_size + y_index * raw_size], "\n");
                        paraview_file_x_u_w.write_all(&string_raw[..].as_bytes())?;
                    }
                    else{
                        string_raw = format!(r"{:.6}, {:.6}, {:.6}{}", steps , exact_vector[exact_vector.len() -1], tvector[tvector.len() -1], "\n");
                        paraview_file_x_u_w.write_all(&string_raw[..].as_bytes())?;
                    }
                }
            }
        }
//Else I can pass it into csv format
    let necessity_of_csv = necessity_of_csv.unwrap_or(false);//shaded variable
    let mut new_switch_path: PathBuf;
    if necessity_of_csv == true {
        let ub = format!(r"csv_{0}", nf);
        new_switch_path = dir.join(ub);
        let csv_data_dir = Path::new(&new_switch_path);
        let err = fs::create_dir_all(csv_data_dir);
        if let Some(err) = err.err(){
            warn!("{}", err);
        }
    }
    let mut csv_array = vec![vec![0.0; 2_usize * raw_size];
        cmp::max(tvector.len(), exact_vector.len())];
    let mut x_index: usize;
        //let mut wtr_inner;
        //let mut temp_csv;
        Ok(())
}

pub fn add_additional_info_in_datas_end(dir: &PathBuf, nf: usize, t_max: Option<f64>,  elements_per_raw: Option<usize>)-> std::io::Result<()>{
    let raw_size: usize= if let Some(elements_per_raw) = elements_per_raw{
        elements_per_raw
    }
    else{
        DEFAULT_ELEMENTS_PER_RAW
    };
    //First check if I had alredy written info 
    let param_treated = dir.join(format!("treated_datas_{0}", nf));
    let param_ex_to_read = param_treated.join( format!("parameters_nf{0}.txt", nf));
    printc!(red: "File with parameters - {:?},\n",  param_ex_to_read.display());
    printc!(yellow: "which exists? {:?}\n",  param_ex_to_read.exists());
    let path_to_read = Path::new(&param_ex_to_read);
    let mut is_written_already: bool = false;
    //This won't create file, so func create_safe_file can be applied
    {
        let prm_file= create_safe_file(None, Some(&param_ex_to_read), true)?;  
    let mut reader_parameters = io::BufReader::new(&prm_file);
    let mut pbuf = String::new();
    'cycle: while let nbytes = reader_parameters.read_line(&mut pbuf).ok().expect("ERRMSG")//.expect("reading from cursor won't fail")
    {
        println!("\tBytes readed :{}", nbytes);
        if nbytes !=0{
            if pbuf.to_lowercase().contains("printed elements per raw"){
            println!("Found in file {:#?}", pbuf);
            is_written_already = true;
            }
            else{
                continue
            }
        }
        else{
        break 'cycle
        }
    }
    pbuf.clear();
}
    let mut prm_file_write= create_safe_file(None, Some(&param_ex_to_read), false)?; 
    //Write additional info about reducing steps in graphics and spec for burger max_time
    let new_position_par = prm_file_write.seek(SeekFrom::End(0)).unwrap(); 
    if !is_written_already{
        prm_file_write.write_all(format!("\nprinted elements per raw {}\n", raw_size).as_bytes())?;
    }
    if let Some(t_max) = t_max {
        prm_file_write.write_all(format!("{} Maximum live time in burger task: {}\n","\t", t_max).as_bytes())?;
    }
    Ok(())
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
///These functions search delimeters [first from book Jim Blandy and latter my improved version]
//???????? ?????????????????? ????????????????????????
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
            match s[index+width..].find(separator){//1???? ??????????
           /* None => match (T::from_str(&s[..index]), T::from_str(&s[index+1..])){
            (Ok(_l),Ok(_r)) => None,  //Some((l, r,None)),
            _ => None*/
            None => None,
            Some(indexx) =>{//???????????? ??????????
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
    "???????????????".to_string()
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
