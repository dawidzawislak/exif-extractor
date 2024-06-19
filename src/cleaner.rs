use crate::image_manager::Image;

pub fn clean_exif(buffer: &mut Vec<u8>, image_data: &mut Image) {
    let start = image_data.ifd0_segment_start;
    let end =  image_data.exif_segment_size as usize;
    for i in start..end {
        buffer[i] = 0;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clean_exif() {
        let mut buffer = vec![0; 100];
        let mut image_data = Image::default();
        image_data.ifd0_segment_start = 10;
        image_data.exif_segment_size = 20;
        clean_exif(&mut buffer, &mut image_data);
        for i in 10..30 {
            assert_eq!(buffer[i], 0);
        }
    }
}