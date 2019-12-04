use std::collections::HashMap;
use std::fs::File;
use std::io::Read as _;
use std::path::Path;

use luminance::context::GraphicsContext;
use luminance::tess::{Mode, Tess, TessBuilder, TessError};
use try_guard::verify;
use wavefront_obj::obj;

use crate::vertex::{Vertex, VertexIndex, VertexNormal, VertexPosition};

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
                        let p = object.vertices[key.0];
                        let n = object.normals[key.2.ok_or("Missing vertex normals".to_owned())?];
                        let position = VertexPosition::new([p.x as f32, p.y as f32, p.z as f32]);
                        let normal = VertexNormal::new([n.x as f32, n.y as f32, n.z as f32]);
                        let vertex = Vertex { position: position,
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
