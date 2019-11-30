use luminance::linear::M44;
use luminance::shader::program::Uniform;
use luminance_derive::UniformInterface;

#[derive(Debug, UniformInterface)]
pub struct ShaderInterface {
    #[uniform(unbound)]
    pub projection: Uniform<M44>,
    #[uniform(unbound)]
    pub view: Uniform<M44>,
    #[uniform(unbound)]
    pub rotation: Uniform<M44>,
    #[uniform(unbound)]
    pub translation: Uniform<M44>,
}
