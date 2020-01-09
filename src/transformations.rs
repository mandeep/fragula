use cgmath::{perspective, EuclideanSpace, Matrix4, Point3, Rad, Vector3};
use std::f32::consts::PI;

pub fn create_perspective_matrix(z_near: f32, z_far: f32, width: u32, height: u32) -> Matrix4<f32> {
    let fov = Rad(PI / 2.0);

    let projection = perspective(fov, width as f32 / height as f32, z_near, z_far);

    projection
}

pub fn create_view_matrix(eye: Point3<f32>) -> Matrix4<f32> {
    let center = Point3::origin();
    let up = Vector3::unit_y();
    let view = Matrix4::<f32>::look_at(eye, center, up);

    view
}
