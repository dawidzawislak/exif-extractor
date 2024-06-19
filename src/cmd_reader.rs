use lazy_static::lazy_static;
use clap::{Arg, Command};

#[derive(Debug)]
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
        let matches = Command::new("ExifCleaner")
            .version("1.0")
            .about("Czyści dane EXIF z obrazów JPG")
            .arg(Arg::new("path")
                .help("Ścieżka do obrazu")
                .required(true)
                .index(1))
            .arg(Arg::new("print")
                .short('p')
                .long("print")
                .help("Włącza tryb wydruku")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("clean")
                .short('c')
                .long("clean")
                .help("Włącza tryb czyszczenia")
                .action(clap::ArgAction::SetTrue))
            .arg(Arg::new("new")
                .short('n')
                .long("new")
                .help("Zapisuje jako nowy plik")
                .value_name("NEW_PATH")
                .num_args(1))
            .arg(Arg::new("output")
                .short('o')
                .long("output")
                .help("Ścieżka wyjściowa")
                .value_name("OUTPUT_PATH")
                .num_args(1))
            .get_matches();

        let mut config = Config::default();
        config.path = matches.get_one::<String>("path").unwrap().to_string();
        config.print = matches.get_flag("print");
        config.clean = matches.get_flag("clean");
        if let Some(new_path) = matches.get_one::<String>("new") {
            config.save_new = true;
            config.new_path = new_path.to_string();
        }
        if let Some(output_path) = matches.get_one::<String>("output") {
            config.output = true;
            config.output_path = output_path.to_string();
        }
        config
    };
}


pub fn get_path_from_args() -> &'static Config {
    &CONFIG
}
