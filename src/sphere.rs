use cgmath::Vector3;
use crate::hit_record::HitTable;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}

impl HitTable for Sphere {
    fn new(self, cen: Vector3<f64>, r: f64) -> Sphere {
        Sphere { center: cen, radius : r }
    }
}
