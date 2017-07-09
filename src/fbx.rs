use std::string::String;
use std::io::Read;
use std::convert::From;
use std::iter::Iterator;
use fbx_direct::reader::{EventReader, FbxEvent, Error};
use fbx_direct::common::OwnedProperty;
use mesh::{GeometryType, Mesh};
use cgmath::{Vector3, Vector2,Decomposed,Basis3,Deg,Rotation3};
use image;
use std::path::Path;
use gfx::program::Program;
use object::Object;

#[derive(Debug, PartialEq)]
pub enum NodeType {
	Root,
	Definitions,
	Objects,
	Geometry,
	PolygonVertexIndex(GeometryType, Vec<i32>),
	Vertices(Vec<Vector3<f32>>),
	LayerElementUV,
	UV(Vec<Vector2<f32>>),
	UVIndex(Vec<i32>),
	Other(String),
}

use self::NodeType::*;

#[derive(Debug)]
pub struct FbxNode {
    node_type: NodeType,
    properties: Vec<OwnedProperty>,
    children: Vec<FbxNode>,
}

impl FbxNode {
    pub fn get_indices(&self) -> Option<(GeometryType, Vec<i32>)> {
		self.find_node(&|node| {
			match &node.node_type {
				&PolygonVertexIndex(ref geom_type, ref indices) => 
					return Some((*geom_type, indices.clone())),
				_ => None,
			}
		})
    }

    pub fn get_vertices(&self) -> Option<Vec<Vector3<f32>>> {
		let verts = self.find_node(&|node| {
			match &node.node_type {
				&Vertices(ref verts) => return Some(verts.clone()),
				_ => None,
			}
		}).unwrap();
		
		let indices = self.get_indices().unwrap();
		let (_, indices) = indices;
		
		Some(indices.iter().map(|i| verts[*i as usize]).collect())
    }
	
	pub fn get_texcoords(&self) -> Option<Vec<Vector2<f32>>> {
		let uvindexes = self.find_node(&|node| {
			match &node.node_type {
				&UVIndex(ref idx) => Some(idx.clone()),
				_ => None,
			}
		}).unwrap();
		
		let uv = self.find_node(&|node| {
			match &node.node_type {
				&UV(ref uv) => Some(uv.clone()),
				_ => None,
			}
		}).unwrap();
	
		let mut texcoords = Vec::with_capacity(uvindexes.len());
		
		for idx in uvindexes {
			texcoords.push(uv[idx as usize]);
		}
		
		Some(texcoords)
	}
	
    pub fn find_node<T, F>(&self, find_fn: &F) -> Option<T> where F: Fn(&FbxNode) -> Option<T> {
		match find_fn(&self) {
			Some(val) => return Some(val),
			_ => (),
		};
	
        for child in &self.children {
			let cr = child.find_node(find_fn);
			
			if cr.is_some() {
				return cr;
			}
        };
		
        None
    }
}

fn parse_uv(mut properties: Vec<OwnedProperty>) -> FbxNode {
    assert_eq!(properties.len(), 1);

    let property = properties.pop().unwrap();

    let floats = match property {
        OwnedProperty::VecF32(floats) => floats,
        OwnedProperty::VecF64(floats) => floats.iter().map(|f| *f as f32).collect(),
        _ => panic!("Bad property in parse_normals"),
    };

    assert_eq!(floats.len() % 2, 0);

    FbxNode {
        node_type: UV(floats.chunks(2).map(|chunk| Vector2::new(chunk[0], 1.0-chunk[1])).collect()),
        properties: Vec::new(),
        children: Vec::new(),
    }
}

fn parse_uvindex(mut properties: Vec<OwnedProperty>) -> FbxNode {
    assert_eq!(properties.len(), 1);

    let property = properties.pop().unwrap();

    let ints = match property {
        OwnedProperty::VecI32(ints) => ints,
        _ => panic!("Bad property in parse_normals"),
    };

    FbxNode {
        node_type: UVIndex(ints),
        properties: Vec::new(),
        children: Vec::new(),
    }
}

