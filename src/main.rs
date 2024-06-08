mod image_manager;
mod data_reader;
mod gps;
mod exif;
mod idf;
mod cmd_reader;
mod cleaner;
mod output;

use image_manager::Image;

// Main function
// arg[0] - program path
// arg[1] - photo path
// arg[2..] - program flags
//          -p -print : print all tags
//          -c -clean : clean all tags
//          -n -new : save image with cleared tags in new file
//          -o -output : print output to file
fn main() {
    let config = cmd_reader::get_path_from_args();
    let mut buffer: Vec<u8> = image_manager::open_image(&config.path);
    let mut image_data: Image = image_manager::find_exif(&buffer).expect("No EXIF data found!");   

    println!("------------ IDF TAGS ------------");
    idf::idf_tags(&buffer, &mut image_data);

    println!("------------ GPS TAGS ------------"); 
    match image_data.gps_segment_start {
        0 => println!("No GPS tags found!"),
        _ => gps::gps_tags(&buffer, &mut image_data),
    }
    println!("------------ EXIF TAGS ------------");
    match image_data.exif_ifd_segment_start {
        0 => println!("No EXIF tags found!"),
        _ => exif::exif_tags(&buffer, &mut image_data),
    }
    println!();
    
    if config.clean {
        cleaner::clean_exif(&mut buffer, &mut image_data);
        println!("Cleaned exif data!");
        image_manager::save_image(&buffer, &config);
    }
}
