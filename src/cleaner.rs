use crate::image_manager::Image;

pub fn clean_exif(buffer: &mut Vec<u8>, image_data: &mut Image) {
    let start = image_data.ifd0_segment_start;
    let end =  image_data.exif_segment_size as usize;
    for i in start..end {
        buffer[i] = 0;
    }
}