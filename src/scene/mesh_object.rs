use cgmath::Vector3;
use time;
use gfx::mesh::Mesh;
use scene::SceneObject;

pub struct MeshObject {
    pub mesh: Mesh,
}

impl SceneObject for MeshObject {
    fn render(&self) {
    }

    fn get_pos(&self) -> Vector3<f32> {
        Vector3::new(0.0, 0.0, 0.0)
    }

    fn think(&mut self, time: time::Timespec) {
    }
}
