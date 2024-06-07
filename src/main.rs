#![allow(warnings)] 

mod image_manager;
mod data_reader;
mod gps;
mod exif;
mod idf;
mod cmd_reader;

use image_manager::Image;
use cmd_reader::Config;

// Main function
// arg[0] - program path
// arg[1] - photo path
// arg[2] - program flags
//          -p -print : print all tags
//          -c -clear -clean : clear all tags
//          -n -new : save image with cleared tags in new file

fn main() {
    let config: Config = cmd_reader::get_path_from_args().unwrap();
    let mut buffer: Vec<u8> = image_manager::open_image(&config.path);
    let mut image_data: Image = image_manager::find_exif(&buffer, &config).expect("No EXIF data found!");   

    if config.print { println!("------------ IDF TAGS ------------"); }
    idf::idf_tags(&buffer, &mut image_data, &config);

    if config.print { println!("------------ GPS TAGS ------------"); }
    match image_data.gps_segment_start {
        0 => println!("No GPS tags found!"),
        _ => gps::gps_tags(&buffer, &mut image_data, &config),
    }
    if config.print { println!("------------ EXIF TAGS ------------"); }
    match image_data.exif_ifd_segment_start {
        0 => println!("No EXIF tags found!"),
        _ => exif::exif_tags(&buffer, &mut image_data, &config),
    }
}
