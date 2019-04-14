mod ray;
mod hit_record;
mod sphere;

use cgmath::{Vector3, dot};
use crate::ray::Ray;

fn length(v: &Vector3<f64>) -> f64 {
    let square_len = v.x.powi(2) + v.y.powi(2) + v.z.powi(2);
    square_len.sqrt()
}

fn unit_vector(v : Vector3<f64>) -> Vector3<f64> {
    v / length(&v)
}

fn hit_sphere(center: Vector3<f64>, radius: f64, r: &Ray) -> f64 {
    let oc = r.a - center;
    let a = dot(r.b, r.b);
    let b = 2.0 * dot(oc, r.b);
    let c = dot(oc, oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt())/(2.0 * a)
    }
}

// linearly blend white + blue depending on the up/downess of the y coordinate
fn color(r : Ray) -> Vector3<f64> {
    let t = hit_sphere(Vector3::new(0.0, 0.0, -1.0), 0.5, &r);
    if t > 0.0 {
        let n = unit_vector(r.point_at_parameter(t) - Vector3::new(0.0, 0.0, -1.0));
        0.5 * Vector3::new(n.x + 1.0, n.y + 1.0, n.z + 1.0)
    } else {
        let unit_direction = unit_vector(r.b);
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Vector3::new(1.0, 1.0, 1.0) + t * Vector3::new(0.5, 0.7, 1.0)
    }
}

fn main() {
    let nx = 200;
    let ny = 100;
    println!("P3\n{} {}\n255", nx, ny);
    let lower_left_corner = Vector3::new(-2.0, -1.0, -1.0);
    let horizontal = Vector3::new(4.0, 0.0, 0.0);
    let vertical = Vector3::new(0.0, 2.0, 0.0);
    let origin = Vector3::new(0.0, 0.0, 0.0);

    for j in (0..ny-1).rev() {
        for i in 0..nx {
            let u : f64 = i as f64/ nx as f64;
            let v : f64 = j as f64 / ny as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
            let col = color(r);
            let ir = (255.99 * col.x) as i64;
            let ig = (255.99 * col.y) as i64;
            let ib = (255.99 * col.z) as i64;
            println!("{} {} {}", ir, ig, ib);
        }
    }
}
