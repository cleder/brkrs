use image::{Rgb, RgbImage};

fn main() {
    // Create 256x256 test ORM texture (Red=0.5 occlusion, Green=0.7 roughness, Blue=0.3 metallic)
    let mut orm = RgbImage::new(256, 256);
    for pixel in orm.pixels_mut() {
        *pixel = Rgb([
            (0.5 * 255.0) as u8, // Occlusion (red channel)
            (0.7 * 255.0) as u8, // Roughness (green channel)
            (0.3 * 255.0) as u8, // Metallic (blue channel)
        ]);
    }
    orm.save("tests/fixtures/textures/test_orm.png").unwrap();
    println!("Created test_orm.png");

    // Create 256x256 emissive texture (red glow pattern - radial gradient)
    let mut emissive = RgbImage::new(256, 256);
    for y in 0..256 {
        for x in 0..256 {
            let dx = (x as f32 - 128.0).abs();
            let dy = (y as f32 - 128.0).abs();
            let dist = (dx * dx + dy * dy).sqrt();
            let intensity = (1.0 - (dist / 180.0).min(1.0)).max(0.0);
            emissive.put_pixel(
                x,
                y,
                Rgb([
                    (intensity * 255.0) as u8, // Red glow
                    0,                         // No green
                    0,                         // No blue
                ]),
            );
        }
    }
    emissive
        .save("tests/fixtures/textures/test_emissive.png")
        .unwrap();
    println!("Created test_emissive.png");

    // Create 256x256 depth texture (grayscale depth pattern - circular depression)
    let mut depth = RgbImage::new(256, 256);
    for y in 0..256 {
        for x in 0..256 {
            let dx = (x as f32 - 128.0).abs();
            let dy = (y as f32 - 128.0).abs();
            let dist = (dx * dx + dy * dy).sqrt();
            // Center is darkest (deepest), edges are lightest (highest)
            let depth_value = (dist / 180.0).min(1.0);
            let gray = (depth_value * 255.0) as u8;
            depth.put_pixel(x, y, Rgb([gray, gray, gray]));
        }
    }
    depth
        .save("tests/fixtures/textures/test_depth.png")
        .unwrap();
    println!("Created test_depth.png");
}
