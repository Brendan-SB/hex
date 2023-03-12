use crate::cid;
use hecs::component_manager::Component;
use hex_math::Ortho;

#[derive(Clone)]
pub struct Camera {
    dimensions: [f32; 3],
    view: Ortho,
    pub active: bool,
}

impl Camera {
    pub fn new(dimensions: [f32; 3], active: bool) -> Self {
        Self {
            dimensions,
            view: Self::calculate_view(&dimensions),
            active,
        }
    }

    pub fn dimensions(&self) -> [f32; 3] {
        self.dimensions
    }

    pub fn set_dimensions(&mut self, dimensions: [f32; 3]) {
        self.dimensions = dimensions;

        self.update_view();
    }

    pub fn view(&self) -> Ortho {
        self.view
    }

    fn update_view(&mut self) {
        self.view = Self::calculate_view(&self.dimensions);
    }

    fn calculate_view(dimensions: &[f32; 3]) -> Ortho {
        let dimensions = dimensions.map(|d| d / 2.0);

        Ortho::new(
            -dimensions[0],
            dimensions[0],
            -dimensions[1],
            dimensions[1],
            -dimensions[2],
            dimensions[2],
        )
    }
}

impl Component for Camera {
    fn id() -> usize {
        cid!()
    }
}
