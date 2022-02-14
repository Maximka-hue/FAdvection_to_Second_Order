//As this training task for first steps learning language, 
//DESIGNATIONS will be following:📔
//E!xt- Doesn't import crate(extension)
//C!ircumvent- desire to do smth advanced to avoid ...e.g. creating temp value, etc.(et cetera)
//W!ork - Doesn't work
//D!esire - I would like to use it, but didn't find appropriate method/way to use for it) 
//*********************************************************************
//My libraries 
//extern crate time_measure;
//extern crate input_structure;
#![feature(path_try_exists)]
#[macro_use]
extern crate time; 
use input_structure::initial_data_utils::{Path,PathBuf, function_utils::print_macros::macro_lrls::{pt}};
use input_structure::initial_data_utils::{parse_into_file_parameters};
#[warn(unused_imports)]
use input_structure::cfutils::{ChooseSleepTime, ColorPrintState, ArgumentParseFilesError, create_safe_file_with_options, 
    op_sys, approx_equal, write_at_end, traverse_not_hidden_files, show_shape, save_files, add_additional_info_in_datas_end,
    IS_CHOSEN_WRITE_IN_MAIN_CYCLE};
use input_structure::{TaskType, TaskTypeCs, FileParametres, initial_information_of_advection, 
    advection_input, process_files, main_initialization, do_exact_solutions, SIMPLE_STEP_TYPE,
    main_cycle_first_order, main_cycle_with_correction, calculate_output_time_vec_based_on_outtime_rate};
#[macro_use]
extern crate colour;
#[macro_use] 
extern crate tcprint;
extern crate ansi_term;
extern crate num_cpus;
extern crate rand;
use env_logger;
extern crate log;
use log::{debug, error, info, warn};
extern crate walkdir;
use gtk::prelude::*;
use rayon::{prelude::*};
use gtk::{Application, ApplicationWindow, Box as GTKBox, Button, Label};
pub use ansi_term::{Colour::{Fixed, Black as AnsiBlack, Red as AnsiRed, Green as AnsiGreen, Yellow as AnsiYellow, Blue as AnsiBlue, Purple as AnsiPurple, 
    Cyan as AnsiCyan, Fixed as AnsiFixed}, Style as AnsiStyle};
///These imports from library as I already downloaded these crates)
use std::{cmp::Ordering,time::Instant, time::Duration as SDuration, thread, env, fs::{self, OpenOptions}, io::{ Write}};
use chrono::{Duration as CDuration};
use time::macros::date;
use itertools::Itertools;
use rand::{/*distributions::{Distribution, Uniform}, prelude::*, task_rng,*/ Rng, prelude::*};
//Determine in cycle all provided for constants arguments, otherwise default.
mod determine_my_impls{
    pub const INITIAL_INFO_ABOUT_PROGRAMM: bool = false;
    pub const MY_ARGUMENT_PARSING: bool = false;
}
mod determine_calculation_modes{
pub const PATH_DEBUG_INFO: bool = true;
pub const DO_STEP_REDUCE: bool = true;
pub const PATH_CREATION: bool = true;
pub const LETS_DO_PAUSE: bool = true;
pub const GENERATE_RANDOM_EXAMPLE: bool = false;
pub const RANDOM_PATH_CREATION: bool = false;
pub const GET_FILES_FROM_DIRECTORY: bool = false;
pub const CHECK_ENDS_OF_DOMAIN: bool = true;
}
mod modifications{
pub const RANDOM_TRANSLATE_MARGINE_BOUNDARY: bool = true;
pub const TIME_OUTPUT: bool = true;
pub const MY_TEX_PATH_FILE: bool = true;
pub const ADDITION_OF_TIME_VECTORS: bool = true;
pub const MAXIMUM_FILES_TO_EXPECT: usize = 6;
pub const DIVIDE_ALL_STEPS_TO_PYTHON_PIC: usize = 11;
pub const LANGUAGE_TO_USE_CORRECTION: bool = true;//this is to switch among smooth.c and smooth.rs programs
pub const REDUCE_TIME_TO_INT: bool = true;
pub const SHOULD_SLEEP_IN_MAIN: bool = false;
}

