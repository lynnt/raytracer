use cgmath::{dot, Vector3};
use crate::hit_record::{HitRecord, HitTable};
use crate::ray::Ray;

pub struct Sphere {
    center: Vector3<f64>,
    radius: f64,
}

impl Sphere {
    fn new(self, cen: Vector3<f64>, r: f64) -> Sphere {
        Sphere { center: cen, radius : r }
    }
}

impl HitTable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.a - self.center;
        let a = dot(r.b, r.b);
        let b = dot(oc, r.b);
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - a * c;

        let temp = (-b - discriminant.sqrt())/a;
        if discriminant > 0.0 {
            if temp < t_max && temp > t_min {
                rec.t = temp;
                rec.p = r.point_at_parameter(rec.t);
                rec.normal = (rec.p - self.center)/self.radius;
                true
            } else { 
                let temp = (-b + discriminant.sqrt())/a;
                if temp < t_max && temp > t_min {
                    rec.t = temp;
                    rec.p = r.point_at_parameter(rec.t);
                    rec.normal = (rec.p - self.center)/self.radius;
                    true
                } else {
                    false
                }
            }
        } else {
            false
        }
    }
}
