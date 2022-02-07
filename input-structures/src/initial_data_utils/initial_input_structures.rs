//#[crate_type = "staticlib"]
///Deduce types of variables, store arguments from parsing input
///Create file structures from which programm will read data
use crate::initial_data_utils::function_utils::{cfutils::{Argumento, ArgumentParseFilesError, approx_equal, parse_three, parse_pair, preprocess_text, run},
    print_macros::{ rainbowify, flush_styles, 
    macro_lrls::{pt, mypt, generate_random_parameters}}};
/* Building struct */
pub use derive_builder::Builder;
pub use std::borrow::Cow;
pub use std::default::Default;
use std::fs::{self, File, OpenOptions};
pub use std::path::{self, PathBuf, Path};
use std::time::{self, Instant};
use tcprint::{Color as TColor, ColorSpec};
use std::{io::Write, env, thread};
use ansi_term::{Style, Colour::*};
use std::sync::{Mutex, Arc};
use itertools::{Itertools};
const SWITCH_TIME: bool= false;
use std::collections::HashSet;
use std::process;
use std::{io::ErrorKind, error::{Error as SError}};
type StdResult<T> = std::result::Result<T, Box<dyn SError>>;
///This enumerattion determine type of task: Burger or Advection.
///Specificity of the **first** entered argument (type of equation)
//Специфика введенного первого аргумента (типа уравнения)
#[derive(Debug, Clone, PartialEq)]
pub enum TaskType{
    Burger(BurgerOrder, String),//It's only to choose type of equation 
    Transfer{a: f64},  //,u0_1:f32,u0_2:f32,u0_3:Option<f32>},
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BurgerOrder{
    BurgerFirstOrder, 
    BurgerSecondOrder,
    Arbitrary
}
impl Default for TaskType {
    fn default() -> Self { TaskType::Transfer{a: 1_f64} }
}
///This is for specifying some context in which TaskType will be used.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TaskTypeCs{
    pub burger_cs_first_order: ColorSpec,
    pub burger_cs_second_order: ColorSpec,
    pub burger_cs_world_time: ColorSpec,
    pub burger_cs_step_time: ColorSpec,
}
///Implementation uses crate __tcprint__.
impl Default for TaskTypeCs {
    fn default() -> Self {
        // By default, pet names are printed in bold blue.
        let mut burger_cs_first_order = ColorSpec::new();
        let mut burger_cs_second_order = ColorSpec::new();
        let mut burger_cs_world_time = ColorSpec::new();
        let mut burger_cs_step_time = ColorSpec::new();
        burger_cs_first_order.set_fg(Some(TColor::Blue)).set_bold(true);
        burger_cs_second_order.set_fg(Some(TColor::Green)).set_bold(true);
        burger_cs_world_time.set_fg(Some(TColor::Cyan)).set_underline(true);
        burger_cs_step_time.set_fg(Some(TColor::Yellow)).set_underline(true);
        TaskTypeCs{burger_cs_first_order,
            burger_cs_second_order: burger_cs_second_order,
            burger_cs_world_time, burger_cs_step_time}
    }
}