//Then will be blocks that only for me to understand rust!(Maybe you will do another initializations,....)
//Further mark it like ))) and my own implementations of crates like *******....
//))) blue for paths and debug, 
pub use determine_my_impls::*;
pub use determine_calculation_modes::*;
use modifications::*;

use std::error::Error as SError;
//type StdResult<T> = std::result::Result<T, Box<dyn SError>>; D!esire to return Result #TODO
fn main() {//-----------------------------------------
    let application = Application::new(
        Some("com.github.rust-ui-rundown.rust-ui-gtk"),
        Default::default(),
    );//.expect("failed to initialize GTK application");

    application.connect_activate(|app| {
        let window = ApplicationWindow::new(app);
        window.set_title("This is advection programm!");
        window.set_default_size(700, 200);

        let container = GTKBox::new(gtk::Orientation::Vertical, 10);
        let label = Label::new(None);
        let button = Button::with_label("Click me!");
        container.add(&label);
        container.add(&button);
        window.add(&container);

        button.connect_clicked(move |_| {
            let _ = &label.set_label("Hello, World!");
        });
        initial_information_of_advection();
        window.show_all();
    });
    application.run();//-----------------------------------------
    let app_get = date!(2021 - 01 - 31);
    magenta!("App was done at {app_get:?}");
    let began_advection = SDuration::ZERO;
    let std_duration = SDuration::from_millis(0 as u64);
    let advection_start = std::time::Instant::now();
    let from_cli = if MY_ARGUMENT_PARSING{
        //process it by myself
        advection_input()
    }
    else{
        //with clap
        //cargo run -- -output-style(maybe cli-args , etc) --file-paths input-pstructures/src/advec_examples/TransferBurgerMccornack_iconditions00.txt
        advection_input()
        //home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/input-pstructures/src/advec_examples/TransferBurgerMccornack_iconditions0.txt
    //55.seconds() 
    };
    let duration = advection_start.elapsed();
    let new_now  = std::time::Instant::now();
    std_duration.saturating_add(duration);
    if TIME_OUTPUT{
        println!("App initialization: {:?} {duration:?}", new_now.duration_since(advection_start));
    }
    let (argumento, my_config) = from_cli.unwrap();
//-----------------------------------------
//Here I am defining by default colored struct with task type
    let mut burger_rng = rand::thread_rng();
    let burger_order: u16 = burger_rng.gen_range(0..3);
    let mut state = ColorPrintState::<TaskTypeCs>::default();
    tcprintln!(state,
        (""),
        [burger_cs_first_order: "{}", "OOO"],
        ("!"),("!")
    );
//General instruction about operating sys and environment variables
    let _comp_os = op_sys();
    let num_threads = num_cpus::get();
//From this point I will determine file hierarchy on which output and input files will be.
//----------------------------------------- 
//First I need to determine places where to save input images, datas, etc.
    let env_path = env::current_dir().unwrap();
    let advection_path = env_path.join("src");
    let animation_path = advection_path.join("animation");
    let calculation_path = animation_path.join("datas");
    let photos_path = animation_path.join("photos");
    let output_path = advection_path.join("OutputFiles");
    let log_output_path = output_path.join("logging/advec_log.txt");
    let input_fpath = env_path.parent().expect("Determine input file's path").join("input-pstructures").join("src").join("advec_examples/");
    let directory_with_examples_exists = input_fpath.try_exists().expect("Can't check existence of file does_not_exist.txt");
    let is_example_dir: bool = if directory_with_examples_exists { input_fpath.is_dir()} else{ false};
    if PATH_DEBUG_INFO {
        assert_eq!(&env_path.as_path().display().to_string(), "/home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/advection");
        println!("{prog}  {env_path:#?}\n{anim}  {advection_path:#?}\n\
        {calc}  {calculation_path:#?}\n{photo}  {photos_path:#?}\n\
        {input_txt}  {input_fpath:#?}\n\
        ",
        prog=  AnsiGreen.blink().paint("Programm start location:"), photo=  AnsiGreen.bold().paint("Photos path:"),
        anim= AnsiGreen.bold().paint("Animation storage"), calc=  AnsiGreen.bold().paint("Calculation data:"), 
        input_txt= AnsiGreen.bold().paint(""));
        if !directory_with_examples_exists{
        println!("Number of threads on your Computador: {num_threads}\n\
        You entered path {input_fpath:?}\n that {} to exist",
        directory_with_examples_exists);
        }
        else if directory_with_examples_exists && !is_example_dir{
            println!("You entered path {input_fpath:?}\n that exists but {} to be directory\n\tPlease enter another path", is_example_dir);
        } 
    }
    println!("burger_order: {}", burger_order);
    if PATH_CREATION {
        fs::create_dir_all(&animation_path).unwrap(); 
        fs::create_dir_all(&calculation_path).unwrap(); 
        fs::create_dir_all(&photos_path).unwrap(); 
        if !directory_with_examples_exists {
        fs::create_dir_all(&input_fpath).unwrap();
        println!("{}", Fixed(221).on(Fixed(124)).paint(format!("{input_fpath:?}Is it now directory? {}" , fs::canonicalize(&input_fpath).unwrap_or(PathBuf::new()).is_dir())));
        }
        if output_path.try_exists().expect("Can't check existence of OuputFiles directory[tex and log]"){
            fs::create_dir_all(&log_output_path).unwrap();
            println!("{}", Fixed(221).on(Fixed(124)).paint(format!("file for Logging created at {:?}", &log_output_path)));
        }
        else {
            fs::create_dir_all(&output_path).unwrap();
        }
        let example_data_path = &calculation_path.join("example_datas");
        //Here will be stored calculations for every file example
        std::fs::create_dir_all(&example_data_path).unwrap();
    }
    if RANDOM_PATH_CREATION {
        if directory_with_examples_exists{
            pt!("\nDirectory for examples already exists\n");
            fs::create_dir_all(&input_fpath.join("random_examples")).unwrap();
        }
        else{
            fs::create_dir_all(&input_fpath).unwrap();
        }
    }
//Then I am initializing structure that would be passed as initial datas for program, but!
    let data_default = FileParametres::first_initializing(1).expect("Something wrong in Initializing");//It is as default for program
//There are options: 1 generate from file[GENERATE_RANDOM_EXAMPLE= false or datas from file will be illigal]
//(in that case supported Transfer task)
//2 from txt files which *will be from input path getted *collected from command line *from file[their paths].
// **Command line can be processed by hand-made parser into struct Argumento or with clap
    let mut file_paths_with_examples = my_config.get_files();
    let is_there_some_files = file_paths_with_examples.len()!= 0_usize;
    let advection_modes = my_config.get_advection_modes();
    println!("{:?} - {advection_modes:?}", file_paths_with_examples);
    let mut dir_to_search: PathBuf = my_config.get_directory_with_files();
    let dir_not_empty = dir_to_search.read_dir().map(|mut i| i.next().is_none()).unwrap_or(false);
    dir_to_search = if dir_not_empty && is_there_some_files {
        fs::canonicalize(dir_to_search).unwrap()
    }
    else{
        input_fpath.clone()
    };
    if GENERATE_RANDOM_EXAMPLE {    
        parse_into_file_parameters(RANDOM_TRANSLATE_MARGINE_BOUNDARY);
        //Ok((String::new()))
    }
    else if !advection_modes.2{//out_style
        //Get txt with datas
        if GET_FILES_FROM_DIRECTORY{
            let txt_files = traverse_not_hidden_files(PATH_DEBUG_INFO, MAXIMUM_FILES_TO_EXPECT, &dir_to_search);
            let tex_file_path = advection_path.join("OutputFiles").join("RUSTadvection.tex");
            let mut tex_file = OpenOptions::new()
                .write(true).open(tex_file_path).expect("Writing to tex");
            if MY_TEX_PATH_FILE{
                let is_success_tex = write_at_end(&mut tex_file, my_config.get_files_len());
            }
        }
        else{
            //Get from input path
            let path_at_which_to_search_examples = my_config.get_directory_with_files();
            let txt_files = traverse_not_hidden_files(PATH_DEBUG_INFO, MAXIMUM_FILES_TO_EXPECT, &path_at_which_to_search_examples);
            }
            //Ok((String::new()))
        }
//Now let's read datas and overwrite them more clearly
let mut file_parameters_from_cli = (Vec::<FileParametres>::new(), Vec::<String>::new());
let calculation_path_as_string = calculation_path.into_os_string().into_string().unwrap();
let deb_my = advection_modes.2;
let correction = advection_modes.1;
if deb_my {
    file_parameters_from_cli = process_files(&mut file_paths_with_examples, Some(advection_modes.3), 
        Some(advection_modes.0), Some(LETS_DO_PAUSE), Some(calculation_path_as_string)).unwrap();
}
let deb_my = false;
let calculation_path_as_str = &animation_path.join("datas").into_os_string().into_string().unwrap()[..];
let number_of_files_with_data = file_parameters_from_cli.1.len();
let debug_add = true;
let all_exact_record = true;  
let exp_path = Path::new("/home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/advection/src/animation/exper");
let _exp_buf = PathBuf::from(exp_path);
if all_exact_record{
    fs::create_dir_all(exp_path).unwrap();
}
std::env::set_var("RUST_LOG", "advection=debug");
env_logger::init();
//Begin main calculations +++++++++++++++++++++++++++++++++++++++++++++++++++++++++====
(file_parameters_from_cli.0, file_parameters_from_cli.1).into_par_iter().zip((0..number_of_files_with_data).into_iter()).for_each(|(data, fi)| {
    
    let mut my_time_counter = ChooseSleepTime::add_default_time();
    let concrete_digits_data = data.0;
    let paths_to_processed_datas = data.1;
    let calculation_data_path = PathBuf::from(calculation_path_as_str);
    println!("{:?} \n {:?}", concrete_digits_data, Path::new(&paths_to_processed_datas));
//-------------------------------------------------
/*&String: file_ith_argument*/let fiarg = &concrete_digits_data; //This ith file from command line!
/*type*/        let equation = fiarg.eq_type;
/*nodes*/       let steps = fiarg.quantity_split_nodes as usize;
/*domain*/      let domain = fiarg.margin_domain;
                let domain_ends_difference = (domain.1 - domain.0).abs();
/*step*/        let dx = domain_ends_difference/steps as f64;
/*Courant*/     let co = fiarg.n_corant;
/*Ic*/          let i_parameters = fiarg.init_conditions;
/*It*/          let i_type = fiarg.init_type;
                let bound = fiarg.bound_type;
/*Transfer_velocity*/let velocity_t = fiarg.add_args.0.clone();
let time_decrease: f64 = 20.0;//additional parameter to determine size of time_vec in real
let switch_time = advection_modes.4;
let type_of_correction_program = true;
/*period of end and output*/let time_ev = fiarg.time_eval_period_stage;
    //In any way in process_clfile I had written TRANSFER, so i can switch it there
    //But more convienient as I suppose that if TRANSFER=0_f32, then switch)  
    let veloc: f64 = match velocity_t.expect("Maybe velocity not specified"){
        TaskType::Transfer{a} => {
            if debug_add{
                println!("Speed: {}", a);
                if LETS_DO_PAUSE{
                    my_time_counter.add_duration(1, None);
                }
            } 
                a},
        TaskType::Burger(a, b) => {println!("However, this is burger equation: {:?} {:?}", a, b); 0_f64},
    };
    println!("Velocity from input: {veloc}");
    let a_positive: bool = veloc > 0.0; //add parameter to detect sheme later
    info!("Sign of speed: {}\n", a_positive);
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    let (mut first_ex , mut second_ex , mut temporary, mut vprevious, mut inner_vector, mut diferr_0, mut x_v_w_txt_0, mut x_v_w_csv_0, smax) = 
        main_initialization(steps, debug_add, calculation_path_as_str, fi.clone(), 
            equation, i_type, dx, i_parameters.0, i_parameters.1, i_parameters.2.unwrap_or(0_f64), veloc, domain.0, domain.1, CHECK_ENDS_OF_DOMAIN);
//----------------------------------------------------------------------------------------
//________________________________Some precycle clarification_______________________//  
    let elements_per_raw_pyarray: usize = ((steps as f32).floor()) as usize;//This will output array with this or less amount of columns
    let existing_time = temporary.iter().min_by(|a, b|
        a.partial_cmp(&b).unwrap_or(Ordering::Less)).unwrap_or(&0_f64);
    println!("Minimum in temporary error vector: {}", &existing_time); 
    let t_max = -1_f64/existing_time;
    if equation==1 {
        println!("Existing minimum time of burger: {} and will live: {}",
            existing_time, t_max);
        info!("{}", format!("Existing minimum time of burger: {}", existing_time));
        info!("{}", format!("And so the maximum live time will be: {}", t_max));//........................................
    }
    let possgn_smax = smax > 0_f64;// later to switch scheme equation
    println!("Maximum velocity from first initial layer: {}", smax);
    let fuu = match &equation{
        0 => veloc,
        1 => 0.0, //Further in main cycle will determine this
        _ => 0.0
    };
    println!("fuuuu: {fuu}, {veloc}");
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
    let output_time_max = time_ev.0;
    let mut output_time_rate = time_ev.1;
    //To pass in method std::Duration as u64 digit I need to converge it
    let max_time_output_precised_secs = (((output_time_max * 1000_000_000_f64)).ceil() as u64)/1000_000_000_u64;
    let max_time_output_precised_nanosecs = ((output_time_max * 1000_000_000_f64)).ceil() as u64;
    let time_output_precised_secs = (((output_time_rate * 1000_000_000_f64)).floor() as u64)/1000_000_000_u64;
    let time_output_precised_nanosecs = ((output_time_rate * 1000_000_000_f64)).floor() as u64;
    //__________________________________________________________________________//
    let maxl_time_secs  = SDuration::from_secs(max_time_output_precised_secs);// below, to set precision up to 6 characters after commas 
    let maxl_time_nanosecs  = SDuration::from_nanos(max_time_output_precised_nanosecs);
    let mut maxl_time_secs = maxl_time_secs.as_secs();
    let maxl_time_nanosecs = maxl_time_nanosecs.as_nanos();
/*Period of output*/let out_time_secs = SDuration::from_secs(time_output_precised_secs); 
    let out_time_nanosecs = SDuration::from_nanos(time_output_precised_nanosecs); 
    let mut out_time_secs = out_time_secs.as_secs();
    let out_time_nanosecs = out_time_nanosecs.as_nanos();
//---------------------------------------------------------------------------------------
/*step on y*/let dt_from_init = match equation {
            0 => if a_positive {co * dx/(smax)} else {co * dx/(-smax)},
            1 => if possgn_smax {co * dx/(smax)} else {co * dx/(-smax)},
            _ => panic!("Not type match")
        };
        let dt = if dt_from_init< 0.000001 {panic!("t step too small, please coreect initial conditions")} 
        else{
            dt_from_init
        };
        //Amount of steps vertically = dt * #steps vertically which in turn determine by maxl_time 
    let height = (output_time_max as f64 / dt as f64).ceil() as usize;
    let width = if SIMPLE_STEP_TYPE {steps} else { steps + 2_usize};
    let smooth_correction: bool = advection_modes.1;
    let smooth_intensity = 0.2;
    let left_domend = domain.0;
    let right_domend = domain.1;
    let centre_mat_expect = i_parameters.0;
    let width_alpha = i_parameters.1;
    let height_init = i_parameters.2.unwrap_or(0.0);
        let mut prediction = vec![0_f64; width];
        let mut first_correction = vec![0_f64; width];
        let mut second_correction = vec![0_f64; width];
        let mut fu_next: f64 = 0.0;   
        let mut fu_prev: f64 = 0.0;
        let mut y_index: usize = 0;//whole vertical direction over time vector
        let mut x_index: usize = 0;//whole horizont over time vector
        let mut cur_period: f64 = 0.0;
        let mut period: usize = 1_usize;
        let mut output_periods: Vec<usize> = Vec::new();
        let all_steps = if SIMPLE_STEP_TYPE {steps} else {steps + 2_usize};  
//REDUCE_TIME_TO_INT
    let print_npy = DIVIDE_ALL_STEPS_TO_PYTHON_PIC;
        //save here for numerical and exact output
        let vec_output = vec![
            vec![0_f64; time_decrease.ceil() as usize /* * size_time */* (output_time_max / output_time_rate) as usize  + 2_usize], 
            vec![0_f64; time_decrease.ceil() as usize /* * size_time */ * (output_time_max / output_time_rate) as usize  + 2_usize]];
        let mut vector_time: Vec<f64>= if !switch_time {
            //What that size? Out_time / time_rate = #amount of times to output, so this must be * on amount of elements per horizontal
            //Vec::<f64>::with_capacity((output_time_max as f64 * print_npy as f64/ output_time_rate as f64).ceil() as usize)
            vec![0_f64; (output_time_max as f64 / (output_time_rate as f64 * dt as f64)) as usize + 100_usize] // print_npy as f64
        }
            else{
                //What that size? This depends on cycle time as output_time_rate(cycle time as unit of measure)
                //vec![0_f64; (output_time_max / (output_time_rate * cycle_time)).ceil() as usize]
                Vec::new()
        };
        let mut vector_time_exact = if !switch_time {
            //What that size? Out_time / time_rate = #amount of times to output, so this must be * on amount of elements per horizontal
            //Vec::<f64>::with_capacity((output_time_max as f64 * print_npy as f64/ output_time_rate as f64).ceil() as usize)
            vec![0_f64; (output_time_max as f64/ (output_time_rate as f64 * dt as f64)) as usize + 100_usize]// print_npy as f64
        }
            else{
                //What that size? This depends on cycle time as output_time_rate(cycle time as unit of measure) but this will be 
                //measured in main cycle after conditional time
                Vec::new()
        };

    let do_step_reduce_now = DO_STEP_REDUCE;
    let dir_to_graphics: PathBuf = calculation_data_path.join("datas");
    show_shape(all_steps, print_npy, &vprevious, &first_ex, &calculation_data_path, fi, "This is the time after initializing shape", Some("the_beggining_shape"), deb_my);
    if deb_my{
        println!("{} {} {} {}", ansi_term::Colour::Yellow.underline().paint(format!("Switch time mode - ")), switch_time,
            ansi_term::Colour::Yellow.underline().paint(format!("Correction time mode - ")), correction);
    }
        //Loops dtermined by dt
        let mut processed_time_nanos = chrono::Duration::nanoseconds(0);
    //This measure current time layer by layer determined by dt
        let mut current_time_on_dt = 0_f64;//will be increased by every time(dt) loop
        let mut exact_float_time_dif: f64 = output_time_max;
//This means step by which aliquot will be reported in time vec(horizontal step)
        let hor_time_step = if do_step_reduce_now {
            (all_steps as f64/ print_npy as f64).floor() as usize}
            else{
                1 as usize
            };
        let begin_of_main= Instant::now(); 
        let mut curtime_on_vel = 0.0;            
        let fp_next: f64 = 0.0;
        let fp_prev: f64 = 0.0;
        let mut time_dif_in_nanos: f64 = maxl_time_nanosecs as f64;
        let mut time_dif_in_secs: f64 = maxl_time_secs as f64;
//Needed to measure real-time rate
    let mut begin_of_cycle; 
    let mut cycle_time = chrono::Duration::nanoseconds(0);
    let mut cycle_time_nanos: i64 = cycle_time.num_nanoseconds().unwrap();
    let mut fi_exp_path = PathBuf::new();
    fi_exp_path = exp_path.join(format!("exp_{}", fi));
    let fi_exp_path_update = fi_exp_path.clone();
    if all_exact_record{
        fs::create_dir_all(fi_exp_path).unwrap();
    }
    let mut output_time_rate_add_calc = output_time_rate;
    let mut output_time_rate_add = output_time_rate;
        println!("Approximate equal to 3 float digits? - {}", approx_equal(time_dif_in_secs, 0.0, 3));
//+++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
        'main_cycle: while exact_float_time_dif > 0.0 {
        let updating_x_u_w =  if all_exact_record{
            Some(fi_exp_path_update.join(format!("x_u_w_{0}=itype{1}={2}.txt", fi, i_type, period)))
        }
        else{ None };
        let buf_def = PathBuf::from(format!("/home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/advection/src/animation/datas/example_datas/differ_errors_{0}", fi));
        let new_buf_def = buf_def.clone();
        fs::create_dir_all(new_buf_def).unwrap();
        let mut union_x_u_w = std::fs::File::create(updating_x_u_w.unwrap()).unwrap();
        println!("exact_float_time_dif {}\n current_time_on_dt - output_time_rate {}", exact_float_time_dif, current_time_on_dt - output_time_rate);
        if SHOULD_SLEEP_IN_MAIN{thread::sleep(SDuration::from_secs(1_u64));}
            begin_of_cycle = Instant::now();
            if deb_my {
                println!("Approximate equal to 3 float digits? - {}", approx_equal(maxl_time_nanosecs as f64 - current_time_on_dt, 0.0, 3));
                println!("{} and {}", ansi_term::Colour::Yellow.underline().paint(format!("Rest time before loop(nanosecs_to_float): {time_dif_in_nanos:6}")), 
                ansi_term::Colour::Yellow.underline().paint(format!("While clause(secs): {}" , (maxl_time_secs as f64 - current_time_on_dt))));
                println!("all_steps: {} ", all_steps);
            }
            //thread::sleep(SDuration::from_millis(200_u64));
            curtime_on_vel = current_time_on_dt * fuu;
            union_x_u_w.write_all("x, exv, numv\n".as_bytes()).unwrap();
            let write_gen = true;
//Simply calculate second layer based on previous one
            if !correction{
                output_time_rate_add = main_cycle_first_order(&mut vprevious, &mut inner_vector, &mut first_ex, &mut union_x_u_w, fuu, fu_next, fu_prev, dt, dx,
                    equation, bound,curtime_on_vel, current_time_on_dt, output_time_rate_add, output_time_rate, a_positive, possgn_smax, i_type,
                    left_domend, width_alpha, height_init, all_steps, &buf_def, period, deb_my, write_gen,fi).expect("Nothing special");
                    //println!("vprevious: {:?}\n inner_vector: {:?}", vprevious, inner_vector);
                }
//Otherwise  calculate  with correction
            else{
                output_time_rate_add = main_cycle_with_correction(&mut vprevious, &mut inner_vector, &mut prediction, &mut first_correction, &mut second_correction, &mut first_ex,
                    fuu, fu_next, fu_prev, fp_next, fp_prev, dt, dx, equation, bound, 
                    all_steps, deb_my, type_of_correction_program, smooth_intensity, width_alpha, height_init, a_positive, period, i_type, fi, &mut union_x_u_w, &buf_def, left_domend,
                    curtime_on_vel, current_time_on_dt, output_time_rate_add ,  output_time_rate).expect("Nothing special");
            }      
//++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++
            do_exact_solutions(equation, all_steps, left_domend, dx, current_time_on_dt, output_time_rate, fuu, curtime_on_vel, width_alpha, height_init,
                deb_my,  &mut vprevious, &mut first_ex, &mut second_ex);
//------------------------------------------------------------------------------
            if ADDITION_OF_TIME_VECTORS{

            }
            else{

            }
            //swap them and delete old data
            if deb_my{
                let vprev_str = vprevious.clone().iter().join("\t");//.iter().map(|s| ToString::from(s)).collect();
                let inner_vector = inner_vector.clone().iter().join("\t");
                pt!("Upper and lower layers after cycle: ", "time");
                pt!(vprev_str);pt!("/n"); pt!(inner_vector);
            }
            //Calculate time per cycle, remaining and other time
            let new_cycle = std::time::Instant::now();
            let elapsed_in = begin_of_main.elapsed();
             println!("\nMain calculation: {:?} ^ {:?}", elapsed_in, begin_of_main.duration_since(new_now));
            time_dif_in_nanos = maxl_time_nanosecs as f64 - current_time_on_dt;
            if switch_time {
                //Loops made on real-time 
                time_dif_in_secs = maxl_time_secs as f64 - elapsed_in.as_secs() as f64;
                time_dif_in_nanos = (maxl_time_nanosecs as f64 - elapsed_in.as_nanos() as f64 )/ 1000_000_000_f64;
                cycle_time = chrono::Duration::from_std(new_now - begin_of_cycle).unwrap();//Duration::nanoseconds(
                cycle_time_nanos = cycle_time.num_nanoseconds().unwrap();
                cur_period+= dt;
                println!("Duration on cycle: {}", cycle_time);
                info!("This time extract cycle_end of one horiz. step(millis) {:?}", cycle_time_nanos / 1000_000_i64);
            }
            else{
            //Now let's save datas to create animations further. 
            (x_index, y_index, output_time_rate_add_calc) = calculate_output_time_vec_based_on_outtime_rate(all_steps, current_time_on_dt, hor_time_step,
                x_index, y_index, output_time_rate, output_time_rate_add_calc, &mut vector_time, &mut vector_time_exact, &vprevious/*because it is current swapped*/, &first_ex,
                do_step_reduce_now, print_npy, Some(deb_my));
                exact_float_time_dif = output_time_max - current_time_on_dt;
                time_dif_in_nanos = maxl_time_nanosecs as f64 - current_time_on_dt;
                time_dif_in_secs = maxl_time_secs as f64 - current_time_on_dt;
            //println!("vector_time: \n{:?} ^vector_time_exact: \n{:?}", vector_time, vector_time_exact);
            println!("dt- {}", dt);
        if SHOULD_SLEEP_IN_MAIN{thread::sleep(SDuration::from_secs(1_u64));}
            }
            period+=1_usize;
            vprevious.copy_from_slice(&inner_vector);
            inner_vector = vec![0.0; all_steps];
            current_time_on_dt += dt;// move up
            //Measure real-time from procesing programm
            processed_time_nanos = processed_time_nanos.checked_add(&chrono::Duration::from_std(new_now - begin_of_main).unwrap()).unwrap();
            println!("{} ^ {}", processed_time_nanos, maxl_time_secs as f64 - elapsed_in.as_secs() as f64 * cur_period as f64);
            println!("{}", 
                ansi_term::Colour::Yellow.underline().paint(format!("Real-time elapsed: {processed_time_nanos:6?}")));
            if vprevious.iter().all(|&v| approx_equal(v, 0.0, 7)){
                warn!("Main cycle has been broken before designated time!-cause all numeric elements equal to zero with 3 digit precision");
                if vector_time.iter().all(|&v| approx_equal(v, 0.0, 3)) &&
                vector_time_exact.iter().all(|&v| approx_equal(v, 0.0, 3)){
                    panic!("Nothing has been output in png as output rate too large to fix things now");
            }
            break 'main_cycle;
        }
    }
        let animation_path = env::current_dir().unwrap().join("src").join("animation");
        let calculation_anim_path = animation_path.join("datas");
        let t_maxx = if equation ==0 {None} else {Some(t_max)}; //&vector_time, &vector_time_exact,
        show_shape(all_steps, all_steps, &vprevious, &inner_vector, &calculation_anim_path, fi, "This is the time after all processed time.", Some("the_ultimate_shape"), deb_my);
        //save_files(&calculation_anim_path,  vector_time, Some(vector_time_exact), (all_steps, Some(left_domend), Some(right_domend)), Some(print_npy), 
        //    fi, y_index, Some(true), Some(true), Some(deb_my));
        let determined_step = if IS_CHOSEN_WRITE_IN_MAIN_CYCLE{
            all_steps
        }
        else{
            print_npy
        };
        let write_res = add_additional_info_in_datas_end(&calculation_anim_path, fi, t_maxx, Some(determined_step));
    });
    let end_of_program = std::time::Instant::now();
    println!("Programm had been finished at: {:?}", end_of_program.duration_since(advection_start));
}/* 
extern crate once_cell;
extern crate log4rs;

use log::{info, warn, LevelFilter};
use log4rs::append::file::FileAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};
ANSIByteStrings(&[
    Green.paint("user data 1\n".as_bytes()),
    Green.bold().paint("user data 2\n".as_bytes()),
]).write_to(&mut std::io::stdout()).unwrap();
let dirs = dirs.map(|file| file.unwrap().path());
let logfile = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{l} - {m}\n")))
        .build("log/output.log").unwrap();

    let config = Config::builder()
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(Root::builder()
                   .appender("logfile")
                   .build(LevelFilter::Info)).unwrap();

    log4rs::init_config(config).unwrap();

    log::info!("Hello, world!");   // more program logic goes here...

}     
*/