fn parse_vertices(mut properties: Vec<OwnedProperty>) -> FbxNode {
    assert_eq!(properties.len(), 1);

    let property = properties.pop().unwrap();

    let floats = match property {
        OwnedProperty::VecF32(floats) => floats,
        OwnedProperty::VecF64(floats) => floats.iter().map(|f| *f as f32).collect(),
        _ => panic!("Bad property in parse_vertices"),
    };

    assert_eq!(floats.len() % 3, 0);

    FbxNode {
        node_type: Vertices(floats.chunks(3).map(|chunk| Vector3::new(chunk[0], chunk[1], chunk[2])).collect()),
        properties: Vec::new(),
        children: Vec::new(),
    }
}

fn parse_indices(mut properties: Vec<OwnedProperty>) -> FbxNode {
    assert_eq!(properties.len(), 1);

    let property = properties.pop().unwrap();

    let indices = match property {
        OwnedProperty::VecI32(indices) => indices,
        _ => panic!("Bad property in parse_vertices"),
    };
	
	let geom_type = match indices[2] < 0 {
		true => GeometryType::Tris,
		false => GeometryType::Quads,
	};
	
	FbxNode {
		node_type: PolygonVertexIndex(geom_type, indices.iter().cloned().map(|i| if i < 0 { i.abs() - 1 } else { i }).collect()),
		properties: Vec::new(),
		children: Vec::new(),
	}
}

fn convert_node(name: String, properties: Vec<OwnedProperty>) -> FbxNode {
    match name.as_ref() {
        "Vertices" => parse_vertices(properties),
        "PolygonVertexIndex" => parse_indices(properties),
        "Objects" => FbxNode { node_type: Objects, properties: Vec::new(), children: Vec::new() },
        "Geometry" => FbxNode { node_type: Geometry, properties: Vec::new(), children: Vec::new() },
        "Definitions" => FbxNode { node_type: Definitions, properties: Vec::new(), children: Vec::new() },
		"LayerElementUV" => FbxNode { node_type: LayerElementUV, properties, children: Vec::new() },
		"UV" => parse_uv(properties),
		"UVIndex" => parse_uvindex(properties),
        _ => FbxNode { node_type: Other(name), properties, children: Vec::new() },
    }
}

fn read_node(mut root: FbxNode, events: &mut Iterator<Item = Result<FbxEvent, Error>>, keep_others: bool) -> FbxNode {
    loop {
        let event = events.next();

        match event {
            Some(Ok(FbxEvent::StartNode { name, properties })) => {
                let converted = convert_node(name, properties);

                let keep = match &converted.node_type {
                    &Other(_) => keep_others,
                    _ => true,
                };

                let n = read_node(converted, events, keep_others);
                if keep {
                    root.children.push(n);
                }
            },
            None | Some(Ok(FbxEvent::EndNode)) => return root,
			Some(Err(err)) => panic!("Got error {:?}", err),
            _ => (),
        };
    }
}

pub fn read<T: Read>(reader: T) -> FbxNode {
    let fbr = EventReader::new(reader);

    let mut events = fbr.into_iter();

    return read_node(FbxNode { node_type: Root, properties: vec![], children: vec![] }, &mut events, false);
}

impl From<FbxNode> for Object {
    fn from(root: FbxNode) -> Object {
        let vertex_data = root.get_vertices().unwrap();
        let (geom_type, index_data) = root.get_indices().unwrap();
        let texcoord_data = root.get_texcoords().unwrap();
        let image = image::load_image(&Path::new("monkey.png")).unwrap();
        let normals = image::load_image(&Path::new("normals.png")).unwrap();
		let program = Program::from_path(&Path::new("shader.vert"), &Path::new("shader.frag"));
		
		Object {
			mesh: Mesh::new(program, &vertex_data, &normals, &index_data, &texcoord_data, &image, geom_type),
			
			transform: Decomposed::<Vector3<f32>, Basis3<f32>> {
				scale: 1.0,
				rot: Basis3::from_angle_x(Deg(-90.0)),
				disp: Vector3::new(0.0, 0.0, -2.75),
			},
		}
    }
}
