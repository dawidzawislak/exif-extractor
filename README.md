# EXIF Extractor & Cleaner
The EXIF Extractor is a Rust-based tool designed to efficiently parse, extract, and clean EXIF metadata from JPG images. EXIF (Exchangeable Image File Format) metadata provides essential information about the image, such as camera settings, date and time, and geolocation data.

## How to run
Download project:
```
git clone https://github.com/dawidzawislak/exif-extractor
```
Run the project with the following commands:
```
cd exif-extractor
cargo run <path/to/photo.jpg> <flags>
```
Where flags are:
- `-p` or `--print` - prints all EXIF tags
- `-o` or `--output` - saves all EXIF tags to a file named `output.txt`
- `-c` or `--clean` - removes all EXIF tags and:
    - `-n` / `-new` `<filename.jpg>`- creates a new file named `filename.jpg` with the cleaned EXIF data
    - none - overwrites the original file with the cleaned EXIF data

## Example
todo example  
<img src="./resources/images/example.jpg">

## Tests
todo tests

## Supported file formats
Currently only JPG files are supported.

## Test & Example photos
Photos used in tests and examples are acquired from [github](https://github.com/drewnoakes/metadata-extractor-images)