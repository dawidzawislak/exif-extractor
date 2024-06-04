use crate::data_reader;
use crate::image_manager::Image;

pub fn gps_tags(buffer: &[u8], image_data: &mut Image) {
    let gps_segment_start = image_data.gps_segment_start;
    let tiff_header_start = image_data.tiff_header_start;
    let is_le = image_data.is_le;
    let gps_no_entries =  data_reader::fetch_u16(&buffer, gps_segment_start, is_le);
    let mut i = gps_segment_start + 2;
    for _ in 0..gps_no_entries {
        let tag : u16 = data_reader::fetch_u16(&buffer, i, is_le);
        let format : u16 = data_reader::fetch_u16(&buffer, i+2, is_le);
        let len : u32 = data_reader::fetch_u32(&buffer, i+4, is_le);

        let size : u32 = len * data_reader::format_size(format);
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