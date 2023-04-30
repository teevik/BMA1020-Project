/// Helpers for vek's Vec2
pub trait Vec2Extension {
    /// Convert vek::Vec2 into glam::Vec2
    fn to_glam(self) -> nannou::glam::Vec2;

    /// Find angle of vector
    fn angle(self) -> f32;
}

impl Vec2Extension for vek::Vec2<f32> {
    fn to_glam(self) -> nannou::geom::Vec2 {
        nannou::geom::Vec2::new(self.x, self.y)
    }

    fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }
}
