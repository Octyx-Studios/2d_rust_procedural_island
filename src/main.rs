use image::{ImageBuffer, Luma};
use noise::{
    utils::{NoiseMapBuilder, PlaneMapBuilder},
    Fbm, Perlin,
};

const WIDTH: usize = 400;
const HEIGHT: usize = 400;

fn main() {
    let fbm = Fbm::<Perlin>::new(123);

    let noise_map = PlaneMapBuilder::<_, 2>::new(&fbm)
        .set_size(WIDTH, HEIGHT)
        .set_x_bounds(-2.0, 2.0)
        .set_y_bounds(-2.0, 2.0)
        .build();

    let mut pixels: Vec<u8> = Vec::with_capacity(WIDTH * HEIGHT);

    for i in noise_map.iter() {
        pixels.push(((i * 0.5 + 0.5).clamp(0.0, 1.0) * 255.0) as u8);
    }

    let mut img = ImageBuffer::from_fn(WIDTH as u32, HEIGHT as u32, |x, y| {
        let pixel = pixels[(y * WIDTH as u32 + x) as usize];
        image::Rgb([pixel, pixel, pixel])
    });

    img.save("noise_map.png").unwrap();

    // subtracting the noise map from the falloff map
    let falloff = generate_falloff_map();

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let falloff_value = falloff.get_pixel(x, y)[0];
        let noise_value = pixels[(y * WIDTH as u32 + x) as usize];

        let value = noise_value as f32 * (falloff_value as f32 / 255.0);

        *pixel = image::Rgb([value as u8, value as u8, value as u8]);

        pixels[(y * WIDTH as u32 + x) as usize] = value as u8;
    }

    img.save("falloff_noise_map.png").unwrap();

    // adding colors
    let grass_color = image::Rgb([0, 255, 0]);
    let sand_color = image::Rgb([255, 255, 0]);
    let water_color = image::Rgb([0, 0, 255]);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let value = pixels[(y * WIDTH as u32 + x) as usize];

        let color = if value < 50 {
            water_color
        } else if value < 70 {
            sand_color
        } else {
            grass_color
        };

        *pixel = color;
    }

    img.save("color_map.png").unwrap();
}

fn generate_falloff_map() -> image::ImageBuffer<Luma<u8>, Vec<u8>> {
    let mut img: ImageBuffer<Luma<u8>, Vec<_>> = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);

    let center_x = WIDTH as f32 / 2.0;
    let center_y = HEIGHT as f32 / 2.0;

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let dx = center_x - x as f32;
        let dy = center_y - y as f32;

        let distance = (dx * dx + dy * dy).sqrt();

        //let max_distance = (center_x.powi(2) + center_y.powi(2)).sqrt();
        let max_distance = 240.0;

        let falloff = 1.0 - (distance / max_distance);

        *pixel = Luma([(falloff * 255.0) as u8]);
    }

    img.save("falloff_map.png").unwrap();

    img
}
