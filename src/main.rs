use std::env;
use std::path::Path;
use image::io::Reader as ImageReader;
use image::ImageFormat;

fn imgconv(fin: &str, fout: &str) {
    let img = ImageReader::open(fin).unwrap().decode().unwrap();

    let fmt = match Path::new(fout).extension().unwrap().to_str().unwrap() {
        "png" => ImageFormat::Png,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "gif" => ImageFormat::Gif,
        "bmp" => ImageFormat::Bmp,
        "tif" | "tiff" => ImageFormat::Tiff,
        "webp" => ImageFormat::WebP,
        _ => {
            eprintln!("Unsupported output file format");
            return
        }
    };

    img.save_with_format(fout, fmt).unwrap();
    println!("Converted: {} to {}", fin, fout);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file> <output_file>", args[0]);
        return;
    }

    let fin = &args[1];
    let fout = &args[2];

    let extin = Path::new(fin).extension().unwrap().to_str().unwrap();
    let extout = Path::new(fout).extension().unwrap().to_str().unwrap();

    let fmts = vec!["png", "jpg", "jpeg", "gif", "bmp", "tif", "tiff", "webp"];

    if !fmts.contains(&extin) {
        eprintln!("Error: The input file must have a supported extension ({:?})", fmts);
        return;
    }

    if !fmts.contains(&extout) {
        eprintln!("Error: The output file must have a supported extension ({:?})", fmts);
        return;
    }

    imgconv(fin, fout);
}


