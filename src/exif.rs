use crate::data_reader;
use crate::image_manager::Image;
use crate::println;

pub fn exif_tags(buffer: &[u8], image_data: &mut Image) {
    let exif_ifd_segment_start = image_data.exif_ifd_segment_start;
    let tiff_header_start = image_data.tiff_header_start;
    let is_le = image_data.is_le;

    let exif_ifd_no_entries =  data_reader::fetch_u16(&buffer, exif_ifd_segment_start, is_le);
    let mut i = exif_ifd_segment_start + 2;
    for _ in 0..exif_ifd_no_entries {
        let tag : u16 = data_reader::fetch_u16(&buffer, i, is_le);
        let format : u16 = data_reader::fetch_u16(&buffer, i+2, is_le);
        let len : u32 = data_reader::fetch_u32(&buffer, i+4, is_le);

        let size : u32 = len * data_reader::format_size(format);
        let data : u32 = data_reader::fetch_u32(&buffer, i+8, is_le);
        let data16 : u16 = data_reader::fetch_u16(&buffer, i+8, is_le);

        match tag {
            0x829a => println!("Exposure time [s]: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x829d => println!("F-number: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x8822 => println!("Exposure Time: {}", data16),
            0x8824 => println!("Spectral sensitivity: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x8827 => println!("ISO Speed Ratings: [{}, {}]", data_reader::fetch_u16(&buffer, i+8, is_le), data_reader::fetch_u16(&buffer, i+10, is_le)),
            0x8830 => println!("Sensitivity type: {}", data16),
            0x8831 => println!("Standard output sensitivity: {}", data),
            0x8832 => println!("Recomended ExposureIndex: {}", data),
            0x8833 => println!("ISO Speed: {}", data),
            0x8834 => println!("ISO Speed Latitude yyy: {}", data),
            0x8835 => println!("ISO Speed Latitude zzz: {}", data),
            0x9000 => {
                let byte_slice = &buffer[i+8..i+12];
                println!("Exif version: {}", std::str::from_utf8(byte_slice).map(|s| s.to_string()).unwrap_or_else(|e| format!("Failed to convert: {}", e)));
            },
            0x9003 => println!("Datetime original: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x9004 => println!("Datetime original digitized: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x9010 => println!("Offset time: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x9011 => println!("Offset time original: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x9101 => println!("Component configuration: {}", data),
            0x9102 => println!("Avarage compresion ratio of JPEG: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9201 => println!("Shutter speed value: {}", data_reader::fetch_signed_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9202 => println!("Aperture value: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9203 => println!("Brightness [EV]: {}", data_reader::fetch_signed_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9204 => println!("Exposure bias [EV]: {}", data_reader::fetch_signed_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9205 => println!("Max apreture value: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9206 => println!("Distance to focus point [m]: {}", data_reader::fetch_signed_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9207 => println!("Metering mode: {}", data16),
            0x9208 => println!("Light source: {}", data16),
            0x9209 => println!("Flash parameters: {:08b}", data16),
            0x920a => println!("Focal length of lens: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0x9214 => {
                match len {
                    2: println!("Subject area: [{}, {}]", data16, data_reader::fetch_u16(&buffer, i+10, is_le));
                    3: println!("Subject area: [{}, {}, {}]", data_reader::fetch_u16(&buffer, tiff_header_start + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 2 + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 4 + data as usize, is_le));
                    4: println!("Subject area: [{}, {}, {}, {}]", data_reader::fetch_u16(&buffer, tiff_header_start + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 2 + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 4 + data as usize, is_le), data_reader::fetch_u16(&buffer, tiff_header_start + 6 + data as usize, is_le));
                    _: println!("Error while parsing subject area!")
                }
            }
            0x927c => println!("Location of manufacturer dependent internal data: {}", data),
            0x9286 => println!("User comment: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x9290 => println!("Datetime subseconds: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x9291 => println!("Datetime original subseconds: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0x9292 => println!("Datetime digitized subseconds: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0xa000 => {
                let byte_slice = &buffer[i+8..i+12];
                println!("FlashPix version: {}", std::str::from_utf8(byte_slice).map(|s| s.to_string()).unwrap_or_else(|e| format!("Failed to convert: {}", e)));
            },
            0xa001 => println!("Color space: {}", data16),
            0xa002 => println!("Main image width: {}", data),
            0xa003 => println!("Main image height: {}", data),
            0xa004 => println!("Related sound file: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0xa005 => println!("Interoperability IFD pointer: {}", data),
            0xa20b => println!("Flash energy: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0xa20e => println!("FocalPlaneXResolution: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0xa20f => println!("FocalPlaneYResolution: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0xa210 => println!("Unit of FocalPlaneXResoluton/FocalPlaneYResolution ['1' means no-unit, '2' inch, '3' centimeter]: {}", data16),
            0xa214 => println!("Subject location: [{}, {}]", data16, data_reader::fetch_u16(&buffer, i+10, is_le)),
            0xa215 => println!("Exposure index: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0xa217 => println!("Type of image sensor unit: {}", data16),
            0xa300 => println!("FileSource: {}", std::str::from_utf8(&buffer[i+8..i+9]).map(|s| s.to_string()).unwrap_or_else(|e| format!("Failed to convert: {}", e))),
            0xa301 => println!("SceneType: {}", std::str::from_utf8(&buffer[i+8..i+9]).map(|s| s.to_string()).unwrap_or_else(|e| format!("Failed to convert: {}", e))),
            0xa401 => println!("Custom image processing: {}", data16),
            0xa402 => println!("Exposure mode: {}", data16),
            0xa403 => println!("White balance: {}", data16),
            0xa404 => println!("Digital zoom ratio: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0xa405 => println!("Focal length in 35mm film: {}", data16),
            0xa406 => println!("Scene campute type: {}", data16),
            0xa407 => println!("Gain control: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),
            0xa408 => println!("Contrast: {}", data16),
            0xa409 => println!("Saturation: {}", data16),
            0xa40a => println!("Sharpness: {}", data16),
            0xa40c => println!("Subject distance range: {}", data16),
            0xa420 => println!("Unique image ID: {}", std::str::from_utf8(&buffer[i+8..i+41]).map(|s| s.to_string()).unwrap_or_else(|e| format!("Failed to convert: {}", e))),
            0xa430 => println!("Camera owner name: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0xa431 => println!("Body serial number: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0xa432 => println!("Lens specs: [Min focal lenght (mm): {},  Min focal lenght (mm): {}, Min F number: {}, Max F number: {}]", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le), data_reader::fetch_rational(&buffer, tiff_header_start + 8 + data as usize, is_le), data_reader::fetch_rational(&buffer, tiff_header_start + 16 + data as usize, is_le), data_reader::fetch_rational(&buffer, tiff_header_start + 24 + data as usize, is_le)),
            0xa433 => println!("Lens manufacturer: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0xa434 => println!("Lens model: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0xa435 => println!("Lens serial number: {}", data_reader::fetch_null_terminated_str(&buffer, tiff_header_start + data as usize)),
            0xa500 => println!("Gamma: {}", data_reader::fetch_rational(&buffer, tiff_header_start + data as usize, is_le)),

            _ => println!("OTHER: TAG: 0x{:04X} | format {} | size {} | data {}", tag, format, size, data),
        }
        i += 12;
    }
}