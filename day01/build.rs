use std::env;
use std::io::Write;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("input.rs");
    let mut inputs: Vec<String> = Vec::new();
    {
        let input_file = File::open("src/input.txt").unwrap();
        let buf_reader = BufReader::new(input_file);
        let lines = buf_reader.lines();
        for l in lines {
            inputs.push(l.unwrap());
        }
    }
    
    let output_file = File::create(&dest_path).unwrap();
    write!(&output_file, "const INPUT_SIZE: usize = {};\n", inputs.len()).unwrap();
    write!(&output_file, "static INPUT: [u32; INPUT_SIZE] = [\n").unwrap();
    for l in &inputs {
        write!(&output_file, "{},\n", l).unwrap();
    }
    write!(&output_file, "];\n").unwrap();
    println!("cargo:rerun-if-changed=src/input.txt");
} 
