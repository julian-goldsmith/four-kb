use png;
use std::fs::File;
use std::path::Path;
use std::io;

#[derive(Debug)]
pub struct Image {
	pub width: u32,
	pub height: u32,
	pub data: Vec<u8>,
}

/// Load the image using `png`
pub fn load_image(path: &Path) -> io::Result<Image> {
    let decoder = png::Decoder::new(try!(File::open(path)));
    let (info, mut reader) = try!(decoder.read_info());
    let mut img_data = vec![0; info.buffer_size()];
    try!(reader.next_frame(&mut img_data));
    
    Ok(Image {
		width: info.width,
		height: info.height,
		data: img_data,
	})
}