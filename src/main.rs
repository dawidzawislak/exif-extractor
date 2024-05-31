use std::fs::File;
use std::io::{Read};
use std::ffi::CStr;
use std::os::raw::c_char;
use std::str;
use std::io;

fn fetch_u16(data: &[u8], offset: usize, is_le : bool) -> u16 {
    if is_le {
        u16::from_le_bytes([data[offset], data[offset + 1]])
    } else {
        u16::from_be_bytes([data[offset], data[offset + 1]])
    }
}

fn fetch_u32(data: &[u8], offset: usize, is_le : bool) -> u32 {
    if is_le {
        u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
    } else {
        u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]])
    }
}

fn fetch_rational_str(data: &[u8], offset: usize, is_le : bool) -> String {
    if is_le {
        let num = u32::from_le_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
        let den = u32::from_le_bytes([data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]]);
        format!("{}/{}", num, den)
    } else {
        let num = u32::from_be_bytes([data[offset], data[offset + 1], data[offset + 2], data[offset + 3]]);
        let den = u32::from_be_bytes([data[offset + 4], data[offset + 5], data[offset + 6], data[offset + 7]]);
        format!("{}/{}", num, den)
    }
}

fn fetch_null_terminated_str(data: &[u8], offset: usize) -> &str {
    let offset_ptr = unsafe { data.as_ptr().add(offset) };

    let c_str: &CStr = unsafe { CStr::from_ptr(offset_ptr as *const c_char) };

    c_str.to_str().unwrap()
}

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

fn main()  {
    let path = "res/img/test.jpg";

    let mut file = File::open(path).unwrap();

    let mut buffer = Vec::<u8>::new();

    let _ = file.read_to_end(&mut buffer);

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
            if fetch_u16(&buffer, i, is_le) == 0xFFE1 {
                exif_segment_start = i as u16;
                exif_segment_size = fetch_u16(&buffer, i+2, is_le);
                println!("EXIF segment size {} and has size of {} bytes", i, exif_segment_size);
                if fetch_null_terminated_str(&buffer, i+4) != "Exif" {
                    println!("Invalid EXIF segment!");
                    return
                }
                tiff_header_start = i+10;
                if fetch_u32(&buffer, i+10, is_le) == 0x49492A00 {
                    is_le = true;
                } 
                else if fetch_u32(&buffer, i+10, is_le) == 0x4D4D002A {
                    is_le = false;
                }
                no_entries = fetch_u16(&buffer, i+18, is_le);
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
            let tag : u16 = fetch_u16(&buffer, i, is_le);
            let format : u16 = fetch_u16(&buffer, i+2, is_le);
            let len : u32 = fetch_u32(&buffer, i+4, is_le);

            let size : u32 = len * format_size(format);

            let data : u32 = fetch_u32(&buffer, i+8, is_le);

            match tag {
                0x0100 => println!("Image width: {}", data),
                0x0101 => println!("Image height: {}", data),
                0x010F => println!("Manufacturer of the recording equipment: {}", fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
                0x0110 => println!("Recording equipment model: {}", fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
                0x0112 => println!("Orientation: {}", data),
                0x011A => println!("Image resolution in width direction: {}", fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le)),
                0x011B => println!("Image resolution in height direction: {}", fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le)),
                0x0128 => println!("Image resolution unit (inches): {}", data),
                0x0131 => println!("Software used to create image: {}", fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
                0x0132 => println!("Photo created at: {}", fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
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
        let gps_no_entries =  fetch_u16(&buffer, gps_segment_start, is_le);
        i = gps_segment_start + 2;
        for j in 0..gps_no_entries {
            let tag : u16 = fetch_u16(&buffer, i, is_le);
            let format : u16 = fetch_u16(&buffer, i+2, is_le);
            let len : u32 = fetch_u32(&buffer, i+4, is_le);

            let size : u32 = len * format_size(format);
            let data : u32 = fetch_u32(&buffer, i+8, is_le);
            
            match tag {
                0x0001 => println!("N or S latitude: {}", fetch_null_terminated_str(&buffer, i+8)),
                0x0002 => println!("Latitude: {} degs {} minutes {} seconds", fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le), fetch_rational_str(&buffer, tiff_header_start + 8 + data as usize, is_le), fetch_rational_str(&buffer, tiff_header_start + 16 + data as usize, is_le)),
                0x0003 => println!("W or E longitude: {}", fetch_null_terminated_str(&buffer, i+8)),
                0x0004 => println!("Longitude: {} degs {} minutes {} seconds", fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le), fetch_rational_str(&buffer, tiff_header_start + 8 + data as usize, is_le), fetch_rational_str(&buffer, tiff_header_start + 16 + data as usize, is_le)),
                _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
            }
            
            i += 12;
        }
    }
    println!("------------ EXIF TAGS ------------");
    if exif_ifd_segment_start != 0 {
        let exif_ifd_no_entries =  fetch_u16(&buffer, exif_ifd_segment_start, is_le);
        i = exif_ifd_segment_start + 2;
        for j in 0..exif_ifd_no_entries {
            let tag : u16 = fetch_u16(&buffer, i, is_le);
            let format : u16 = fetch_u16(&buffer, i+2, is_le);
            let len : u32 = fetch_u32(&buffer, i+4, is_le);

            let size : u32 = len * format_size(format);
            let data : u32 = fetch_u32(&buffer, i+8, is_le);
            
            match tag {
                
                _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
            }
            
            i += 12;
        }
    }
}