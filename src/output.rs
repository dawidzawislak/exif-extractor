#[macro_export]
macro_rules! println {
    ($($arg:tt)*) => {{
        use std::fs::OpenOptions;
        use std::io::Write;
        use crate::cmd_reader;
        let config = cmd_reader::get_path_from_args();
        if config.print {
            std::println!($($arg)*);
        }
        if config.output {
            let mut file = OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("outp.txt")
                .expect("Failed to open output file!");
            writeln!(file, $($arg)*).expect("Failed to write to output file!");
        }
    }}
}