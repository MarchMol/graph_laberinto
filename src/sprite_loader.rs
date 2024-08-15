use std::fs::File;
use std::io::BufReader;
use crate::color::Color;
use bmp::{from_reader, Pixel};
use crate::framebuffer::Framebuffer;

pub struct Sprite{
    pub buffer: Vec<Color>,
    pub height: usize,
    pub width: usize,
}

impl Sprite{
    pub fn render_screen(framebuffer: &mut Framebuffer, sprite: &Sprite) {
        for x in 0..framebuffer.width - 1 {
            // Transforming x to sprite coordinates
            let trans_x = ((x as f32) / framebuffer.width as f32) * (sprite.width) as f32;
            let current_line = sprite.get_line(trans_x as usize);
            for y in 0..framebuffer.height - 1 {
                // Transforming y to sprite coordinates
                let trans_y = ((y as f32) / framebuffer.height as f32)* sprite.height as f32;
                let color = Color::to_hex(&current_line[trans_y as usize]);
                framebuffer.set_current_color(color);
                framebuffer.point(x as usize, y);
            }
        }
    }
    
    pub fn get_line(&self, x:usize)->Vec<Color>{
        let mut line_buffer: Vec<Color> = Vec::new();
        for y in 0..self.height{
            line_buffer.push(self.buffer[x+(y*self.width)]);
        }
        return line_buffer;
    }

    pub fn new(file_path: &str) -> Self{
        let mut buffer: Vec<Color>  = Vec::new();
        let mut height = 0;
        let mut width = 0;
        match read_bmp_to_framebuffer(file_path) {
            Ok(sprite) => {
                buffer=sprite;
            }
            Err(e) => eprintln!("Failed to read BMP file: {}", e),
        }
        match get_dimentions(file_path) {
            Ok(dim) => {
                height = dim.0;
                width = dim.1;
            }
            Err(e) => eprintln!("Failed to read BMP file: {}", e),
        }

        Sprite{
            buffer,
            height,
            width,
        }
    }
}
pub fn read_bmp_to_framebuffer(file_path: &str) -> Result<Vec<Color>, String>{
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);

    // Load the BMP image

    let bmp_image = from_reader(&mut reader).map_err(|e| e.to_string())?;

    let mut framebuffer: Vec<Color> = Vec::new();

    for (x, y) in bmp_image.coordinates() {
        let pixel: Pixel = bmp_image.get_pixel(x, y);
        let color_value: u32 = ((pixel.r as u32) << 16) | ((pixel.g as u32) << 8) | (pixel.b as u32);
        framebuffer.push(Color::from_hex(color_value));
    }
    Ok(framebuffer)
}

pub fn get_dimentions(file_path: &str) -> Result<(usize,usize), String>{
    let file = File::open(file_path).map_err(|e| e.to_string())?;
    let mut reader = BufReader::new(file);

    // Load the BMP image

    let bmp_image = from_reader(&mut reader).map_err(|e| e.to_string())?;
    let height = bmp_image.get_height();
    let width = bmp_image.get_width();
    Ok((height as usize,width as usize))
}

