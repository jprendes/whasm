use std::io::Read;
use whasm::structure::{module::Module};
use whasm::binary::{WasmBinary};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let config = Config::new(&args)
        .unwrap_or_else(|err| {
            println!("Problem pargin input arguments: {}", err);
            println!("Usage: {} [-p|--print] file.wasm", args[0]);
            std::process::exit(1);
        });

    let mut file = std::fs::File::open(&config.filename)
        .unwrap_or_else(|err| {
            println!("Could not open file \"{}\"", config.filename);
            println!("{}", err);
            std::process::exit(1);
        });

    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)
        .unwrap_or_else(|err| {
            println!("Could not read file \"{}\"", config.filename);
            println!("{}", err);
            std::process::exit(1);
        });

    let mut iter = buffer.iter().copied();

    let module: Module = iter.parse()
        .unwrap_or_else(|err| {
            println!("Error parsing file \"{}\".", config.filename);
            println!("{}", err);
            std::process::exit(1);
        });

    if config.print {
        println!("{:#?}", module);
    }
}

struct Config {
    filename: String,
    print: bool,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, Box<dyn std::error::Error>> {
        let mut print = false;
        let mut filename = "/home/jprendes/Projects/Classic/release/matlab.wasm".into();

        let mut args = args.iter();
        let _binname = args.next().ok_or("Not enough arguments.")?;
        for arg in args {
            match &arg[..] {
                "-p" | "--print" => {
                    print = true;
                },
                other => {
                    filename = other.into();
                },
            }
        }

        if filename == "" {
            return Err("Not enough arguments.")?;
        }
        
        Ok( Config { filename, print } )
    }
}