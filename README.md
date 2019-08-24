# Ordered-Dithering-Filter
Reduces an image to an 18 color palette with ordered dithering
![Input Image](https://github.com/ConnorBelman/Ordered-Dithering-Filter/blob/master/images/lenna_input.png)
![Output Image](https://github.com/ConnorBelman/Ordered-Dithering-Filter/blob/master/images/lenna-output.png)

## Build  
`cargo build --release`

## Usage
```
USAGE:
    ordered-dither [OPTIONS] <input_file> <output_file>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -m, --matrix <matrix>    Bayer Matrix to use [default: 4x4]

ARGS:
    <input_file>     input file to be filtered
    <output_file>    path to output file

Matrix sizes: 2x2, 4x4, 8x8, 4x2, 4x1, 8x2, 8x4, 3x3, 5x3
```
