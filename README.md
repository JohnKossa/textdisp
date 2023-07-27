# Text Displayer

Render images as ascii text.

## Usage

```textdisp.exe path\to\image.png 80 40```
will generate an ascii image 80 characters wide and 40 characters tall of the image in path\to\image.png

```textdisp.exe path\to\image.png 120 80 true```
will generate an ascii image 120 characters wide and 80 characters tall of the image and will also normalize the image's value range

Image will be stretched or squashed to fit if the aspect ratio does not match.

## Planned Future Work

1. ~~Support for dynamic ranging (image adjusts scale from absolute to relative to the most and least bright regions in the source image)~~
2. ~~Convert this from an executable to a library module, so I can reuse it in some of my other projects.~~
3. Match text to image aspect ratio automatically if desired. (Scale and crop functionality)
4. I have an idea for how to generate different ascii outputs depending on the target font, analyzing it and building a gradient from it from scratch.
5. Speed upgrades? Maybe Rayon, but we'll see. It's pretty fast as is.