use std::fs::File;
use std::io::prelude::*;

use luminance::linear::M44;
use luminance::shader::program::{Program, Uniform};
use luminance_derive::UniformInterface;

use crate::vertex::VertexSemantics;

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    pub projection:  Uniform<M44>,
    #[uniform(unbound)]
    pub view:        Uniform<M44>,
    #[uniform(unbound)]
    pub rotation:    Uniform<M44>,
    #[uniform(unbound)]
    pub translation: Uniform<M44>,
    #[uniform(unbound)]
    pub time:        Uniform<f32>,
    #[uniform(unbound)]
    pub resolution:  Uniform<[u32; 2]>,
}

pub fn create_fragment_shader(file: &String) -> String {
    let mut fragment_file = File::open(file).unwrap();
    let mut fragment_shader = String::new();
    fragment_file.read_to_string(&mut fragment_shader).unwrap();

    fragment_shader
}

pub fn create_shader_program(vertex_shader: &String,
                             fragment_shader: &String)
                             -> Program<VertexSemantics, (), ShaderInterface> {
    let program: Program<VertexSemantics, (), ShaderInterface> =
        Program::from_strings(None, vertex_shader, None, &fragment_shader).unwrap()
                                                                          .ignore_warnings();

    program
}
