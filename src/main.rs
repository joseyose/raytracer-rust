mod libs;

fn ray_color(r: libs::ray::ray) -> libs::Vec3::Vec3 {
    let unit_direction = libs::Vec3::Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * libs::Vec3::Vec3::initialize(1.0, 1.0, 1.0) + t * libs::Vec3::Vec3::initialize(0.5, 0.7, 1.0);
}

fn main() {

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = libs::Vec3::Vec3::initialize(0.0, 0.0, 0.0);
    let horizontal = libs::Vec3::Vec3::initialize(viewport_width, 0.0, 0.0);
    let vertical = libs::Vec3::Vec3::initialize(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - libs::Vec3::Vec3::initialize(0.0, 0.0, focal_length);


    // Render
    // print!("P3\n{image_width} {image_height}\n255\n");
    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = libs::ray::ray::initialize(origin, lower_left_corner + u*horizontal + v*vertical - origin);
            let pixel_color = ray_color(r);
            libs::color::write_color(pixel_color);

            // let ir = (255.999 * pixel_color.x()) as i32;
            // println!("ir: {}", ir);
        }
    }

    eprintln!("Done.");
}
