use std::io::Read;
use std::fs::File;
use crate::data_reader;
use crate::cmd_reader::Config;

pub struct Image {
    pub is_le: bool,
    pub tiff_header_start: usize,
    pub no_entries: u16,
    pub curr_ifd_entry: u16,
    pub i: usize,
    pub gps_segment_start: usize,
    pub exif_ifd_segment_start: usize,
    pub found_exif_segment: bool,
    pub exif_segment_start: u16,
    pub exif_segment_size: u16,
    pub ifd0_segment_start: usize,
}

impl Default for Image {
    fn default() -> Self {
        Image {
            is_le: false,
            tiff_header_start: 0,
            no_entries: 0,
            curr_ifd_entry: 0,
            i: 0,
            gps_segment_start: 0,
            exif_ifd_segment_start: 0,
            found_exif_segment: false,
            exif_segment_start: 0,
            exif_segment_size: 100,
            ifd0_segment_start: 0,
        }
    }
}
pub fn open_image(path: &String) -> Vec<u8> {
    let mut file = File::open(path).expect("File not found!");
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).expect("Failed to read file!");
    buffer
}

pub fn find_exif(buffer: &[u8]) -> Option<Image> {
    let mut i = 0;
    let mut is_le = false;
    let mut tiff_header_start = 0;
    let mut no_entries = 0;
    let mut found_exif_segment = false;
    let mut exif_segment_start = 0;
    let mut exif_segment_size = 100;
    let mut ifd0_segment_start = 0;
    while i < exif_segment_start as usize + exif_segment_size as usize  {
        if found_exif_segment == false {
            if data_reader::fetch_u16(&buffer, i, is_le) == 0xFFE1 {
                exif_segment_start = i as u16;
                exif_segment_size = data_reader::fetch_u16(&buffer, i+2, is_le);
                println!("EXIF segment at {} and has size of {} bytes", i, exif_segment_size);
                if data_reader::fetch_null_terminated_str(&buffer, i+4) != "Exif" {
                    println!("Invalid EXIF segment!");
                    return None;
                }
                tiff_header_start = i+10;
                match data_reader::fetch_u32(&buffer, i+10, is_le) {
                    0x49492A00 => is_le = true,
                    0x4D4D002A => is_le = false,
                    _ => panic!("Invalid TIFF header!")
                }
                no_entries = data_reader::fetch_u16(&buffer, i+18, is_le);
                ifd0_segment_start = i + 10 + data_reader::fetch_u32(&buffer, i+14, is_le) as usize;
                println!("IFD0 start {}", ifd0_segment_start);
                println!("No. IFD0 entries {}", no_entries);
                println!("-----------------------------------------------------");
                found_exif_segment = true;

                break;
            }
            else {
                i += 1;
            }
        }
    }
    if found_exif_segment == false {
        println!("No EXIF segment found!");
        return None;
    }
    Some(Image {
        is_le,
        tiff_header_start,
        no_entries,
        curr_ifd_entry: 0,
        i: 0,
        gps_segment_start: 0,
        exif_ifd_segment_start: 0,
        found_exif_segment,
        exif_segment_start,
        exif_segment_size,
        ifd0_segment_start,
    })
}

pub fn save_image(buffer: &[u8], config: &Config) {
    if config.save_new {
        let mut result = File::create(&config.new_path).expect("Failed to create new file!");
        std::io::Write::write_all(&mut result, &buffer).expect("Failed to write to new file!");
        println!("New image saved as new.jpg")
    } else {
        let mut result = File::create(&config.path).expect("Failed to create new file!");
        std::io::Write::write_all(&mut result, &buffer).expect("Failed to write to new file!");
        println!("Image overwritten!")
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_exif_none() {
        let buffer = vec![0; 200];
        let image = find_exif(&buffer);
        assert_eq!(image.is_some(), false);
    }

    #[test]
    fn test_find_exif_some() {
        let mut buffer = vec![0; 200];
        buffer[0] = 0xFF;
        buffer[1] = 0xE1;
        buffer[2] = 0x00;
        buffer[3] = 0x0E;
        buffer[4] = 'E' as u8;
        buffer[5] = 'x' as u8;
        buffer[6] = 'i' as u8;
        buffer[7] = 'f' as u8;
        buffer[8] = 0x00;
        buffer[9] = 0x00;
        buffer[10] = 0x4D;
        buffer[11] = 0x4D;
        buffer[12] = 0x00;
        buffer[13] = 0x2A;
        let image = find_exif(&buffer);
        assert_eq!(image.is_some(), true);
    }
}