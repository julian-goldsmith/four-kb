use time;
use cgmath::Vector3;

pub trait SceneObject {
    fn render(&self);
    fn get_pos(&self) -> Vector3<f32>;
    fn think(&mut self, time: time::Timespec);
}

struct Scene {
    pub objects: Vec<Box<SceneObject>>,
}
