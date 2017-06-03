use std::string::String;
use std::io::Read;
use fbx_direct::reader::{EventReader, FbxEvent, Error};
use fbx_direct::common::OwnedProperty;
use mesh::Mesh;

#[derive(Debug)]
pub struct FbxNode {
    name: String,
    properties: Vec<OwnedProperty>,
    children: Vec<FbxNode>,
}

fn convert_node(name: String, properties: Vec<OwnedProperty>) -> FbxNode {
    FbxNode {
        name,
        properties,
        children: vec![],
    }
}

fn read_node(mut root: FbxNode, events: &mut Iterator<Item = Result<FbxEvent, Error>>) -> FbxNode {
    loop {
        let event = events.next();

        match event {
            Some(Ok(FbxEvent::StartNode { name, properties })) => {
                let n = read_node(convert_node(name, properties), events);
                root.children.push(n);
            },
            None | Some(Ok(FbxEvent::EndNode)) => return root,
            _ => (),
        };
    }
}

pub fn read<T: Read>(reader: T) -> FbxNode {
    let fbr = EventReader::new(reader);

    let mut events = fbr.into_iter();

    return read_node(FbxNode { name: String::from("root"), properties: vec![], children: vec![] }, &mut events);
}
