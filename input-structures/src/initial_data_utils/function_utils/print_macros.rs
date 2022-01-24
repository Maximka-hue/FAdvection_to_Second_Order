//pub use crate::initial_data_utils::function_utils::cfutils::shorthand_functions::pt;
use crate::initial_data_utils::function_utils::cfutils::{PrimaryColor, PrintStyle};
use crate::initial_data_utils::function_utils::cfutils::{ForegroundColor, BackgroundColor};
pub use std::borrow::Cow;
pub use better_term::{flush_styles, rainbowify};
pub use termion::color as termcolor;
use std::fmt::Debug;

use simple_colors;

pub mod macro_lrls {
    /*
    #[allow(missing_docs)]
    #[macro_export]
    #[warn(unused_macros)]
    macro_rules! ptc {
         ($a: expr) => {
               pt($a, None, None, None, None)
          };
         ($a: expr, $b: expr) =>{
               pt($a , $b, None, None, None)
          };
         ($a: expr, $b: expr, $c: expr) =>{
               pt($a , $b, $c,None, None)
          };
         ($a: expr, $b: expr, $c: expr, $d: expr) =>{
               pt($a , $b, $c, $d, None)
          };
         ($a: expr, $b: expr, $c: expr, $d: expr, $e: expr) =>{
               pt($a , $b, $c, $d, $e)
          };
    }*/
#[macro_export]
macro_rules! fill_none {
        // No more positions; we're done
     { ($func:expr) ($($arg:expr,)*) [] [] } => {
          ($func)($($arg),*)
     };
     
        // Empty position, use None
     { ($func:expr) ($($arg:expr,)*) [] [? $($pos:tt)*] } => {
          $crate::fill_none! { ($func)($($arg,)* None,) [] [$($pos)*] }
     };
        
        // Optional position with argument, use Some($next)
        { ($func:expr) ($($arg:expr,)*) [$next:expr, $($rest:tt)*] [? $($pos:tt)*] }
        => {
            $crate::fill_none! { ($func)($($arg,)* Some($next),) [$($rest)*] [$($pos)*] }
        };
    
        // Required position with argument, use $next
        { ($func:expr) ($($arg:expr,)*) [$next:expr, $($rest:tt)*] [! $($pos:tt)*] }
        => {
            $crate::fill_none! { ($func)($($arg,)* $next,) [$($rest)*] [$($pos)*] }
        };
    
    }

