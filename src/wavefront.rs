use std::collections::HashMap;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use luminance::context::GraphicsContext;
use luminance::tess::{Mode, Tess, TessBuilder, TessError};
use try_guard::verify;
use wavefront_obj::obj;

use crate::vertex::{Vertex, VertexIndex, VertexNormal, VertexPosition, VertexTexture};

#[derive(Debug)]
pub struct Obj {
    vertices: Vec<Vertex>,
    indices: Vec<VertexIndex>,
}

impl Obj {
    pub fn to_tess<C: GraphicsContext>(self, ctx: &mut C) -> Result<Tess, TessError> {
        TessBuilder::new(ctx).set_mode(Mode::Triangle)
                             .add_vertices(self.vertices)
                             .set_indices(self.indices)
                             .build()
    }

    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, String> {
        let file_contents = {
            let mut file = File::open(path).map_err(|e| format!("Cannot open file: {}", e))?;
            let mut contents = String::new();
            file.read_to_string(&mut contents).unwrap();
            contents
        };

        let obj_set = obj::parse(file_contents).map_err(|e| format!("Cannot parse: {:?}", e))?;
        let objects = obj_set.objects;

        verify!(objects.len() == 1).ok_or("Expecting a single object".to_owned())?;

        let object = objects.into_iter().next().unwrap();

        verify!(object.geometry.len() == 1).ok_or("Expecting a single geometry".to_owned())?;

        let geometry = object.geometry.into_iter().next().unwrap();

        let mut vertex_cache: HashMap<obj::VTNIndex, VertexIndex> = HashMap::new();
        let mut vertices: Vec<Vertex> = Vec::new();
        let mut indices: Vec<VertexIndex> = Vec::new();

        for shape in geometry.shapes {
            if let obj::Primitive::Triangle(a, b, c) = shape.primitive {
                for key in &[a, b, c] {
                    if let Some(vertex_index) = vertex_cache.get(key) {
                        indices.push(*vertex_index);
                    } else {
                        let v = object.vertices[key.0];
                        let vt = if object.tex_vertices.len() > 0 {
                            object.tex_vertices[key.1.unwrap()]
                        } else {
                            wavefront_obj::obj::TVertex { u: 0.0, v: 0.0, w: 0.0 }
                        };
                        let vn = object.normals[key.2.ok_or("Missing vertex normals".to_owned())?];
                        let position = VertexPosition::new([v.x as f32, v.y as f32, v.z as f32]);
                        let texture = VertexTexture::new([vt.u as f32, vt.v as f32, vt.w as f32]);
                        let normal = VertexNormal::new([vn.x as f32, vn.y as f32, vn.z as f32]);
                        let vertex = Vertex { position: position,
                                              texture: texture,
                                              normal: normal };
                        let vertex_index = vertices.len() as VertexIndex;

                        vertex_cache.insert(*key, vertex_index);
                        vertices.push(vertex);
                        indices.push(vertex_index)
                    }
                }
            } else {
                return Err("Unsupported non-triangle shape".to_owned());
            }
        }

        Ok(Obj { vertices, indices })
    }
}
