use std::io;
use std::io::Read;
fn main() {
    println!("Hello, world!");
    let file = std::fs::File::open("ab.jpg").unwrap();
    let mut reader = io::BufReader::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer).unwrap();
    //println!("{:?}", buffer);

    // FF D8
    let mut i = 0;
    //let mut size = 0;
    while i < buffer.len() {
        if buffer[i] == 0xFF && buffer[i+1] == 0xD8 {
            println!("Found SOI at {}", i);
            i += 2;
            let size = (buffer[i] as u16) << 8 | buffer[i+1] as u16;
            println!("Size: {}", size);
            i += 2;
        }
        else {
            /*
            01 0F is the ID for the first tag in the first IFD, in this case the manufacturer of the camera
00 02 is the type of the value (2 means it's an ASCII string)
00 00 00 16 is the number of components, meaning we'll have a 22-byte string
00 00 01 B2 (434 decimal) is a pointer to the location of that string, relative to the TIFF header (MM). You can't see it in this screenshot, but it points to 45 41 53 54 4D 41 4E 20 4B 4F 44 41 4B 20 43 4F 4D 50 41 4E 59 00 which is EASTMAN KODAK COMPANY in ASCII
            */
            if buffer[i] == 0x01 && buffer[i+1] == 0x0F {
                //println!("Found tag1 at {}", i);
                let tag = i;
                i += 2;
                if buffer[i] == 0x00 && buffer[i+1] == 0x02 {
                    //println!("Found type string at {}", i);
                    i += 2;
                    let sizerr = (buffer[i] as u32) << 24 | (buffer[i+1] as u32) << 16 | (buffer[i+2] as u32) << 8 | buffer[i+3] as u32;
                    //println!("Size: {}", sizerr);
                    i += 4;
                    let offset = (buffer[i] as u32) << 24 | (buffer[i+1] as u32) << 16 | (buffer[i+2] as u32) << 8 | buffer[i+3] as u32;
                    //println!("Offset: {}", offset);
                    i += 4;
                    //println!("i = {}", i);
                    let mut j = (offset+((i-tag) as u32)) as usize;
                    let mut k = 0;
                    while k < sizerr {
                        print!("{}.", buffer[j] as char);
                        j += 1;
                        k += 1;
                    }
                    println!();
                }
            }
            else {
                i += 1;
            }
        }
    }
}

