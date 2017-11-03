use std::io::Read;
use std::mem;
use model;
use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use cgmath::{Matrix4, Vector2, Vector3,Decomposed,Basis3,Deg,Rotation3, Matrix};

fn read_bool(reader: &mut Read) -> bool {
    let mut buf = [0];
    reader.read(&mut buf);

    buf[0] == 1
}

fn read_string(reader: &mut Read) -> String {
    let string_len = reader.read_u16::<BigEndian>().unwrap() as usize;

    let mut string_buf = Vec::with_capacity(string_len);

    unsafe {
        string_buf.set_len(string_len);
    };

    reader.read(&mut string_buf).expect("Read failed");

    String::from_utf8(string_buf).unwrap()
}

fn read_transform(reader: &mut Read) -> Matrix4<f32> {
    let mut buf = Vec::with_capacity(mem::size_of::<Matrix4<f32>>());

    unsafe {
        buf.set_len(mem::size_of::<Matrix4<f32>>());
    };

    reader.read(&mut buf);

    let mut matrix: Matrix4<f32> = unsafe { mem::uninitialized() };
    unsafe {
        BigEndian::read_f32_into_unchecked(&buf, 
                                           &mut matrix.as_mut() as &mut [f32; 16]);
    }

    matrix
}

fn read_texture(reader: &mut Read) -> Option<model::Texture> {
    if !read_bool(reader) {
        return None;
    };

    let width = reader.read_u16::<BigEndian>().unwrap();
    let height = reader.read_u16::<BigEndian>().unwrap();
    let num_pixels = width as usize * height as usize;
    let size = Vector2::<u16>::new(width, height);
    
    let mut pixels: Vec<Vector3<u8>> = Vec::with_capacity(num_pixels);

    for _ in 0..pixels.capacity() {
        let mut pixel: Vector3<u8> = unsafe { mem::uninitialized() };

        reader.read(&mut pixel[0..3]).expect("Pixel read failed");

        pixels.push(pixel);
    };

    Some(model::Texture {
        size,
        pixels: pixels.into_boxed_slice(),
    })
}

fn read_material(reader: &mut Read) -> model::Material {
    let name = read_string(reader);

    // shaders go here
    let shader_vertex = read_string(reader);
    let shader_fragment = read_string(reader);
    
    let normals = read_texture(reader);
    let diffuse = read_texture(reader);
    let specular = read_texture(reader);

    model::Material {
        name,
        shader_vertex,
        shader_fragment,
        normals,
        diffuse,
        specular,
    }
}

fn read_vertex(reader: &mut Read) -> Vector3<f32> {
    let mut vertex: Vector3<f32> = unsafe { mem::zeroed() };

    unsafe {
        reader.read_f32_into_unchecked::<BigEndian>(&mut vertex[0..3]).unwrap();
    };

    vertex
}

fn read_texcoord(reader: &mut Read) -> Vector2<f32> {
    let mut texcoord: Vector2<f32> = unsafe { mem::zeroed() };

    unsafe {
        reader.read_f32_into_unchecked::<BigEndian>(&mut texcoord[0..2]).
            unwrap();
    };

    texcoord
}

fn read_index(reader: &mut Read) -> u32 {
    reader.read_u32::<BigEndian>().unwrap()
}

fn read_and_box<T, F>(reader: &mut Read, read_fn: F) -> Box<[T]> 
    where F: Fn(&mut Read) -> T {

    let num_items = reader.read_u32::<BigEndian>().unwrap() as usize;

    let mut items = Vec::with_capacity(num_items);

    for _ in 0..num_items {
        let item = read_fn(reader);

        items.push(item);
    };

    items.into_boxed_slice()
}

pub fn load_model(reader: &mut Read) -> model::Model {
    let name = String::from("");//read_string(reader);
    let transform = read_transform(reader);
    let materials = Box::new([]);//read_and_box(reader, read_material);
    let indices = read_and_box(reader, read_index);
    let vertices = read_and_box(reader, read_vertex);
    let texcoords = read_and_box(reader, read_texcoord);

    model::Model {
        name,
        transform,
        materials,
        indices,
        vertices,
        texcoords,
    }
}
