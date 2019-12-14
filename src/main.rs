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
        panic!("No file to interpret, use -h flag for more information");
    }
    else if args.len() < 3 {
        panic!("No argument provided, use -h flag for more information");
    }
    else if args.contains(&String::from("-h"))
         || args.contains(&String::from("--help")) {
        display_help();
        std::process::exit(0);
    }

    let buffer = read_input_file(&args[1]).unwrap();

    if (args.contains(&String::from("-t"))
    || args.contains(&String::from("--transpile")))
    && args.len() < 5 {
        panic!("Not enough arguments provided to transpile, use -h flag for more information")
    }
    else if args.contains(&String::from("-t"))
    || args.contains(&String::from("--transpile")) {
        match &*args[3] {
            "rs" | "rust" => transpiler::to_rust(buffer, &args[4])?,
            "c" | "clang" => transpiler::to_c(buffer, &args[4])?,
            _ => panic!("Unknown language, use -h flag for more information"),
        }
    }
    else if args.contains(&String::from("-i"))
    || args.contains(&String::from("--interpret")) {
        let mut interpreter = Interpreter::new(6000);
        interpreter.run(buffer);
    }
    else {
        panic!("Unknown argument, use -h for more information");
    }

    Ok(())
}

fn display_help() {
    println!(r#"
Usage: program [file] [arguments]

Arguments:
-i, --interpret                 Interpret the file
-t, --transpile <lang> <file>   Write transpiled code in <lang> to <file>
                                Available <lang>:
                                    c, clang     C
                                    rs, rust     rust
"#);
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
