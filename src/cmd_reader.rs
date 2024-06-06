use std::env;

pub struct Config {
    pub path: String,
    pub print: bool,
    pub clear: bool,
    pub save_new: bool,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: String::from(""),
            print: false,
            clear: false,
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
    for arg in args.iter().skip(2) {
        match arg.as_str() {
            "-p" | "-print"            => config.print = true,
            "-c" | "-clear" | "-clean" => config.clear = true,
            "-n" | "-new"              => config.save_new = true,
            _ => panic!("Unknown argument: {}", arg),
        }
    }
    Some(config)
}
