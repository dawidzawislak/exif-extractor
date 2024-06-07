use std::env;

pub struct Config {
    pub path: String,
    pub new_path: String,
    pub print: bool,
    pub clean: bool,
    pub save_new: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: String::from(""),
            new_path: String::from("./new.jpg"),
            print: false,
            clean: false,
            save_new: false,
        }
    }
}

pub fn get_path_from_args() -> Option<Config> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <path to image>", args[0]);
        return None;
    }
    let mut config = Config::default();
    config.path = args[1].to_string();
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "-p" | "-print"            => config.print = true,
            "-c" | "-clear" | "-clean" => config.clean = true,
            "-n" | "-new"              => {config.save_new = true; 
                                           config.new_path = args[i+1].to_string(); 
                                           i+=1;}
            _ => panic!("Unknown argument: {}", args[i]),
        }
        i += 1;
    }
    Some(config)
}
