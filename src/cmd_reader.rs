use std::env;
use lazy_static::lazy_static;

pub struct Config {
    pub path: String,
    pub new_path: String,
    pub output_path: String,
    pub print: bool,
    pub clean: bool,
    pub save_new: bool,
    pub output: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: String::from(""),
            new_path: String::from("./new.jpg"),
            output_path: String::from("./output.txt"),
            print: false,
            clean: false,
            save_new: false,
            output: false,
        }
    }
}

lazy_static! {
    static ref CONFIG: Config = {
        let args: Vec<String> = env::args().collect();
        if args.len() < 2 {
            println!("Usage: {} <path to image>", args[0]);
            std::process::exit(1);
        }
        let mut config = Config::default();
        config.path = args[1].to_string();
        let mut i = 2;
        while i < args.len() {
            match args[i].as_str() {
                "-p" | "-print"  => config.print = true,
                "-c" | "-clean"  => config.clean = true,
                "-n" | "-new"    => {config.save_new = true; 
                                    config.new_path = args[i+1].to_string(); 
                                    i+=1;}
                "-o" | "-output" => {config.output = true;
                                    config.output_path = args[i+1].to_string(); 
                                    i+=1;}
                _ => panic!("Unknown argument: {}", args[i]),
            }
            i += 1;
        }
        config
    };
}

pub fn get_path_from_args() -> &'static Config {
    &CONFIG
}
