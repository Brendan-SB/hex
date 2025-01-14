use crate::nalgebra::{Matrix4, Orthographic3, Vector2};
use parking_lot::RwLock;
use std::sync::Arc;

#[derive(Clone)]
pub struct Camera {
    dimensions: Vector2<f32>,
    end: u32,
    proj: Matrix4<f32>,
}

impl Camera {
    pub fn new(dimensions: Vector2<f32>, end: u32) -> Arc<RwLock<Self>> {
        Arc::new(RwLock::new(Self {
            dimensions,
            end,
            proj: Self::calculate_proj(dimensions, end),
        }))
    }

    pub fn dimensions(&self) -> Vector2<f32> {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: Vector2<f32>) {
        self.dimensions = dimensions;

        self.update_proj();
    }

    pub fn end(&self) -> u32 {
        self.end
    }

    pub fn set_end(&mut self, end: u32) {
        self.end = end;

        self.update_proj();
    }

    pub fn proj(&self) -> Matrix4<f32> {
        self.proj
    }

    fn update_proj(&mut self) {
        self.proj = Self::calculate_proj(self.dimensions, self.end);
    }

    fn calculate_proj(v: Vector2<f32>, end: u32) -> Matrix4<f32> {
        let z = end as f32;
        let v = v / 2.0;

        Orthographic3::new(-v.x, v.x, -v.y, v.y, -z, z).to_homogeneous()
    }
}
