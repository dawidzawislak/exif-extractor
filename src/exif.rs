use crate::data_reader;

pub fn exif_tags(buffer: &[u8], exif_ifd_segment_start: usize, is_le: bool) {
    let exif_ifd_no_entries =  data_reader::fetch_u16(&buffer, exif_ifd_segment_start, is_le);
    let mut i = exif_ifd_segment_start + 2;
    for _ in 0..exif_ifd_no_entries {
        let tag : u16 = data_reader::fetch_u16(&buffer, i, is_le);
        let format : u16 = data_reader::fetch_u16(&buffer, i+2, is_le);
        let len : u32 = data_reader::fetch_u32(&buffer, i+4, is_le);

        let size : u32 = len * data_reader::format_size(format);
        let data : u32 = data_reader::fetch_u32(&buffer, i+8, is_le);
        
        match tag {
            _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
        }
        i += 12;
    }
}