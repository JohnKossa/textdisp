# Text Displayer

Render images as ascii text.

## Usage

```textdisp.exe path\to\image.png```
will generate an ascii image 80 characters wide and 40 characters tall of the image in path\to\image.png

The default dimensions are 80 by 40.

```textdisp.exe path\to\image.png --width=120 --normalize=true```
will generate an ascii image 120 characters wide.
The width will be determined by the aspect ratio of the image.
This will also normalize the image's value range.

```textdisp.exe path\to\image.png --width=120 --height=80 --invert=true```
will generate an ascii image 120 characters wide and 80 characters high.
Image will be stretched or squashed to fit if the aspect ratio does not match.
The colors will also be value inverted (so black becomes white and white becomes black).

## Planned Future Work

1. ~~Support for dynamic ranging (image adjusts scale from absolute to relative to the most and least bright regions in the source image)~~
2. ~~Convert this from an executable to a library module, so I can reuse it in some of my other projects.~~
3. ~~Match text to image aspect ratio automatically if desired. (Scale and crop functionality)~~
4. Option for an "extended" ascii set that provides more granularity in brightness value but it does look a bit more cluttered.
5. I have an idea for how to generate different ascii outputs depending on the target font, analyzing it and building a gradient from it from scratch.
6. Speed upgrades? Maybe Rayon, but we'll see. It's pretty fast as is.
