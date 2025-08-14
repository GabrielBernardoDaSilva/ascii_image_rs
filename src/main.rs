use std::error::Error;

use image::{GenericImageView, Rgba, imageops::FilterType};

fn luma(rgb: image::Rgba<u8>) -> u8 {
    let [r, g, b, _] = rgb.0;
    (256.0 * (r as f32 / 256.0) * 0.2126 + (g as f32 / 256.0) * 0.7152 + (b as f32 / 256.0) * 0.722)
        as u8
}
const RAMP: &[u8] = b"@%#*+=-:. ";
fn main() -> Result<(), Box<dyn Error>> {
    let cargo_env = env!("CARGO_MANIFEST_DIR");
    let path = std::path::PathBuf::from(cargo_env).join("cbp.jpeg");
    let file = image::open(path).unwrap();

    let (w, h) = file.dimensions();

    let target_cols = 120usize;
    let height_compression = 0.5_f32;

    let out_w = target_cols.max(1) as u32;
    let out_h = ((h as f32 / w as f32) * out_w as f32 * height_compression)
        .max(1.0)
        .round() as u32;

    let resized = file.resize_exact(out_w, out_h, FilterType::CatmullRom);

    let mut out = String::with_capacity((out_w as usize + 1) * out_h as usize);

    for y in 0..out_h {
        for x in 0..out_w {
            let px = resized.get_pixel(x, y);
            let l = luma(px) as usize;
            let idx = (l * (RAMP.len() - 1)) / 255;
            let ch = RAMP[idx] as char;

            out.push(ch);
        }
        out.push('\n');
    }

    print!("{out}");
    Ok(())
}
