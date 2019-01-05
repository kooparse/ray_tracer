mod ray;
mod sphere;

use crate::ray::{FVec3, HitRecord, Hitable, Ray};
use crate::sphere::Sphere;
use nalgebra_glm as glm;
use std::fs::File;
use std::io::LineWriter;
use std::io::Write;

const OUTPUT_NAME: &str = "output.ppm";
const FILE_PATH: &str = "assets/";
const NB_COLUMN: i32 = 200;
const NB_ROW: i32 = 400;

fn unit_vector(v: FVec3) -> FVec3 {
    // In mathematics, "length", "magnitude", "norm" of a vector are
    // the same thing
    v / v.magnitude()
}

fn color(r: Ray, objects: &Vec<Sphere>) -> FVec3 {
    let mut record = HitRecord::default();
    let mut is_hit = false;
    let mut closest_so_far = std::f32::MAX;

    objects.iter().for_each(|obj| {
        let mut temp_record = HitRecord::default();
        if obj.hit(&r, 0., closest_so_far, &mut temp_record) {
            is_hit = true;
            closest_so_far = temp_record.t;
            record = temp_record;
        }
    });

    if is_hit {
        return 0.5
            * glm::vec3(
                record.normal.x + 1.,
                record.normal.y + 1.,
                record.normal.z + 1.,
            );
    } else {
        let unit_direction = unit_vector(r.get_direction());
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * glm::vec3(1., 1., 1.) + t * glm::vec3(0.5, 0.7, 1.)
    }
}

fn main() -> std::io::Result<()> {
    // First, some cleanup.
    let _ = std::fs::remove_dir_all(FILE_PATH);

    // Create our empty file.
    let _ = std::fs::create_dir(FILE_PATH)?;
    let file = File::create(format!("{}{}", FILE_PATH, OUTPUT_NAME))?;
    let mut file = LineWriter::new(file);

    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", NB_ROW, NB_COLUMN).as_bytes())?;
    file.write_all(b"255\n")?;

    let mut objects: Vec<Sphere> = vec![];
    objects.push(Sphere::new(glm::vec3(0., 0., -1.), 0.5));
    objects.push(Sphere::new(glm::vec3(0., -100.5, -1.), 100.));

    let lower_left_corner = glm::vec3(-2., -1., -1.);
    let horizontal = glm::vec3(4., 0., 0.);
    let vertical = glm::vec3(0., 2., 0.);

    let ray_origin = glm::vec3(0., 0., 0.);

    for j in (0..NB_COLUMN).rev() {
        for i in 0..NB_ROW {
            let u = i as f32 / NB_ROW as f32;
            let v = j as f32 / NB_COLUMN as f32;

            let ray = Ray::new(
                ray_origin,
                lower_left_corner + u * horizontal + v * vertical,
            );

            let col = color(ray, &objects);
            let ir = (255.99 * col.x) as i32;
            let ig = (255.99 * col.y) as i32;
            let ib = (255.99 * col.z) as i32;

            let color = format!("{} {} {}\n", ir, ig, ib);
            file.write_all(color.as_bytes())?;
        }
    }

    Ok(())
}
