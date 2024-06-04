use std::env;

mod image_manager;
mod data_reader;
mod gps;
mod exif;
mod idf;

// Main function
// arg[0] - program path
// arg[1] - photo path

fn main() {
    let path: String = get_path_from_args().unwrap();
    let mut buffer: Vec<u8> = image_manager::open_image(path);
    let mut image_data: image_manager::Image = image_manager::find_exif(&buffer).expect("No EXIF data found!");   

    println!("------------ IDF TAGS ------------");
    idf::idf_tags(&buffer, &mut image_data);

    println!("------------ GPS TAGS ------------");
    if image_data.gps_segment_start != 0 {
        gps::gps_tags(&buffer, &mut image_data);
    }
    else {
        println!("No GPS tags found!");
    }

    println!("------------ EXIF TAGS ------------");
    if image_data.exif_ifd_segment_start != 0 {
        exif::exif_tags(&buffer, &mut image_data);
    }
    else {
        println!("No EXIF tags found!");
    }
}

fn get_path_from_args() -> Option<String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <path to image>", args[0]);
        return None;
    }
    Some(args[1].clone())
}
