use std::process::Command;
use std::env;

fn main() {
    let o = env::var("OUT_DIR").unwrap();
    assert!(Command::new("gcc").args(&["c/test.c", "-o", format!("{}/tests", o).as_str()]).status().unwrap().success(), "C file failed to build");
}
