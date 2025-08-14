use image::{GenericImageView, Rgba};

fn lumen(rgb: image::Rgba<u8>) -> u8 {
    let [r, g, b, _] = rgb.0;
    (256.0 * (r as f32 / 256.0) * 0.2126 + (g as f32 / 256.0) * 0.7152 + (b as f32 / 256.0) * 0.722)
        as u8
}

fn main() {
    let cargo_env = env!("CARGO_MANIFEST_DIR");
    let path = std::path::PathBuf::from(cargo_env).join("cbp.jpeg");
    let file = image::open(path).unwrap();

    let (w, h) = file.dimensions();
    let mut new_img = image::ImageBuffer::new(w, h);

    file.pixels().for_each(|(x, y, rgb)| {
        let l = lumen(rgb);

        new_img.put_pixel(x, y, Rgba([l, l, l, 255]));
    });

    new_img
        .save_with_format("./test.png", image::ImageFormat::Png)
        .unwrap();
}
