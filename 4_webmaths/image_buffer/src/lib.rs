use packed_struct::prelude::*;
use vector_math::{*};

use std::vec;
use std::fs::File;
use std::io::prelude::*;

pub type Color = Vec3;

pub struct ImageBuffer<'buffer> {
    pub width: usize,
    pub height: usize,
    pub first_line: usize,
    data: &'buffer mut [Color]
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

impl<'buffer> ImageBuffer<'buffer> {
    pub fn alloc_data_buf(width: usize, height: usize) -> Vec<Color> {
        let default_col = Color::new(0.0, 0.5, 0.0);
        return vec![default_col; width * height];
    }

    pub fn new(width: usize, height: usize, data_buf: &'buffer mut Vec<Color>) -> ImageBuffer<'buffer> {
        return ImageBuffer {
            width: width,
            height: height,
            first_line: 0,
            data: &mut data_buf[..]
        };
    }

    pub fn get_split_buffers(&mut self, splits: usize) -> Vec<ImageBuffer> {
        let mut buffer_vec = Vec::new();
        let slice_height = self.height / splits;
        let mut split_start = 0;
        let mut slice_size = slice_height * self.width;
        for (slice_idx, data_slice) in self.data.chunks_mut(slice_size).enumerate() {
            if slice_idx == splits - 1 {
                slice_size = (self.height - split_start) * self.width;
            }
            let sliced_buffer = ImageBuffer {
                width: self.width,
                height: slice_size / self.width,
                first_line: split_start,
                data: data_slice
            };
            buffer_vec.push(sliced_buffer);
            split_start += slice_height;
        }
        return buffer_vec;
    }

    #[inline(always)]
    fn pixel_index(&self, x: usize, y: usize) -> usize{
        return y * self.width + x;
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

    pub fn tonemap_aces(&mut self, exposure: Scalar) {
        let a: Scalar = 2.51;
        let b: Scalar = 0.03;
        let c: Scalar = 2.43;
        let d: Scalar = 0.59;
        let e: Scalar = 0.14;
        for pixel_idx in 0..self.data.len() {
            let pixel = self.data[pixel_idx] * exposure;
            let mut pixel_mapped = (pixel * (a * pixel + b)) / (pixel * (c * pixel + d) + e);
            pixel_mapped.set_x(pixel_mapped.x().clamp(0.0, 1.0));
            pixel_mapped.set_y(pixel_mapped.y().clamp(0.0, 1.0));
            pixel_mapped.set_z(pixel_mapped.z().clamp(0.0, 1.0));
            self.data[pixel_idx] = pixel_mapped;
        }
    }

    pub fn get_rgb8_buffer(&self) -> Vec<u8> {
        let mut pixel_stream = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.data[y * self.width + x];
                pixel_stream.push((pixel.b() * 255.0) as u8);
                pixel_stream.push((pixel.g() * 255.0) as u8);
                pixel_stream.push((pixel.r() * 255.0) as u8);
            }
        }
        return pixel_stream;
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
        for y in 0..self.height {
            for x in 0..self.width {
                let pixel = self.data[(self.height - y - 1) * self.width + x];
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
        }
        f.write_all(&pixel_stream).expect("Write error");
    }
}
