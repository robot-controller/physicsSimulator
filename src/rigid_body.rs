use cgmath::InnerSpace;
use cgmath::Point3;
use cgmath::Vector3;

pub struct RigidBody {
    // Linear
    pub position: Point3<f32>,
    pub velocity: Vector3<f32>,
    pub mass: f32,

    // Angular
    pub orientation: Quaternion<f32>,
    pub inertia_tensor: Matrix3<f32>,
    pub angular_velocity: Vector3<f32>,
    pub force: Vector3<f32>,
    pub torque: Vector3<f32>,
}
