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
Options:
- `-p` or `--print` - prints all EXIF tags
- `-o` or `--output` `<outputname.txt>` - saves all EXIF tags to a file named `<outputname.txt>`
- `-c` or `--clean` - removes all EXIF tags and:
    - `-n` / `-new` `<filename.jpg>`- creates a new file named `filename.jpg` with the cleaned EXIF data
    - none - overwrites the original file with the cleaned EXIF data

Additional flags:
- `-h` or `--help` - prints help message
- `-v` or `--version` - prints version

## Example
<table>
    <thead>
        <tr>
            <th>Before</th>
            <th>After</th>
        </tr>
    </thead>
    <tbody>
        <tr>
            <td align="center"><img src="./resources/readme/before1.png" height=300/></td>
            <td align="center"><img src="./resources/readme/after1.png" height=300/></td>
        </tr>
        <tr>
            <td align="center"><img src="./resources/readme/before2.png" height=300/></td>
            <td align="center"><img src="./resources/readme/after2.png" height=300/></td>
        </tr>
        <tr>
            <td align="center"><img src="./resources/readme/before3.png" height=300/></td>
            <td align="center"><img src="./resources/readme/after3.png" height=300/></td>
        </tr>
    </tbody>
</table>


## Tests
todo tests

## Supported file formats
Currently only JPG files are supported.

## Test & Example photos
Photos used in tests and examples are acquired from [github](https://github.com/drewnoakes/metadata-extractor-images)