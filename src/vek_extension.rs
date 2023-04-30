pub trait Vec2Extension {
    fn to_glam(self) -> nannou::glam::Vec2;

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
