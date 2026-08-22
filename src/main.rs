use bfc::*;
use std::env;
use std::fs;
use std::process::Command;
use std::path::Path;
use timelog::Timer;

fn main() {
    let mut timer = Timer::new();
    timer.time("compiling");
    let args: Vec<String> = env::args().collect();
    let path = Path::new(&args[1]);
    let name = path.file_stem().unwrap().to_str().unwrap();
    let src = match fs::read_to_string(args[1].clone()) {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };
    println!("Translating {name} to Rust...");
    let rscode = match bfrs(&src) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Err: {e}");
            std::process::exit(1);
        }
    };
    println!("Writing {name}.rs...");
    fs::write(format!("{}.rs", name), rscode).unwrap();
    println!("Compiling {name}.rs...");
    let status = Command::new("rustc")
        .arg(format!("{}.rs", name))
        .arg("-o")
        .arg(format!("{}", name))
        .status()
        .unwrap();
    println!("Finished compiling {name}.rs");
    println!("{status}");
    let duration: f64= timer.time_log("compiling", true);
    println!("Compiled {} for {} ms",name , duration);
}
