use crate::libs::Vec3::Vec3;

pub fn write_color(pixel_color: Vec3) {
    // Write the translated [0,255] value of each color component.
    println!("{} {} {}", (255.999 * pixel_color.x()) as i32,
                         (255.999 * pixel_color.y()) as i32,
                         (255.999 * pixel_color.z()) as i32,
    );
}
