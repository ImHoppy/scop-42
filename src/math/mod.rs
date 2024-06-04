// pub use angle;
pub use matrix::Matrix4;
pub use vector::{vec2, vec3, vec4, Vector2, Vector3, Vector4};

pub type Vec2 = Vector2<f32>;
pub type Vec3 = Vector3<f32>;
pub type Vec4 = Vector4<f32>;

mod angle;
mod matrix;
mod vector;