![header](header.png "Fragula")
[![build](https://img.shields.io/travis/com/mandeep/fragula?style=flat-square)](https://travis-ci.com/mandeep/fragula) [![crates](https://img.shields.io/crates/v/fragula?style=flat-square)](https://crates.io/crates/fragula) [![license](https://img.shields.io/crates/l/fragula?style=flat-square)](https://crates.io/crates/fragula)

Fragula lets you view your fragment shader changes in real time. Simply save your fragment
shader while editing and watch your changes appear in the Fragula window in real-time.

Installation
============

To install Fragula run `cargo install fragula` in a terminal prompt.

Usage
=====
```
USAGE:
    fragula [OPTIONS] <obj> <shader>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -t, --texture <texture>    The file path to the texture image to load

ARGS:
    <obj>       The file path to the Obj file to load
    <shader>    The file path to the fragment shader to load

```

To use Fragula you can run `fragula` in a terminal with the path to the
OBJ file and fragment shader you would like to use:

    $  fragula my_3d_model.obj fragment.glsl

If you have an image texture that you would like to use in your
fragment shader you can use the `--texture` or `-t` flag:

    $  fragula my_3d_model.obj fragment.glsl -t my_image_texture.png

You can scale, translate, and rotate your model using the following keys:

```
R: Reset the model back to its original state
W: Rotate the model counter clockwise about the x axis
S: Rotate the model clockwise about the x axis
D: Rotate the model counter clockwise about the y axis
A: Rotate the model clockwise about the y axis
Q: Rotate the model counter clockwise about the z axis
E: Rotate the model clockwise about the z axis
C: Translate the model in the negative x direction
V: Translate the model in the positive x direction
B: Translate the model in the negative y direction
N: Transalte the model in the positive y direction
Z: Scale the model in negative increments
X: Scale the model in positive increments
```

Demo
====
![demo](demo.gif)