    fn deduce_default(){
         #[allow(missing_docs)]
         #[macro_export]
         #[allow(unused)]
         macro_rules! tuple_default {
              ($($tup_tys:ty),*) => {
                   ($tup_tys::Default::default(),
                   )*
              };}
         }
    macro_rules! replace_expr {
         ($_t:tt $sub:expr) => {$sub};
    }
    #[allow(missing_docs)]
    #[warn(unused_macros)]
    #[macro_export]
    macro_rules! pt {
        ($($arg:expr),*) => {
            $crate::fill_none!(($crate::initial_data_utils::function_utils::print_macros::shorthand_functions::pt)() [$($arg,)*] [! ? ? ? ?])
        }
    }
    #[allow(missing_docs)]
    #[warn(unused_macros)]
    #[macro_export]
    macro_rules! mypt {
     ($($arg:expr),*) => {
         $crate::fill_none!(($crate::initial_data_utils::function_utils::print_macros::shorthand_functions::pt)() [$($arg,)*] [! ? ? ? ?])
     }
 }
    #[allow(missing_docs)]
    #[warn(unused_macros)]
    #[macro_export]
        macro_rules! scanline {
             ($x: expr) => ({
                  io::stdin().read_line(&mut $x).unwrap();
             });
        }
pub use {mypt,pt};
pub(crate) use scanline;
}
pub mod shorthand_functions {
     use std::time::{Instant};
     use chrono::{Local};
     use super::*;
     pub const  PRINTFUNC_DBGOUT: bool = true;
//_____________________________________________________//
fn define_colour_style(context: &str, desired_colour: Option<PrintStyle>) -> PrintStyle {
     let mut colour = PrintStyle::Dft;
     if let Some(desired_colour) = desired_colour{
          colour = desired_colour;
     }
     else
     {
          if context.contains("t") || context.contains("time"){
               if PRINTFUNC_DBGOUT {println!("Colour for time will be green on yellow: ");}
               colour = PrintStyle::Time;
          }
     else if context.contains("l") || context.contains("loop"){
          if PRINTFUNC_DBGOUT {println!("Colour for time will be blue on cyan: ");}
          colour = PrintStyle::Loop;
     }
     else if context.contains("deb") || context.contains("dbg") || context.contains("debug"){
          if PRINTFUNC_DBGOUT {println!("Colour for time will be red on magenta: ");}
          colour = PrintStyle::Debug;
     }
     else if context.contains("i") || context.contains("impl") || context.contains("debug"){
          if PRINTFUNC_DBGOUT {println!("Colour for time will be red on magenta: ");}
          colour = PrintStyle::Impl;
          }
     //if PRINTFUNC_DBGOUT {println!("Will be used default color style");}
     }
     colour
}
fn define_my_colour_style(context: &str, desired_colour: Option<(&str, &str)>) -> (ForegroundColor, BackgroundColor) {
     let mut colour = (ForegroundColor::Default, BackgroundColor::Default);
     if context.contains("t") || context.contains("time"){
          if PRINTFUNC_DBGOUT {println!("Colour for time will be green on yellow: ");}
          colour = (ForegroundColor::Green, BackgroundColor::Yellow);
         }
     else if context.contains("l") || context.contains("loop"){
          if PRINTFUNC_DBGOUT {println!("Colour for time will be blue on cyan: ");}
          colour = (ForegroundColor::Blue, BackgroundColor::Cyan);
     }
     else if context.contains("deb") || context.contains("dbg") || context.contains("debug"){
          if PRINTFUNC_DBGOUT {println!("Colour for time will be red on magenta: ");}
          colour = (ForegroundColor::Red, BackgroundColor::Magenta);
     }
     else if context.contains("i") || context.contains("impl") || context.contains("debug"){
          if PRINTFUNC_DBGOUT {println!("Colour for time will be red on magenta: ");}
          colour = (ForegroundColor::Green, BackgroundColor::Cyan);
     }
     else{
          colour = (ForegroundColor::Default, BackgroundColor::Default);
     }
     colour
}
 //Little shorthand for debug
pub fn mypt<'a, S: AsRef<str>> (text: S, con_text: Option<&str>, textv: Option<&Vec<String>>, colour_style: Option<(&str, &str)>, data_in_row: Option<u32>)
     where S: Into<Cow<'a, str>> + Debug {
         let mut context =String::new();
         if let Some(context_) = con_text{
             context = context_.to_string();
         }
         else{
             yellow!("Default context");
             context = "default".to_string();
         }
         if let Some(text_vector) = textv{
             for (i,s) in text_vector.iter().enumerate(){
             // prints the text in rainbow colors
                 print!("{}", rainbowify(&format!("{:?} \t", s)));
                 const MAX_TEXT_IN_RAW:u32 =11;
                 const MIN_TEXT_IN_RAW:u32 = 3;
                 if let Some(raw_skip) = data_in_row{
                     if let MIN_TEXT_IN_RAW..= MAX_TEXT_IN_RAW = raw_skip{
                         if i % raw_skip as usize ==0 {print!("\n");}
                     }
                 else{
                     if i % 2 as usize ==0 {print!("\n");}
                 }
             }
         }
     }
     let colst = define_my_colour_style(&context.to_lowercase(), None);
     let cs = colour_style.unwrap_or(("", ""));
     println!("{:?}", colst);
     let TextContext = PrimaryColor::new(colst.0, colst.1);
     let local_time = Local::now();
     TextContext.print(&format!("{} {}", &text.into(), termcolor::Fg(termcolor::Reset))[..]);
     println!();
     if context.contains("time") || context.contains("dbg") || context.contains("debug"){
          dark_green!("Local time: ");
          TextContext.println(&format!("\n\t{}", local_time)[..]);
     }
}
pub fn pt<'a, S: AsRef<str>> (text: S, con_text: Option<&str>,  textv: Option<&Vec<String>>, colour_style: Option<PrintStyle>, data_in_row: Option<u32>)
where S: Into<Cow<'a, str>> + Debug {
    let mut context =String::new();
    if let Some(context_) = con_text{
        context = context_.to_string();
    }
    else{
        yellow!("Default context");
        context = "default".to_string();
    }
    if let Some(text_vector) = textv{
        for (i,s) in text_vector.iter().enumerate(){
        // prints the text in rainbow colors
            print!("{}", rainbowify(&format!("{:?} \t", s)));
            const MAX_TEXT_IN_RAW:u32 =11;
            const MIN_TEXT_IN_RAW:u32 = 3;
            if let Some(raw_skip) = data_in_row{
                if let MIN_TEXT_IN_RAW..= MAX_TEXT_IN_RAW = raw_skip{
                    if i % raw_skip as usize ==0 {print!("\n");}
                }
            else{
                if i % 2 as usize ==0 {print!("\n");}
            }
        }
    }
}
let colour_style = define_colour_style(&context.to_lowercase(), None);//.expect("extracting color style in pt macro");
if PRINTFUNC_DBGOUT {println!("{:?}", colour_style);}
let local_time = Local::now();

if context.contains("time") || context.contains("dbg") || context.contains("debug"){
     green!("Local time: ");
     green!("\n\t\t{}\n", local_time);
          }
green!("{} {}", &text.into(), termcolor::Fg(termcolor::Reset));
     }
}
