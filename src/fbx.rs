use std::string::String;
use std::io::Read;
use std::convert::From;
use fbx_direct::reader::{EventReader, FbxEvent, Error};
use fbx_direct::common::OwnedProperty;
use mesh::{GeometryType, Mesh};
use cgmath::{Vector3, Vector2};
use image;
use std::path::Path;

#[derive(Debug, PartialEq)]
pub enum NodeType {
    Root,
    Definitions,
    Objects,
    Geometry,
    PolygonVertexIndex(GeometryType, Vec<i32>),
    Vertices(Vec<Vector3<f32>>),
    LayerElementNormal,
    Normals(Vec<Vector3<f32>>),
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
    pub fn print(&self, depth: u32) {
        let spaces = (0..depth).fold(String::from(""), |acc, _| acc + "  ");

        println!("{} {:?} {:?}", &spaces, &self.node_type, &self.properties);

        for child in &self.children {
            child.print(depth + 1);
        }
    }

    pub fn get_indices(&self) -> Option<(GeometryType, Vec<i32>)> {
        for child in &self.children {
            match &child.node_type {
                &PolygonVertexIndex(ref geom_type, ref indices) => 
                    return Some((*geom_type, indices.clone())),
                _ => match child.get_indices() {
                    Some(indices) => return Some(indices),
                    _ => ()
                },
            }
        };
        None
    }

    pub fn get_vertices(&self) -> Option<Vec<Vector3<f32>>> {
        for child in &self.children {
            match &child.node_type {
                &Vertices(ref verts) => return Some(verts.clone()),
                _ => match child.get_vertices() {
                    Some(verts) => return Some(verts),
                    _ => (),
                },
            }
        };
        None
    }
	
	pub fn get_normals(&self) -> Option<Vec<Vector3<f32>>> {
        for child in &self.children {
            match &child.node_type {
                &Normals(ref verts) => return Some(verts.clone()),
                _ => match child.get_vertices() {
                    Some(verts) => return Some(verts),
                    _ => (),
                },
            }
        };
        None
	}
	
	pub fn get_texcoords(&self) -> Option<Vec<Vector2<f32>>> {
		Some(Vec::<Vector2<f32>>::new())
	}
}

fn parse_normals(mut properties: Vec<OwnedProperty>) -> FbxNode {
    assert_eq!(properties.len(), 1);

    let property = properties.pop().unwrap();

    let floats = match property {
        OwnedProperty::VecF32(floats) => floats,
        OwnedProperty::VecF64(floats) => floats.iter().map(|f| *f as f32).collect(),
        _ => panic!("Bad property in parse_normals"),
    };

    assert_eq!(floats.len() % 3, 0);

    FbxNode {
        node_type: Normals(floats.chunks(3).map(|chunk| Vector3::new(chunk[0], chunk[1], chunk[2])).collect()),
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

fn parse_other(name: String, properties: Vec<OwnedProperty>) -> FbxNode {
    FbxNode {
        node_type: Other(name),
        properties,
        children: Vec::new(),
    }
}

fn convert_node(name: String, properties: Vec<OwnedProperty>) -> FbxNode {
    match name.as_ref() {
        "Vertices" => parse_vertices(properties),
        "Normals" => parse_normals(properties),
        "PolygonVertexIndex" => parse_indices(properties),
        "Objects" => FbxNode { node_type: Objects, properties: Vec::new(), children: Vec::new() },
        "Geometry" => FbxNode { node_type: Geometry, properties: Vec::new(), children: Vec::new() },
        "Definitions" => FbxNode { node_type: Definitions, properties: Vec::new(), children: Vec::new() },
        "LayerElementNormal" => FbxNode { node_type: LayerElementNormal, properties: Vec::new(), children: Vec::new() },
        _ => parse_other(name, properties),
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
            _ => (),
        };
    }
}

pub fn read<T: Read>(reader: T) -> FbxNode {
    let fbr = EventReader::new(reader);

    let mut events = fbr.into_iter();

    return read_node(FbxNode { node_type: Root, properties: vec![], children: vec![] }, &mut events, true);
}

static VS_SRC: &'static str = "#version 150
    in vec3 position;
    in vec3 normal;
	in vec2 texcoord;
	
	out vec2 Texcoord;
	out vec3 Position_worldspace;
	out vec3 EyeDirection_cameraspace;
	out vec3 LightDirection_cameraspace;
	out vec3 Normal_cameraspace;
	out float dist;
	
	uniform mat4 trans;
	uniform mat4 proj;
	uniform mat4 view;
	
    void main() {
		mat4 mvp = proj * trans * view;
		
		vec3 LightPosition_worldspace = vec3(0, 0, 0);
		
		Position_worldspace = (trans * vec4(position, 1)).xyz;
		
		vec3 vertexPosition_cameraspace = (view * trans * vec4(position, 1)).xyz;
		EyeDirection_cameraspace = normalize(vec3(0, 0, 0) - vertexPosition_cameraspace);
		
		vec3 LightPosition_cameraspace = (view * vec4(LightPosition_worldspace, 1)).xyz;
		LightDirection_cameraspace = normalize(LightPosition_cameraspace + EyeDirection_cameraspace);
		
		Normal_cameraspace = normalize((view * trans * vec4(normal, 0)).xyz);
		
		dist = distance(Position_worldspace, LightPosition_worldspace);
		
		Texcoord = texcoord;
		gl_Position = mvp * vec4(position, 1.0);
    }";

static FS_SRC: &'static str = "#version 150
	in vec2 Texcoord;
	in vec3 Position_worldspace;
	in vec3 EyeDirection_cameraspace;
	in vec3 LightDirection_cameraspace;
	in vec3 Normal_cameraspace;
	in float dist;
	
    out vec4 out_color;
	
	uniform sampler2D tex;
	
    void main() {
		float cosTheta = clamp(dot(Normal_cameraspace, LightDirection_cameraspace), 0, 1);
		vec4 mat_color = vec4(1.0, 1.0, 1.0, 1.0);
		vec4 light_color = vec4(0.6, 0.6, 0.6, 1.0);
		vec4 ambient_color = vec4(0.1, 0.1, 0.1, 0.1);
		
		vec3 reverse_normal = reflect(-LightDirection_cameraspace, Normal_cameraspace);
		float cosAlpha = clamp(dot(EyeDirection_cameraspace, reverse_normal), 0, 1);
		
		//texture(tex, Texcoord);
		out_color = 
			mat_color * ambient_color + 
			mat_color * light_color * cosTheta +
			mat_color * light_color * pow(cosAlpha, 8) / (dist * dist);
    }";

impl From<FbxNode> for Mesh {
    fn from(root: FbxNode) -> Mesh {
        let vertex_data = root.get_vertices().unwrap();
		let normal_data = root.get_normals().unwrap();
        let (geom_type, index_data) = root.get_indices().unwrap();
        let texcoord_data = root.get_texcoords().unwrap();
        let image = image::load_image(&Path::new("monkey.png")).unwrap();
		
        Mesh::new(VS_SRC, FS_SRC, &vertex_data, &normal_data, &index_data, &texcoord_data, &image, geom_type)
    }
}
