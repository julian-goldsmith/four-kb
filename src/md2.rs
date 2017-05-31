use byteorder::{LittleEndian, ReadBytesExt};
use mesh::Mesh;
use std::string::String;
use std::io::{Seek, Read, SeekFrom, Error};
use gl::types::*;
use image::Image;
use cgmath::{Vector3};

#[derive(Debug)]
pub struct Md2Header {
	pub ident: u32,
	pub version: u32,
	pub skinwidth: u32,
	pub skinheight: u32,
	pub framesize: u32,
	pub num_skins: u32,
	pub num_vertices: u32,
	pub num_st: u32,
	pub num_tris: u32,
	pub num_glcmds: u32,
	pub num_frames: u32,
	pub offset_skins: u32,
	pub offset_st: u32,
	pub offset_tris: u32,
	pub offset_frames: u32,
	pub offset_glcmds: u32,
	pub offset_end: u32,
}

#[derive(Debug)]
pub struct Md2Texcoord {
	pub s: f32,
	pub t: f32,
}

#[derive(Debug)]
pub struct Md2Triangle {
	pub vertex: [u16; 3],
	pub st: [u16; 3],
}

#[derive(Debug)]
pub struct Md2Vertex {
	pub v: [u8; 3],
	pub normal_index: u8,		// FIXME: anorms.h
}

#[derive(Debug)]
pub struct Md2Frame {
	scale: Vector3<f32>,
	translate: Vector3<f32>,
	name: String,
	verts: Vec<Md2Vertex>,
}

fn read_header(mut reader: &mut Read) -> Result<Md2Header,Error> {
	let ident = reader.read_u32::<LittleEndian>()?;
	let version = reader.read_u32::<LittleEndian>()?;
	let skinwidth = reader.read_u32::<LittleEndian>()?;
	let skinheight = reader.read_u32::<LittleEndian>()?;
	let framesize = reader.read_u32::<LittleEndian>()?;
	let num_skins = reader.read_u32::<LittleEndian>()?;
	let num_vertices = reader.read_u32::<LittleEndian>()?;
	let num_st = reader.read_u32::<LittleEndian>()?;
	let num_tris = reader.read_u32::<LittleEndian>()?;
	let num_glcmds = reader.read_u32::<LittleEndian>()?;
	let num_frames = reader.read_u32::<LittleEndian>()?;
	let offset_skins = reader.read_u32::<LittleEndian>()?;
	let offset_st = reader.read_u32::<LittleEndian>()?;
	let offset_tris = reader.read_u32::<LittleEndian>()?;
	let offset_frames = reader.read_u32::<LittleEndian>()?;
	let offset_glcmds = reader.read_u32::<LittleEndian>()?;
	let offset_end = reader.read_u32::<LittleEndian>()?;
	
	Ok(Md2Header { ident, version, skinwidth, skinheight, framesize, num_skins, num_vertices, num_st, num_tris, num_glcmds, num_frames, offset_skins, offset_st, offset_tris, offset_frames, offset_glcmds, offset_end })
}

fn read_texcoords<T: Read + Seek>(mut reader: &mut T, header: &Md2Header) -> Result<Vec<Md2Texcoord>,Error> {
	reader.seek(SeekFrom::Start(header.offset_st as u64))?;
	
	let mut texcoords = Vec::with_capacity(header.num_st as usize);
	
	for _ in 0..header.num_st {
		let s = reader.read_i16::<LittleEndian>()? as f32 / header.skinwidth as f32;
		let t = reader.read_i16::<LittleEndian>()? as f32 / header.skinheight as f32;
		
		texcoords.push(Md2Texcoord { s, t });
	};
	
	Ok(texcoords)
}

fn read_triangles<T: Read + Seek>(mut reader: &mut T, header: &Md2Header) -> Result<Vec<Md2Triangle>,Error> {
	reader.seek(SeekFrom::Start(header.offset_tris as u64))?;
	
	let mut triangles = Vec::with_capacity(header.num_tris as usize);
	
	for _ in 0..header.num_tris {
		let x = reader.read_u16::<LittleEndian>()?;
		let y = reader.read_u16::<LittleEndian>()?;
		let z = reader.read_u16::<LittleEndian>()?;
		
		let s = reader.read_u16::<LittleEndian>()?;
		let t = reader.read_u16::<LittleEndian>()?;
		let u = reader.read_u16::<LittleEndian>()?;
		
		triangles.push(Md2Triangle { vertex: [x, y, z], st: [s, t, u] });
	};
	
	Ok(triangles)
}

