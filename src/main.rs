use std::env;

mod image_manager;
mod data_reader;
mod exif;

fn format_size(format: u16) -> u32 {
    match format {
        1 => 1,
        2 => 1,
        3 => 2,
        4 => 4,
        5 => 8,
        6 => 1,
        7 => 1,
        8 => 2,
        9 => 4,
        10 => 8,
        11 => 4,
        12 => 8,
        _ => 0
    }
}

// Main function
// arg[0] - program path
// arg[1] - photo path
fn main() {
    let path = get_path_from_args().unwrap();
    let mut buffer: Vec<u8> = image_manager::open_image(path);
    
    let mut found_exif_segment : bool = false;
    let mut exif_segment_start : u16 = 0;
    let mut exif_segment_size : u16 = 100;
    let mut tiff_header_start : usize = 0;

    let mut is_le = false;

    let mut no_entries : u16 = 0;
    let mut curr_ifd_entry : u16 = 0;

    let mut i = 0;

    let mut gps_segment_start = 0;
    let mut exif_ifd_segment_start = 0;

    while i < exif_segment_start as usize + exif_segment_size as usize  {
        if found_exif_segment == false {
            if data_reader::fetch_u16(&buffer, i, is_le) == 0xFFE1 {
                exif_segment_start = i as u16;
                exif_segment_size = data_reader::fetch_u16(&buffer, i+2, is_le);
                println!("EXIF segment size {} and has size of {} bytes", i, exif_segment_size);
                if data_reader::fetch_null_terminated_str(&buffer, i+4) != "Exif" {
                    println!("Invalid EXIF segment!");
                    return
                }
                tiff_header_start = i+10;
                if data_reader::fetch_u32(&buffer, i+10, is_le) == 0x49492A00 {
                    is_le = true;
                } 
                else if data_reader::fetch_u32(&buffer, i+10, is_le) == 0x4D4D002A {
                    is_le = false;
                }
                no_entries = data_reader::fetch_u16(&buffer, i+18, is_le);
                println!("IFD0 start {}", i + 10 + buffer[i+14] as usize);
                println!("No. IFD0 entries {}", no_entries);
                println!("-----------------------------------------------------");
                found_exif_segment = true;
                i += 20;
            }
            else {
                i += 1;
            }
        }
        else {
            // Reading IFD entries
            let tag : u16 = data_reader::fetch_u16(&buffer, i, is_le);
            let format : u16 = data_reader::fetch_u16(&buffer, i+2, is_le);
            let len : u32 = data_reader::fetch_u32(&buffer, i+4, is_le);

            let size : u32 = len * format_size(format);

            let data : u32 = data_reader::fetch_u32(&buffer, i+8, is_le);

            match tag {
                0x0100 => println!("Image width: {}", data),
                0x0101 => println!("Image height: {}", data),
                0x010F => println!("Manufacturer of the recording equipment: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
                0x0110 => println!("Recording equipment model: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
                0x0112 => println!("Orientation: {}", data),
                0x011A => println!("Image resolution in width direction: {}", data_reader::fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le)),
                0x011B => println!("Image resolution in height direction: {}", data_reader::fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le)),
                0x0128 => println!("Image resolution unit (inches): {}", data),
                0x0131 => println!("Software used to create image: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
                0x0132 => println!("Photo created at: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
                0x8769 => exif_ifd_segment_start = tiff_header_start + data as usize,
                0x8825 => gps_segment_start = tiff_header_start + data as usize,
                _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
            }
            
            i += 12;
            curr_ifd_entry += 1;
            if curr_ifd_entry == no_entries {
                break;
            }
        }
    }
    println!("------------ GPS TAGS ------------");
    if gps_segment_start != 0 {
        let gps_no_entries =  data_reader::fetch_u16(&buffer, gps_segment_start, is_le);
        i = gps_segment_start + 2;
        for _ in 0..gps_no_entries {
            let tag : u16 = data_reader::fetch_u16(&buffer, i, is_le);
            let format : u16 = data_reader::fetch_u16(&buffer, i+2, is_le);
            let len : u32 = data_reader::fetch_u32(&buffer, i+4, is_le);

            let size : u32 = len * format_size(format);
            let data : u32 = data_reader::fetch_u32(&buffer, i+8, is_le);
            
            match tag {
                0x0001 => println!("N or S latitude: {}", data_reader::fetch_null_terminated_str(&buffer, i+8)),
                0x0002 => println!("Latitude: {} degs {} minutes {} seconds", data_reader::fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le), data_reader::fetch_rational_str(&buffer, tiff_header_start + 8 + data as usize, is_le), data_reader::fetch_rational_str(&buffer, tiff_header_start + 16 + data as usize, is_le)),
                0x0003 => println!("W or E longitude: {}", data_reader::fetch_null_terminated_str(&buffer, i+8)),
                0x0004 => println!("Longitude: {} degs {} minutes {} seconds", data_reader::fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le), data_reader::fetch_rational_str(&buffer, tiff_header_start + 8 + data as usize, is_le), data_reader::fetch_rational_str(&buffer, tiff_header_start + 16 + data as usize, is_le)),
                _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
            }
            
            i += 12;
        }
    }
    println!("------------ EXIF TAGS ------------");
    if exif_ifd_segment_start != 0 {
        let exif_ifd_no_entries =  data_reader::fetch_u16(&buffer, exif_ifd_segment_start, is_le);
        i = exif_ifd_segment_start + 2;
        for _ in 0..exif_ifd_no_entries {
            let tag : u16 = data_reader::fetch_u16(&buffer, i, is_le);
            let format : u16 = data_reader::fetch_u16(&buffer, i+2, is_le);
            let len : u32 = data_reader::fetch_u32(&buffer, i+4, is_le);

            let size : u32 = len * format_size(format);
            let data : u32 = data_reader::fetch_u32(&buffer, i+8, is_le);
            
            match tag {
                
                _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
            }
            
            i += 12;
        }
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