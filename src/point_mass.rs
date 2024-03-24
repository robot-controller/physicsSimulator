use cgmath::InnerSpace;
use cgmath::Point3;
use cgmath::Vector3;

pub(crate) const GRAVITATION_CONSTANT: f32 = 6.67430e-11;

pub(crate) const SOLAR_MASS: f32 = 1.989e30;

pub struct PointMass {
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub mass: f32,
}

impl PointMass {
    pub fn new(position: Point3<f32>, velocity: Vector3<f32>, mass: f32) -> PointMass {
        PointMass {
            position,
            velocity,
            mass,
        }
    }

    pub fn distance_to(&self, other: &PointMass) -> f32 {
        (self.position - other.position).magnitude()
    }

    /// Calculate the force on
    pub fn force_on(&self, other: &PointMass) -> Vector3<f32> {
        let position_difference = self.position - other.position;
        let distance = position_difference.magnitude();
        let magnitude = GRAVITATION_CONSTANT * self.mass * other.mass / distance.powi(2);
        magnitude * position_difference
    }

    pub fn integration_step(&mut self, force : Vector3<f32>, time: f32) {
        self.velocity += force / self.mass * time;
        self.position += self.velocity * time;
    }
}
