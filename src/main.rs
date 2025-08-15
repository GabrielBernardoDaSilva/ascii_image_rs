use std::{error::Error, path::PathBuf};

use image::{GenericImageView, imageops::FilterType};

const RAMP: &[u8] = b"@%$#*+=-:. ";
const RAMP_2: &[u8] = b"\xE2\x96\x88\xE2\x96\x93\xE2\x96\x92\xE2\x96\x91\x23\x2B\x2D\x2E\x20";
enum ProcessType {
    Gif,
    Image,
}

impl From<bool> for ProcessType {
    fn from(value: bool) -> Self {
        if value {
            return ProcessType::Gif;
        }
        ProcessType::Image
    }
}

struct ProcessConfig<'a> {
    path: &'a PathBuf,
    is_complex: bool,
    is_colorized: bool,
    target_cols: i32,
}

#[inline]
fn luma(rgb: (u8, u8, u8)) -> u8 {
    let (r, g, b) = rgb;
    (0.2126 * r as f32 + 0.7152 * g as f32 + 0.0722 * b as f32)
        .powf(0.95)
        .round() as u8
}

fn process_pixel(r: u8, g: u8, b: u8, complex: bool, is_colorized: bool) -> String {
    let l = luma((r, g, b)) as usize;
    let idx = (l * (RAMP.len() - 1)) / 255;
    let ch = RAMP[idx] as char;
    let r = if is_colorized { r } else { l as u8 };
    let g = if is_colorized { g } else { l as u8 };
    let b = if is_colorized { b } else { l as u8 };

    if !complex {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, ch)
    } else {
        let num_chars = RAMP_2.len() / 3;
        let idx = (l * (num_chars - 1)) / 255;
        let start = idx * 3;
        let end = start + 3;
        format!(
            "\x1b[38;2;{};{};{}m{}\x1b[0m",
            r,
            g,
            b,
            std::str::from_utf8(&RAMP_2[start..end]).unwrap()
        )
    }
}

fn ascii_image(config: &ProcessConfig) -> Result<(), Box<dyn Error>> {
    let file = image::open(config.path)?;
    let (w, h) = file.dimensions();
    let height_compression = 0.5_f32;
    let out_w = config.target_cols.max(1) as u32;
    let out_h = ((h as f32 / w as f32) * out_w as f32 * height_compression)
        .max(1.0)
        .round() as u32;

    let resized = file.resize_exact(out_w, out_h, FilterType::CatmullRom);

    let mut out = String::with_capacity((out_w as usize + 1) * out_h as usize);

    for y in 0..out_h {
        for x in 0..out_w {
            let px = resized.get_pixel(x, y);
            let [r, g, b, _] = px.0;
            let wd = process_pixel(r, g, b, config.is_complex, config.is_colorized);
            out.push_str(&wd);
        }
        out.push('\n');
    }

    print!("\x1b[2J\x1b[H");
    print!("{out}");
    Ok(())
}

fn ascii_gif(config: &ProcessConfig) -> Result<(), Box<dyn Error>> {
    let file = std::fs::File::open(config.path)?;

    let mut decoder = gif::DecodeOptions::new();
    decoder.set_color_output(gif::ColorOutput::RGBA);

    let mut decoder = decoder.read_info(file)?;

    let height_compression = 0.5f32;

    while let Some(frame) = decoder.read_next_frame()? {
        print!("\x1b[2J\x1b[H");
        let w = frame.width as usize;
        let h = frame.height as usize;
        let buf = &frame.buffer;

        let scale_x = w as f32 / config.target_cols as f32;
        let scale_y = scale_x / height_compression;
        let out_h = (h as f32 / scale_y) as usize;

        for oy in 0..out_h {
            let y = (oy as f32 * scale_y) as usize;
            for ox in 0..config.target_cols {
                let x = (ox as f32 * scale_x) as usize;
                let idx = (y * w + x) * 4;
                let r = buf[idx];
                let g = buf[idx + 1];
                let b = buf[idx + 2];
                let wd = process_pixel(r, g, b, config.is_complex, config.is_colorized);
                print!("{}", wd);
            }
            println!();
        }

        std::thread::sleep(std::time::Duration::from_millis(1000 / 30));
    }

    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let args = std::env::args().collect::<Vec<_>>();
    let mut args = args.iter();

    if args
        .clone()
        .by_ref()
        .any(|it| it.contains("-help") || it.contains("--h"))
    {
        println!("Options Availible:");
        println!("\tpath=[filename]");
        println!("\t-infinity or --i: keep it in loop in case that is a gif");
        println!("\t-complex or --cp: use alternative charset");
        println!();
        println!("\t\tComplex {}", std::str::from_utf8(RAMP_2).unwrap());
        println!("\t\tSimple {}", std::str::from_utf8(RAMP).unwrap());
        println!();
        println!("\t-color or --cl: output with color");
        return Ok(());
    }

    let p = args.by_ref().find(|it| it.contains("path="));
    let m = if let Some(opt) = p {
        ProcessType::from(opt.contains(".gif"))
    } else {
        ProcessType::from(false)
    };

    let target_cols = if let Some((terminal_size::Width(w), terminal_size::Height(_))) =
        terminal_size::terminal_size()
    {
        w as i32
    } else {
        80
    };

    if let Some(path) = p {
        let path = std::path::PathBuf::from(path.replace("path=", ""));
        let is_infinity = args
            .clone()
            .by_ref()
            .any(|it| it.contains("-infinity") || it.contains("--i"));
        let is_complex = args
            .clone()
            .by_ref()
            .any(|it| it.contains("-complex") || it.contains("--cp"));
        let is_colorized = args
            .by_ref()
            .any(|it| it.contains("-color") || it.contains("--cl"));

        let config = ProcessConfig {
            path: &path,
            is_complex,
            is_colorized,
            target_cols,
        };
        loop {
            match m {
                ProcessType::Gif => ascii_gif(&config)?,
                ProcessType::Image => {
                    ascii_image(&config)?;
                    break;
                }
            };
            if !is_infinity {
                break;
            }
        }
    } else {
        panic!("File not founded!")
    }

    Ok(())
}
