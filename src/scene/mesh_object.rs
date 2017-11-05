use cgmath::{Vector3, Basis3, Decomposed, Matrix4, Deg, Rotation3};
use time;
use gfx::mesh::Mesh;
use scene::SceneObject;

pub struct MeshObject {
    pub mesh: Mesh,
    pub view: Decomposed<Vector3<f32>, Basis3<f32>>,
}

impl SceneObject for MeshObject {
    fn render(&self, proj: &Matrix4<f32>) {
        self.mesh.draw(&self.view, proj);
    }

    fn think(&mut self, time: time::Timespec) {
        self.view.rot = self.view.rot * Basis3::from_angle_z(Deg(2.0));
    }

    fn get_transform(&self) -> Decomposed<Vector3<f32>, Basis3<f32>> {
        self.view.clone()
    }
}
