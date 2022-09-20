mod libs;

fn ray_color(r: libs::ray::ray) -> libs::Vec3::Vec3 {
    let unit_direction = libs::Vec3::Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    return (1.0 - t) * libs::Vec3::Vec3::initialize(1.0, 1.0, 1.0) + t * libs::Vec3::Vec3::initialize(0.5, 0.7, 1.0);
}

fn main() {

    // Image
    let image_width = 256;
    let image_height = 256;
    //
    // // Render
    // // print!("P3\n{image_width} {image_height}\n255\n");
    // println!("P3");
    // println!("{image_width} {image_height}");
    // println!("255");
    //
    // for j in (0..image_height).rev() {
    //     eprintln!("Scanlines remaining: {}", j);
    //     for i in 0..image_width {
    //         let r = i as f64 / (image_width - 1) as f64;
    //         let g = j as f64 / (image_height - 1) as f64;
    //         let b = 0.25;
    //
    //         let ir = (255.999 * r) as i32;
    //         let ig = (255.999 * g) as i32;
    //         let ib = (255.999 * b) as i32;
    //
    //         println!("{} {} {}", ir, ig, ib);
    //     }
    // }
    //
    // eprintln!("Done.");
}
