use std::io::Read;
use std::fs::File;
use crate::data_reader;

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
        }
    }
}
pub fn open_image(path: String) -> Vec<u8> {
    let mut file = File::open(path).unwrap();
    let mut buffer = Vec::<u8>::new();
    file.read_to_end(&mut buffer).unwrap();
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
                break;
            }
            else {
                i += 1;
            }
        }
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
    })
}

