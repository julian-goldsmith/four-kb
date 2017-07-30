use mesh::Mesh;
use gl::types::*;
use cgmath::{Matrix4,Decomposed,Basis3,Vector3,Deg,Rotation3};

pub struct Object {
	pub name: String,
	pub mesh: Mesh,
	
	pub transform: Decomposed<Vector3<GLfloat>, Basis3<GLfloat>>,
}

impl Object {
	pub fn draw<T: Into<Matrix4<GLfloat>>>(&mut self, proj: T) {
        let proj_mat = proj.into();
		&self.mesh.draw(&proj_mat, &self.transform);
		
		self.transform.rot = self.transform.rot * Basis3::from_angle_z(Deg(-0.375));
	}
}
