//#[crate_type = "staticlib"]
///Deduce types of variables, store arguments from parsing input
///Create file structures from which programm will read data
/// #[macro_use]
use crate::initial_data_utils::function_utils::print_macros::{rainbowify, flush_styles, macro_lrls::{pt, mypt}};
/* Building struct */
pub use derive_builder::Builder;
pub use std::borrow::Cow;
pub use structopt::StructOpt;
use std::default::Default;
use std::fs::{self, File, write, OpenOptions};
use std::path::{self, PathBuf, Path};
use std::time::{self, Instant, Duration};

use tcprint::{tcprintln , BasicColors, ColorPrintState, Color, ColorSpec};
use std::thread;
///This enumerattion determine type of task: Burger or Advection.
///Specificity of the **first** entered argument (type of equation)
//Специфика введенного первого аргумента (типа уравнения)
#[derive(Debug, Clone, PartialEq)]
pub enum TaskType{
    Burger(TaskTypeCs),//It's only to choose type of equation 
    Transfer{a: f64},  //,u0_1:f32,u0_2:f32,u0_3:Option<f32>},
}
impl Default for TaskType {
    fn default() -> Self { TaskType::Transfer{a: 1_f64} }
}
///This is for specifying some context in which TaskType will be used.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskTypeCs{
    pub burger_cs: ColorSpec,
    pub burger_cs_second_order: ColorSpec,
    pub burger_cs_world_time: ColorSpec,
    pub burger_cs_step_time: ColorSpec,
}
///Implementation uses crate __tcprint__.
impl Default for TaskTypeCs {
    fn default() -> Self {
        // By default, pet names are printed in bold blue.
        let mut burger_cs = ColorSpec::new();
        let mut burger_cs_second_order = ColorSpec::new();
        let mut burger_cs_world_time = ColorSpec::new();
        let mut burger_cs_step_time = ColorSpec::new();
        burger_cs.set_fg(Some(Color::Blue)).set_bold(true);
        burger_cs_second_order.set_fg(Some(Color::Green)).set_bold(true);
        burger_cs_world_time.set_fg(Some(Color::Cyan)).set_underline(true);
        burger_cs_step_time.set_fg(Some(Color::Yellow)).set_underline(true);
        TaskTypeCs{burger_cs: burger_cs,
            burger_cs_second_order: burger_cs_second_order,
            burger_cs_world_time, burger_cs_step_time}
    }
}

//____________________________Input file data_____________________________________________________
//This struct creates new file_process manually
#[derive(Debug, Clone, Builder)]
#[builder(build_fn(validate = "Self::validate_parameters"))]
pub struct FileParametres{
    #[builder(public)]
    pub eq_type: i8,
    #[builder(default = "(0 as f64, 1 as f64)")]
    pub margin_domain:(f64,f64),
    pub time_eval_period_stage: (f64,f64),
    pub bound_type: i8,
    pub init_type: i8,
    pub init_conditions: (f32,f32, Option<f32>, Option<f32>),
    pub quantity_split_nodes: u32,
    #[builder(setter(into))]
    pub n_corant:f32,
//#[builder(setter(into, strip_option), default)]- don't work
    pub add_args: (Option<TaskType>, Option<i8>, Option<bool>)//will be last background_mc additional_correction
    //pub add_args: Vec<Option<TypeTsk>, Option<i8>, Option<bool>> I want like this, but don't know way
}
///Boundary, time and equation check in parameters
impl FileParametresBuilder {
    fn validate_parameters(&self) -> std::result::Result<(), String>{//io::ErrorKind
        if let Some(ref eq_type) = self.eq_type {
            match *eq_type {
                i if i < 0 => {pt!("First less than 0, no such type equation", "Impl"); red!("Nothing right in equation!");panic!("Invalid number: {}", i)},//ErrorKind::InvalidData
                i if i > 1 => {pt!("First more than one, no such type equation", "Impl"); red!("Nothing right in equation!");panic!("Invalid number: {}", i)},
                _ => Ok(())
            }
        }/* 
        else if self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).0 < self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).1 {
            println!("Incorrect time specification: {}", self.time_eval_period_stage.unwrap().0);
            pt!("Please correct programm time boundary", None);
            println!("Nothing right in time!");panic!("Invalid time: must be {:.3}>{:.3}", self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).0,
            self.time_eval_period_stage.unwrap_or((0_f32,0_f32)).1)}
        
        else if (self.margin_domain.unwrap_or((0_f32, 0_f32)).0 - self.margin_domain.unwrap_or((0_f32,0_f32)).1).abs()== std::f32::MIN 
        {
            println!("Incorrect Domain input");
            panic!("Domain is 0!");
        }//Check not to divide further by 0 in Transfer task
        else if self.eq_type.unwrap_or(0_i8) == 0
            {if let Some(velocity_) = self.add_args.clone().unwrap().0
                {// if there is smth in additional arguments...
                if velocity_ == (TaskType::Transfer{a: 0_f32}) {
                println!("Transfer build must be not 0!");
                pt!("Please correct transfer parameter or change type equation", None);
                println!("Nothing right in time!");panic!("{:?}",TaskType::Transfer{a:0_f32})}
                else{println!("Input transfer velocity is {:?}", velocity_);
                    return Ok(())
                    }
                }Ok(())}*/
        else {
            thread::sleep(time::Duration::from_millis(500_u64));
            Ok(())
            }
    }
}
///
pub fn initial_information_of_advection() -> Instant {
    //let ttti = TaskType::default();//TaskType::Burger(ColorSpec::new()).default();
    ///This will print initial information about the advection programm and me.
    let now = Instant::now();
    const PROGRAM_NAME:&'static str = "Equations of transfering and Burgens:1st order";
    let mut session_number = String::from("Session number \t");
    session_number.push_str("9\n");session_number.push_str(PROGRAM_NAME);
    mypt!(session_number.as_str(), "dbg");
    let mut my_name = String::with_capacity(15);
    my_name.push_str("Maxim_Mochalov");
    println!("\nAutor:\t{}",my_name);
    let fullname = "\tMaxim Mochalov Sergeevich";
    //println!("length is {}",fullname.len());
    let mut i = 1;
    for name in fullname.split_whitespace(){
        match i {//dark_yellow
        1 => println!("{i}\t{Name} {name}", Name = rainbowify("name:")),
        2 => println!("{i}\t{Surname} {name}", Surname = rainbowify("surname:")),
        3 => println!("{i}\t{Patronymic} {name}", Patronymic = rainbowify("patronymic:")),
        _ => println!("Nothing suitable")}
        i+=1;
        flush_styles();
}
now
}
/*
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
    #[structopt(name = "AmountOfFiles", short = "af", long ="amount_of_files", default_value = "1",
        help = "Pass `-h`: These will process exact amount of initial data files")]
    pub amount_of_files: i32,
}
//___________________________________________________________________________________________________

*/
