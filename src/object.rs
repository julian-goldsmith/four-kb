use mesh::Mesh;
use gl::types::*;
use cgmath::{Matrix4,Decomposed,Basis3,Vector3,Deg,Rotation3};

pub struct Object {
	pub mesh: Mesh,
	
	pub transform: Decomposed<Vector3<GLfloat>, Basis3<GLfloat>>,
}

impl Object {
	pub fn draw(&mut self, proj: &Matrix4<GLfloat>) {
		&self.mesh.draw(proj, &self.transform);
		
		self.transform.rot = self.transform.rot * Basis3::from_angle_z(Deg(-0.375));
	}
}