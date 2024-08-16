use clap::Parser;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};
use vec3::{Point, Vec3};

mod args;
mod color;
mod ray;
mod vec3;
use args::Args;
use color::{write_color, Color};
use ray::Ray;
fn ray_color(r: &Ray) -> Color {
    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y()) + 1.0;
    (1.0 - a) * Color::with_contents(1.0, 1.0, 1.0) + a * Color::with_contents(0.5, 0.7, 1.0)
}
fn main() {
    let args = Args::parse();
    let image_width = args.width;
    let aspect_ratio = 16.0 / 9.0;
    let mut image_height = (image_width as f32 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };
    // Camera
    let focal_length = args.focal_length;
    let viewport_height = args.viewport_height;
    let viewport_width = viewport_height * (image_width / image_height) as f32;
    let camera_center = Point::new();

    let viewport_u = Vec3::with_contents(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::with_contents(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = camera_center
        - Vec3::with_contents(0.0, 0.0, focal_length)
        - viewport_u / 2.0
        - viewport_v / 2.0;
    let pixel100_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

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
            let pixel_center =
                pixel100_loc + (i as f32 * pixel_delta_u) + (j as f32 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(pixel_center, ray_direction);

            /*
            let pixel_color = Color::with_contents(
                (i as f32 / (image_width - 1) as f32),
                (j as f32 / (image_height - 1) as f32),
                0.0,
            );
            */
            let pixel_color = ray_color(&r);
            let colors = write_color(&pixel_color);
            println!("{}", colors);
            file.write_all(colors.as_bytes()).unwrap();
        }
    }
    println!("Done!");
}
