use crate::data_reader;
use crate::image_manager::Image;
use crate::println;

pub fn idf_tags(buffer: &[u8], image_data: &mut Image) {
    let no_entries = image_data.no_entries;
    let tiff_header_start = image_data.tiff_header_start;
    let is_le = image_data.is_le;
    let mut i = image_data.ifd0_segment_start + 2;
    for _ in 0..no_entries {
        let tag : u16 = data_reader::fetch_u16(&buffer, i, is_le);
        let format : u16 = data_reader::fetch_u16(&buffer, i+2, is_le);
        let len : u32 = data_reader::fetch_u32(&buffer, i+4, is_le);

        let size : u32 = len * data_reader::format_size(format);
        let data : u32 = data_reader::fetch_u32(&buffer, i+8, is_le);
        let data16: u16 = data_reader::fetch_u16(&buffer, i+8, is_le);
        
        match tag {
            0x0100 => println!("Image width: {}", data),
            0x0101 => println!("Image height: {}", data),
            0x0102 => println!("Number of bits per component: {}, {}, {}", data_reader::fetch_u16(&buffer, tiff_header_start + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 2 + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 4 + data as usize, is_le)),
            0x0103 => println!("Compression scheme: {}", data16),
            0x0106 => println!("Pixel composition: {}", data16),
            0x010E => println!("Image title: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x010F => println!("Manufacturer of the recording equipment: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x0110 => println!("Recording equipment model: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x0112 => println!("Orientation of image: {}", data16),
            0x0115 => println!("Number of components: {}", data16),
            0x011A => println!("Image resolution in width direction: {}", data_reader::fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le)),
            0x011B => println!("Image resolution in height direction: {}", data_reader::fetch_rational_str(&buffer, tiff_header_start + data as usize, is_le)),
            0x011C => println!("Image data arrangement: {}", data),
            0x0128 => println!("Image resolution unit (inches): {}", data16),
            0x0131 => println!("Software used to create image: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x0132 => println!("Photo created at: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x013B => println!("Person who created image: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x0212 => println!("Subsampling ratio of Y to C: [{}, {}]", data_reader::fetch_u16(&buffer, tiff_header_start + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 2 + data as usize, is_le)),
            0x0213 => println!("Y and C positioning: {}", data16),
            0x8298 => println!("Copyright holder: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x8769 => image_data.exif_ifd_segment_start = tiff_header_start + data as usize,
            0x8825 => image_data.gps_segment_start = tiff_header_start + data as usize,
            _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
        }
        
        i += 12;
    }
}