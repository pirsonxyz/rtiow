use clap::Parser;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

mod args;
mod color;
mod ray;
mod vec3;
use args::Args;
use color::{write_color, Color};
fn main() {
    let args = Args::parse();
    let image_width = args.width;
    let image_height = args.height;

    let head = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("image.ppm")
        .unwrap();
    file.write_all(head.as_bytes()).unwrap();
    for i in 0..image_width {
        for j in 0..image_height {
            println!("\rScanlines remaining: {} ", (image_height - j));
            io::stdout().flush().unwrap();
            /*
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            let colors = format!("{} {} {}\n", ir, ig, ib);
            */
            let pixel_color = Color::with_contents(
                (i as f32 / (image_width - 1) as f32),
                (j as f32 / (image_height - 1) as f32),
                0.0,
            );
            let colors = write_color(&pixel_color);
            file.write_all(colors.as_bytes()).unwrap();
        }
    }
    println!("Done!");
}
