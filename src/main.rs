use std::env;
use std::fs;
use std::process::Command;
use bfc::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = ((args[1].split(".")).collect::<Vec<_>>())[0];
    let src = match fs::read_to_string(args[1].clone()) {
        Ok(a) => a,
        Err(e) => panic!("{e}"),
    };
    println!("Translating {name} to Rust...");
    let rscode = bfrs(&src);
    println!("Writing {name}.rs...");
    fs::write(format!("{}.rs", name), rscode).unwrap();
    println!("Compiling {name}.rs...");
    let status = Command::new("rustc")
        .arg(format!("{}.rs", name))
        .arg("-o")
        .arg(format!("{}",name))
        .status()
        .unwrap();
    println!("Finished compiling {name}.rs");
    println!("{status}");
}