fn read_frames<T: Read + Seek>(mut reader: &mut T, header: &Md2Header) -> Result<Vec<Md2Frame>,Error> {
	reader.seek(SeekFrom::Start(header.offset_frames as u64))?;
	
	let mut frames = Vec::with_capacity(header.num_frames as usize);
	
	for _ in 0..header.num_frames {
		let mut frame = Md2Frame { 
			scale: Vector3 { x: 0.0, y: 0.0, z: 0.0, }, 
			translate: Vector3 { x: 0.0, y: 0.0, z: 0.0, }, 
			name: String::from(""), 
			verts: vec![],
		};
		
		frame.scale.x = reader.read_f32::<LittleEndian>()?;
		frame.scale.y = reader.read_f32::<LittleEndian>()?;
		frame.scale.z = reader.read_f32::<LittleEndian>()?;
		
		frame.translate.x = reader.read_f32::<LittleEndian>()?;
		frame.translate.y = reader.read_f32::<LittleEndian>()?;
		frame.translate.z = reader.read_f32::<LittleEndian>()?;
		
		let mut name_bytes = vec![0 as u8; 16];
		reader.read(&mut name_bytes[..])?;
		frame.name = String::from_utf8(name_bytes).unwrap();
		
		frame.verts = Vec::with_capacity(header.num_vertices as usize);
		
		for _ in 0..header.num_vertices {
			let mut vert = Md2Vertex {
				v: [0; 3],
				normal_index: 0,
			};
			
			reader.read(&mut vert.v[..])?;
			vert.normal_index = reader.read_u8()?;
			
			frame.verts.push(vert);
		}
		
		frames.push(frame);
	};
	
	Ok(frames)
}

fn compute_frame(header: &Md2Header, tris: &Vec<Md2Triangle>, frames: &Vec<Md2Frame>, texcoords: &Vec<Md2Texcoord>) -> (Vec<GLfloat>, Vec<GLfloat>) {
	let frame = &frames[0];
	let mut verts_out = Vec::with_capacity(3 * header.num_vertices as usize);
	let mut texcoords_out = Vec::with_capacity(2 * header.num_vertices as usize);
	
	for tri in tris.iter() {
		for i in 0..3 {
			let vert = &frame.verts[tri.vertex[i] as usize];
			
			verts_out.push(frame.scale.x * vert.v[0] as f32 + frame.translate.x);
			verts_out.push(frame.scale.y * vert.v[1] as f32 + frame.translate.y);
			verts_out.push(frame.scale.z * vert.v[2] as f32 + frame.translate.z);
			
			texcoords_out.push(texcoords[tri.st[i] as usize].s);
			texcoords_out.push(texcoords[tri.st[i] as usize].t);
		}
	};
	
	(verts_out, texcoords_out)
}

// Shader sources
static VS_SRC: &'static str = "#version 150
    in vec3 position;
	in vec2 texcoord;
	
	out vec2 Texcoord;
	
	uniform mat4 trans;
	uniform mat4 proj;
	
    void main() {
		Texcoord = texcoord;
		gl_Position = proj * trans * vec4(position, 1.0);
    }";

static FS_SRC: &'static str = "#version 150
	in vec2 Texcoord;
	
    out vec4 out_color;
	
	uniform sampler2D tex;
	
    void main() {
		out_color = texture(tex, Texcoord);
    }";

pub fn read_md2_model<T: Read + Seek>(mut reader: &mut T, tex: &Image) -> Result<Mesh,()> {
	let header = read_header(&mut reader).unwrap();
	let texcoords = read_texcoords(&mut reader, &header).unwrap();
	let tris = read_triangles(&mut reader, &header).unwrap();
	let frames = read_frames(&mut reader, &header).unwrap();
	
	let (computed_verts, computed_texcoords) = compute_frame(&header, &tris, &frames, &texcoords);
	
	Ok(Mesh::new(VS_SRC, FS_SRC, &computed_verts, &computed_texcoords, tex))
}