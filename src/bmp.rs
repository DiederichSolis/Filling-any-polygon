use std::fs::File;
use std::io::{self, Write, BufWriter};

const BMP_HEADER_SIZE: usize = 54;
const BMP_PIXEL_OFFSET: usize = 54;
const BMP_BITS_PER_PIXEL: usize = 24; // Cambiado a 24 bits por píxel

pub fn write_bmp_file(
    file_path: &str,
    buffer: &[u32],
    width: usize,
    height: usize
) -> io::Result<()> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);

    write_bmp_header(&mut writer, width, height)?;
    write_pixel_data(&mut writer, buffer, width, height)?;

    Ok(())
}

fn write_bmp_header(
    writer: &mut BufWriter<File>,
    width: usize,
    height: usize
) -> io::Result<()> {
    let file_size = BMP_HEADER_SIZE + (width * height * (BMP_BITS_PER_PIXEL / 8)) as usize;
    let reserved: u32 = 0;
    let pixel_data_offset: u32 = BMP_PIXEL_OFFSET as u32;
    let header_size: u32 = 40;
    let planes: u16 = 1;
    let bits_per_pixel: u16 = BMP_BITS_PER_PIXEL as u16;
    let compression: u32 = 0;
    let image_size: u32 = (width * height * (BMP_BITS_PER_PIXEL / 8)) as u32;
    let x_pixels_per_meter: u32 = 0;
    let y_pixels_per_meter: u32 = 0;
    let total_colors: u32 = 0;
    let important_colors: u32 = 0;

    writer.write_all(&[
        0x42, 0x4D, // Signature 'BM'
        (file_size & 0xFF) as u8,
        ((file_size >> 8) & 0xFF) as u8,
        ((file_size >> 16) & 0xFF) as u8,
        ((file_size >> 24) & 0xFF) as u8,
        (reserved & 0xFF) as u8,
        ((reserved >> 8) & 0xFF) as u8,
        ((reserved >> 16) & 0xFF) as u8,
        ((reserved >> 24) & 0xFF) as u8,
        (pixel_data_offset & 0xFF) as u8,
        ((pixel_data_offset >> 8) & 0xFF) as u8,
        ((pixel_data_offset >> 16) & 0xFF) as u8,
        ((pixel_data_offset >> 24) & 0xFF) as u8,
        (header_size & 0xFF) as u8,
        ((header_size >> 8) & 0xFF) as u8,
        ((header_size >> 16) & 0xFF) as u8,
        ((header_size >> 24) & 0xFF) as u8,
        (width & 0xFF) as u8,
        ((width >> 8) & 0xFF) as u8,
        ((width >> 16) & 0xFF) as u8,
        ((width >> 24) & 0xFF) as u8,
        (height & 0xFF) as u8,
        ((height >> 8) & 0xFF) as u8,
        ((height >> 16) & 0xFF) as u8,
        ((height >> 24) & 0xFF) as u8,
        (planes & 0xFF) as u8,
        ((planes >> 8) & 0xFF) as u8,
        (bits_per_pixel & 0xFF) as u8,
        ((bits_per_pixel >> 8) & 0xFF) as u8,
        (compression & 0xFF) as u8,
        ((compression >> 8) & 0xFF) as u8,
        ((compression >> 16) & 0xFF) as u8,
        ((compression >> 24) & 0xFF) as u8,
        (image_size & 0xFF) as u8,
        ((image_size >> 8) & 0xFF) as u8,
        ((image_size >> 16) & 0xFF) as u8,
        ((image_size >> 24) & 0xFF) as u8,
        (x_pixels_per_meter & 0xFF) as u8,
        ((x_pixels_per_meter >> 8) & 0xFF) as u8,
        ((x_pixels_per_meter >> 16) & 0xFF) as u8,
        ((x_pixels_per_meter >> 24) & 0xFF) as u8,
        (y_pixels_per_meter & 0xFF) as u8,
        ((y_pixels_per_meter >> 8) & 0xFF) as u8,
        ((y_pixels_per_meter >> 16) & 0xFF) as u8,
        ((y_pixels_per_meter >> 24) & 0xFF) as u8,
        (total_colors & 0xFF) as u8,
        ((total_colors >> 8) & 0xFF) as u8,
        ((total_colors >> 16) & 0xFF) as u8,
        ((total_colors >> 24) & 0xFF) as u8,
        (important_colors & 0xFF) as u8,
        ((important_colors >> 8) & 0xFF) as u8,
        ((important_colors >> 16) & 0xFF) as u8,
        ((important_colors >> 24) & 0xFF) as u8,
    ])?;

    Ok(())
}

fn write_pixel_data(
    writer: &mut BufWriter<File>,
    buffer: &[u32],
    width: usize,
    height: usize
) -> io::Result<()> {
    for y in (0..height).rev() {
        for x in 0..width {
            let pixel = buffer[y * width + x];
            let b = (pixel & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let r = ((pixel >> 16) & 0xFF) as u8;
            writer.write_all(&[b, g, r])?;
        }
    }
    Ok(())
}
