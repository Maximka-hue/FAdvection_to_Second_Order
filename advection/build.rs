use std::path::Path;

fn main() {

let library_path = Path::new("/home/computadormaxim/_Programming_projects/RUSTprojects/FAdvection_to_Second_Order/input-pstructures");
cc::Build::new()
    .file("../input-pstructures/src/smooth.c")
    .include(library_path)
    .include("src")
    .compile("smooth");


}