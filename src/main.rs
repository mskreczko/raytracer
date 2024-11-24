use crate::ppm::{save_ppm_image, PPMImage, PPMPixel};
use crate::sphere::Sphere;

mod ppm;
mod ray;
mod vector;
mod sphere;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400.0;

    let mut image_height = image_width / aspect_ratio;
    image_height = if image_height < 1f64 {
        1f64
    } else {
        image_height
    };

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width / image_height);
    let camera = vector::Vec3::zero();

    let viewport_u = vector::Vec3::new(viewport_width as f32, 0.0, 0.0);
    let viewport_v = vector::Vec3::new(0.0, -viewport_height as f32, 0.0);

    let pixel_delta_u = viewport_u / image_width as f32;
    let pixel_delta_v = viewport_v / image_height as f32;

    let viewport_upper_left = ((camera - vector::Vec3::new(0.0, 0.0, focal_length)) - viewport_u / 2.0) - viewport_v / 2.0;

    let pixel_loc = viewport_upper_left + (pixel_delta_u + pixel_delta_v) *  0.5;

    let mut img: Vec<Vec<PPMPixel>> = vec![];
    let sphere = Sphere{
        center: vector::Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5
    };
    for j in 0..(image_height as i32) {
        let mut row = vec![];
        for i in 0..(image_width as i32) {
            let pixel_center = pixel_loc + pixel_delta_u * i as f32 + pixel_delta_v * j as f32;
            let ray_direction = pixel_center - camera;
            let r = ray::Ray{origin: camera, dir: ray_direction};

            let pixel_color = write_color(r.color(sphere));
            row.push(PPMPixel{r: pixel_color.r, g: pixel_color.g, b: pixel_color.b});
        }
        img.push(row)
    }

    save_ppm_image(&PPMImage{
        width: image_width as u32,
        height: image_height as u32,
        data: img
    }, "result.ppm").unwrap()
}

fn write_color(color: vector::Vec3) -> PPMPixel {
    PPMPixel{
        r: (255.999 * (color.x())) as u32,
        g: (255.999 * (color.y())) as u32,
        b: (255.999 * (color.z())) as u32,
    }
}