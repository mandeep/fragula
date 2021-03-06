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

    $  fragula examples/suzanne.obj examples/fragment.glsl

If you have an image texture that you would like to use in your
fragment shader you can use the `--texture` or `-t` flag:

    $  fragula examples/spot.obj examples/fragment.glsl -t examples/spot_texture.png

The left mouse button can be used to rotate the model about the x and y axes. However, for
finer control of the model the following key bindings are available:

```
R: Reset the model back to its original state
W: Rotate the model counter clockwise about the x axis
S: Rotate the model clockwise about the x axis
D: Rotate the model counter clockwise about the y axis
A: Rotate the model clockwise about the y axis
Q: Rotate the model counter clockwise about the z axis
E: Rotate the model clockwise about the z axis
Z: Scale the model in negative increments
X: Scale the model in positive increments
Up: Transalte the model in the positive y direction
Down: Translate the model in the negative y direction
Left: Translate the model in the negative x direction
Right: Translate the model in the positive x direction
```

The vertex shader can be found in the `src` directory and contains the following code:
```glsl
in vec3 position;
in vec3 texture;
in vec3 normal;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;

out vec3 vertex_normal;
out vec3 texture_coordinate;

void main() {
    vertex_normal = normalize(view * model * vec4(normal, 0.0)).xyz;
    texture_coordinate = texture;
    gl_Position = projection * view * model * vec4(position, 1.0);
}
```

Apart from the transformation matrices, `uniform float time` and `uniform vec2 resolution` are
available as uniform variables.

Demo
====
![demo](demo.gif)
