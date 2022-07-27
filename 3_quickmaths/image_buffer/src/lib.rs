use packed_struct::prelude::*;
use vector_math::{*};

use std::vec;
use std::fs::File;
use std::io::prelude::*;

pub type Color = Vec3;

pub struct ImageBuffer {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

#[derive(PackedStruct)]
#[packed_struct(endian="lsb")]
pub struct BMPHeader {
    file_type: [u8;2], // "BM"
	file_size: u32,
	reserved_1: u16, // 0
	reserved_2: u16, // 0
	pixel_offset: u32, // 54
	header_size: u32, // 40
	x_size: u32,
	y_size: u32,
	planes: u16, // 1
	bpp: u16, // 24
	compression: u32, // 0
	image_size: u32, // 0
	x_ppm: u32, // 0
	y_ppm: u32, // 0
	used_colors: u32, // 0
	important_colors: u32, // 0
}

impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> ImageBuffer {
        let default_col = Color::new(0.0, 0.5, 0.0);
        return ImageBuffer {
            width: width,
            height: height,
            data: vec![default_col; width * height]
        }
    }

    #[inline(always)]
    fn pixel_index(&self, x: usize, y: usize) -> usize{
        return (self.height - y - 1) * self.width + x;
    }

    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, c: Color) {
        let index = self.pixel_index(x, y);
        self.data[index] = c;
    }

    #[inline]
    pub fn pixel(&self, x: usize, y: usize) -> Color {
        return self.data[self.pixel_index(x, y)];
    }

    pub fn write_bmp(&self, path: &str) {
        let header = BMPHeader{
            file_type: [66, 77],
            file_size: (self.width * self.height * 3 + 54) as u32,
            reserved_1: 0,
            reserved_2: 0,
            pixel_offset: 54,
            header_size: 40,
            x_size: self.width as u32,
            y_size: self.height as u32,
            planes: 1,
            bpp: 24,
            compression: 0,
            image_size: 0,
            x_ppm: 0,
            y_ppm: 0,
            used_colors: 0,
            important_colors: 0,
        };
        let header_packed = header.pack().expect("Header packing error");
        let mut f = File::create(path).expect("Could not open file");
        f.write_all(&header_packed).expect("Write error");

        let mut line_pos = 0;
        let line_max = self.width * 3;

        let mut pixel_stream = Vec::new();
        for pixel in &self.data {
            pixel_stream.push((pixel.b() * 255.0) as u8);
            pixel_stream.push((pixel.g() * 255.0) as u8);
            pixel_stream.push((pixel.r() * 255.0) as u8);

            line_pos += 3;
            if line_pos == line_max {
                while line_pos < line_max + line_max % 4 {
                    pixel_stream.push(0 as u8);
                }
                line_pos = 0;
            }
        }
        f.write_all(&pixel_stream).expect("Write error");
    }
}
