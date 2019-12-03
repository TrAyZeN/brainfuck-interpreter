use std::fs::File;
use std::io;
use std::io::{BufReader, Read};
use std::vec::Vec;
use std::env;
use brainfuck_interpreter::interpreter::Interpreter;
use brainfuck_interpreter::transpiler;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("No file to interpret");
    }
    else if args.contains(&String::from("-h"))
         || args.contains(&String::from("--help")) {
        display_help();
        std::process::exit(0);
    }

    let buffer = read_input_file(&args[1]).unwrap();

    if args.contains(&String::from("-t"))
    || args.contains(&String::from("--transpile")) {
        transpiler::to_rust(buffer, "test.rs")?;
        println!("Transpiling");
    }
    else {
        let mut interpreter = Interpreter::new(60000);
        interpreter.run(buffer);
    }
    // transpiler::to_c(buffer, "test.c")?;

    Ok(())
}

fn display_help() {
    println!("Help");
}

fn read_input_file(filename: &str) -> io::Result<Vec<u8>> {
    let file = match File::open(filename) {
        Ok(f) => f,
        Err(e) => panic!("Failed to open file : Error {:?}", e.kind()),
    };

    let mut file_reader = BufReader::new(file);
    let mut buffer = Vec::new();
    file_reader.read_to_end(&mut buffer)?;

    Ok(buffer)
}