//____________________________Input file data_____________________________________________________
//This struct creates new file_process manually
#[derive(Debug, Clone, Builder, PartialEq)]
#[builder(build_fn(validate = "Self::validate_parameters"))]
pub struct FileParametres{
    #[builder(public, default = "0_i8")]
    pub eq_type: i8,
    #[builder(default = "(0 as f64, 1 as f64)")]
    pub margin_domain:(f64, f64),
    pub time_eval_period_stage: (f64, f64, Option<bool>),
    pub bound_type: i8,
    pub init_type: i8,
    pub init_conditions: (f64, f64, Option<f64>, Option<f64>),
    pub quantity_split_nodes: f64,
    #[builder(setter(into))]
    pub n_corant: f64,
//#[builder(setter(into, strip_option), default)]- don't work
//will be last background_mc additional_correction
    pub add_args: (Option<TaskType>, Option<i8>, Option<bool>)
    //pub add_args: Vec<Option<TypeTsk>, Option<i8>, Option<bool>> I want like this, but don't know way
}
///Boundary, time and equation check in parameters
impl FileParametresBuilder {
    fn validate_parameters(&self) -> std::result::Result<(), String>{//io::ErrorKind
        if let Some(ref eq_type) = self.eq_type {
            match *eq_type {
                //I know that i will be integer
                i if i < 0 => {pt!("First less than 0, no such type equation", "Impl"); red!("\nNothing right in equation!");
                return Err(format!("Invalid number: {}", i))},//ErrorKind::InvalidData
                i if i > 1 => {pt!("First more than one, no such type equation", "Impl"); red!("\nNothing right in equation!");
                return Err(format!("Invalid number: {}", i))},
                _ => return Ok(())
            }
        }//Notice false and equal start & end time points, then will be checked this circomference
        if let Some(time_way) = self.time_eval_period_stage.unwrap_or((0_f64, 0_f64, Some(false))).2 {
            let input_time_boundary =  self.time_eval_period_stage.unwrap_or((0_f64, 0_f64, None));
            if time_way == true {
                //check for real-time
                return Ok(())
            }
            else {
                if approx_equal(input_time_boundary.0, input_time_boundary.1, 3) {
                //.... They must not be approximately less then 3 decimal points
                return Err(format!("Time boundary is too close for calculation: {} ~ {}", input_time_boundary.0, input_time_boundary.1 ));
                }
                else {
                    red!("Incorrect time specification: {}", self.time_eval_period_stage.unwrap().0);
                    pt!("Please correct programm time boundary", "dbg");
                    red!("Nothing right in time!");
                    return Err(format!("Invalid time: must be {:.3}>{:.3}",
                        input_time_boundary.0,
                        input_time_boundary.1))
                }
                return Ok(())
            }
        }
            let boundary = self.margin_domain.unwrap();//_or((0_f64, 0_f64)
            //(time_period.0 - time_period.1).abs() < std::f32::MIN 
            let left = boundary.0;
            let right = boundary.1;
            let qsn = self.quantity_split_nodes.unwrap_or(100_f64);
            let domain_ends_difference = (left - right).abs();
            let dx = domain_ends_difference / qsn;
            if approx_equal(boundary.0, boundary.1, 3) {
                return Err(format!("Domain boundary is too close for calculation: {} ~ {}", boundary.0, boundary.1 ));
            }
            else if (left - right) < 0.0 {
                self.margin_domain.unwrap().0 = right;
                self.margin_domain.unwrap().1 = left;
                pt!("Boundary ends was swapped!");
            }
            else if approx_equal(left + dx as f64 * qsn, right, 6){
                pt!("With accuracy 10e-6 left + ... = right")
            }
            else if qsn == 3.0 || qsn ==4_f64{
                panic!("Quantity of nodes too few!")
            }
            else if approx_equal(dx, 0.000001, 6) {
                panic!("Fragmentation is too small!")
            }
            return Ok(());
        /*
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
    //return Ok(())
    }
}
//*****************************************************************************************************************************  
impl FileParametres {
    pub fn first_initializing(order_of_equation: u16) -> std::result::Result<FileParametres, ArgumentParseFilesError> {
        let task_order:BurgerOrder = if let  0..=2 = order_of_equation { 
            BurgerOrder::BurgerFirstOrder
        }
        else if order_of_equation == 2 {
            BurgerOrder::BurgerSecondOrder
        }
        else{
            BurgerOrder::Arbitrary
        };
        let datas = FileParametresBuilder::default()
            .eq_type(0)
            .time_eval_period_stage((0 as f64, 0 as f64, None))
            .margin_domain((0 as f64, 0 as f64))
            .bound_type(0)
            .init_type(0)
            .init_conditions((0f64, 0_f64, None, None))
            .quantity_split_nodes(0_f64)
            .n_corant(0 as f64)
            .add_args((Some(TaskType::Burger(task_order, "Some(None) speed initial".to_string())), Some(0_i8), Some(false)))
            .build().unwrap();//.map_err(|_| ErrInTransferTask::FileParams)
        println!("{}", ansi_term::Colour::Green.paint("Initializing struct with default zeros\n"));
        Ok(datas)
    }     
pub fn new(eq_type: i8,
    margin_domain:(f64, f64),
    time_eval_period_stage:(f64, f64, bool),
    bound_type: i8,
    init_type: i8,
    init_conditions: (f64, f64, f64, f64),
    quantity_split_nodes: f64,//Option<i32>,
    n_corant: f64,
    add_args: (TaskType, i8, bool)) -> Result<FileParametres, ()> {
        let new_file_prms = FileParametresBuilder::default()
            .eq_type(eq_type)
            .time_eval_period_stage((time_eval_period_stage.0, time_eval_period_stage.1 , Some(time_eval_period_stage.2)))
            .margin_domain((margin_domain.0, margin_domain.1))
            .bound_type(bound_type)
            .init_type(init_type)
            .init_conditions((init_conditions.0, init_conditions.1, Some(init_conditions.2), Some(init_conditions.3)))
            .quantity_split_nodes(quantity_split_nodes)
            .n_corant(n_corant)
            .add_args((Some(add_args.0), Some(add_args.1), Some(add_args.2)))
            .build().unwrap();
        Ok(new_file_prms)
    }
}
pub fn new_from_str(eq_type: String,
    margin_domain:(f64, f64),
    time_eval_period_stage:(f64, f64, bool),
    bound_type: String,
    init_type: String,
    init_conditions: (f64, f64, f64, f64),
    quantity_split_nodes: String,//Option<i32>,
    n_corant: String,
    add_args: (TaskType, i8, bool)) -> FileParametres {
        FileParametres{eq_type: eq_type.trim().parse::<i8>().unwrap(), //ret: trim-slice, parse- to specified type
            margin_domain: (margin_domain.0, margin_domain.1),
            bound_type: bound_type.trim().parse().expect(" "),
            init_type: init_type.trim().parse().unwrap(),
            init_conditions:(init_conditions.0, init_conditions.1, Some(init_conditions.2), Some(init_conditions.3)),
            quantity_split_nodes: quantity_split_nodes.trim().parse().unwrap(),
            n_corant: n_corant.trim().parse().unwrap(),
            time_eval_period_stage: (time_eval_period_stage.0, time_eval_period_stage.1 , Some(time_eval_period_stage.2)), 
            add_args: (Some(add_args.0), Some(add_args.1), Some(add_args.2)),
    }
}
//*****************************************************************************************************************************
pub fn parse_into_file_parameters(RANDOM_TRANSLATE_MARGINE_BOUNDARY: bool){
    let (mut eq_type, mut bound_type,mut init_type,mut add_args,
        mut time_eval_period_stage, mut init_conditions, mut margin_domain,
        mut quantity_split_nodes, mut n_corant, mut velocity):
        (i8, i8, i8, i8,
        (f64, f64, bool), (f64, f64, f64, f64), (f64, f64),
        f64, f64, f64) = (0i8, 0i8, 0i8, 0i8, (0f64, 0f64, false), (0f64, 0f64, 0f64, 0f64), (0f64, 0f64), 0f64, 0f64, 0f64);
    let random_parameters = generate_random_parameters!().unwrap();
    let int_input = random_parameters.0;
    let float_input = random_parameters.1;
    assert_eq!(int_input.len(), 4);
    assert_eq!(float_input.len(), 11);
    if let Some((req_type, rbound_type, rinit_type, radd_args)) = int_input.into_iter().tuples().next(){
        eq_type= req_type; bound_type= rbound_type; init_type= rinit_type; add_args= radd_args;
    };
    if let Some((rtime_eval_period_stage, rt_one, rinit_conditions, ri_one, ri_two, ri_three, rmargin_domain, rm_one, rquantity_split_nodes, rn_corant, rvelocity)) = 
        float_input.into_iter().tuples().next(){
       //This is arbitrary function for reducing output time[second argument]
        time_eval_period_stage = (rt_one, rtime_eval_period_stage / (if rt_one>1.0{rt_one % 3.0} else{rt_one * 10.0 }), SWITCH_TIME);
        init_conditions = (rinit_conditions, ri_one, ri_two, ri_three);
        if RANDOM_TRANSLATE_MARGINE_BOUNDARY{ 
            let mar_dif = rm_one - rmargin_domain;
            margin_domain = (0.0, if mar_dif< 1.0 {mar_dif * 4.0} else{ mar_dif });
        }
        else{
            margin_domain = (rmargin_domain, rm_one);
        }
        quantity_split_nodes = rquantity_split_nodes;
        n_corant = rn_corant;
        velocity = rvelocity;
        println!("Parameters will be: \ntime_eval_period_stage: {}\ninit_conditions: {init_conditions:#?}\n margin_domain: {}\ntime_eval_period_stage: {}
        \tn_corant: {n_corant} ",
        ansi_term::Colour::Cyan.on(ansi_term::Colour::Fixed(240)).fg(ansi_term::Colour::Fixed(45)).paint(format!("{:#?}", time_eval_period_stage)),
        ansi_term::Colour::Cyan.on(ansi_term::Colour::Fixed(240)).fg(ansi_term::Colour::Fixed(45)).paint(format!("{:#?}", margin_domain)),
        ansi_term::Colour::Cyan.on(ansi_term::Colour::Fixed(50)).fg(ansi_term::Colour::Fixed(200)).paint(format!("{:#?}", quantity_split_nodes)));
    }
    //let transfer_velocity = thread_rng().gen_range(0..10);
    let all_datas = FileParametres::new(eq_type, margin_domain,
            time_eval_period_stage, bound_type, init_type, init_conditions,
            quantity_split_nodes, n_corant,
        //Here I pass additional arguments!If not 0=> will be BURGER type, if !=0, then type TRANSFER
        (TaskType::Transfer{a: velocity}, add_args, false)).expect("Initialization in random generation");
}
/**/
type StdtResult<T> = std::result::Result<Vec<T>, Box<dyn SError>>;
fn process_clfiles<'a>(_datas: FileParametres, new_path_obj: &'a mut Vec<PathBuf>, num_files: Option<usize>, db: &bool) 
-> StdtResult<FileParametres>
{ use tutil::crayon::Style;
    use tutil::crayon::Color::*;
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
    if db {
        println!(" {}" , num_files);}
    let mut fiter = argumento.filenames.chunks(num_files);
//Process every chunk of 2,3 etc parts in threads
    let fp= FileParametres::first_initializing(1).unwrap();
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
    let (x_min, x_max) = parse_pair::<f64>(new_init_data[1].as_str(), ':').expect("Second argument margin_domain must be tuple of pair");
    let (i1,i2,i3) = parse_three::<f64>(new_init_data[5].as_str(), ':').expect("Forth argument is init_conditions, must be three digits here");
    let (t1, t2) = parse_pair::<f64>(new_init_data[2].as_str(), ':').expect("3d argument is time, also three digits");
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
    let all_datas =  FileParametres::new(new_init_data[0].parse::<i8>().unwrap(), (x_min,x_max),
    (t1, t2, false), new_init_data[3].parse::<i8>().unwrap(), new_init_data[4].parse::<i8>().unwrap(), (i1, i2, i3, 0_f64),
    new_init_data[6].parse::<f64>().unwrap(), new_init_data[7].parse::<f64>().unwrap(),
    //Here I pass additional arguments!If not 0=> will be BURGER type, if !=0, then type TRANSFER
    (TaskType::Transfer{a: new_init_data[8].trim().parse().unwrap_or(0_f64)}, 0_i8, false)).unwrap();
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
let result: Vec<FileParametres> = files_vec.lock().unwrap().to_vec().clone();
drop(files_vec);
println!("Processed: {:#?}", result);
Ok(result)  
}
fn create_output_dir(fnum: usize, num_files: usize) -> StdResult<( PathBuf, File )>{
    //Создаем файл с именованными полями
    //let mut temp_directory = env::temp_dir();
    //temp_directory.push("/src");
    let path = env::current_dir().unwrap();
    println!("{} {}", ansi_term::Colour::Cyan.on(ansi_term::Colour::Blue).fg(ansi_term::Colour::Yellow).paint("The current directory is "), path.display());
    let new_path = path.join(format!(r"src\treated_datas_{}", fnum));
    println!("{} {}", ansi_term::Colour::Cyan.on(ansi_term::Colour::Blue).fg(ansi_term::Colour::Green).paint("new_path is "), new_path.display());
    fs::create_dir_all(&new_path).unwrap(); //env::temp_dir();
    let temp_fi = new_path.join(format!(r"parameters_nf{}.txt", fnum));
    //let mut processed_params =  std::fs::File::open(&temp_fi).unwrap();
    //println!("{:?}", processed_params);
    let processed_params =  OpenOptions::new().create(true).write(true)/*.mode(0o770)*/.open(&temp_fi).unwrap_or_else(|error| {
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


pub fn initial_information_of_advection() -> Instant {
    //let ttti = TaskType::default();//TaskType::Burger(ColorSpec::new()).default();
    //This will print initial information about the advection programm and me.
    let now = Instant::now();
    const PROGRAM_NAME:&'static str = "Equations of transfering and Burgens:1st order";
    let mut session_number = String::from("Session number \t");
    session_number.push_str("9\n");
    session_number.push_str(PROGRAM_NAME);
    mypt!(session_number.as_str(), "dbg");
    let mut my_name = String::with_capacity(15);
    my_name.push_str("Maxim_Mochalov");
    println!("\nAutor:\t{}", my_name);
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
