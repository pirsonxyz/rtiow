use std::{fs::File, io::Write};

mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    let head = format!("P3\n{} {}\n255\n", image_width, image_height);
    let mut file = File::open("image.ppm").unwrap_or_else(|_| File::create("image.ppm").unwrap());
    file.write_all(head.as_bytes()).unwrap();
    for j in 0..image_width {
        for i in 0..image_height {
            let r = i as f32 / (image_width - 1) as f32;
            let g = j as f32 / (image_height - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;
            let colors = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(colors.as_bytes()).unwrap();
        }
    }
}
