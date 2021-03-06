use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

use luminance::linear::M44;
use luminance::pipeline::BoundTexture;
use luminance::pixel::NormUnsigned;
use luminance::shader::program::{Program, Uniform};
use luminance::texture::Dim2;
use luminance_derive::UniformInterface;

use crate::vertex::VertexSemantics;

#[derive(UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    pub model:         Uniform<M44>,
    #[uniform(unbound)]
    pub projection:    Uniform<M44>,
    #[uniform(unbound)]
    pub view:          Uniform<M44>,
    #[uniform(unbound)]
    pub time:          Uniform<f32>,
    #[uniform(unbound)]
    pub resolution:    Uniform<[u32; 2]>,
    #[uniform(unbound)]
    pub texture_image: Uniform<&'static BoundTexture<'static, Dim2, NormUnsigned>>,
}

/// Read the contents of a fragment shader file into a String
pub fn create_fragment_shader(file: &Path) -> String {
    let mut fragment_file = File::open(file).unwrap();
    let mut fragment_shader = String::new();
    fragment_file.read_to_string(&mut fragment_shader).unwrap();

    fragment_shader
}

/// Create a shader program from a vertex shader and fragment shader
pub fn create_shader_program(vertex_shader: &String,
                             fragment_shader: &String)
                             -> Program<VertexSemantics, (), ShaderInterface> {
    let program: Program<VertexSemantics, (), ShaderInterface> =
        Program::from_strings(None, vertex_shader, None, &fragment_shader).unwrap()
                                                                          .ignore_warnings();

    program
}
