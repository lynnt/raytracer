use cgmath::Vector3;

pub struct Ray {
    pub a: Vector3<f64>,
    pub b: Vector3<f64>,
}

impl Ray {
    pub fn new(a: Vector3<f64>, b: Vector3<f64>) -> Ray {
        Ray { a: a, b: b }
    }

    pub fn point_at_parameter(&self, magnitude : f64) -> Vector3<f64> { self.a + magnitude * self.b }
}

/*
#[cfg(test)]
mod tests {
#[test]
fn it_works() {
assert_eq!(2 + 2, 4);
}
}
*/
