use nannou::prelude::*;

pub struct Particle {
    pub mass: f32,
    pub radius: f32,
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
    pub color: Srgba,
    pub is_new: bool,
}

impl Particle {
    pub fn new(mass: f32, position: Vec2, velocity: Vec2, acceleration: Vec2) -> Self {
        let radius = mass.sqrt() * 20.0;
        let color = Srgba::new(1.0, 1.0, 1.0, 1.0);
        let is_new = true;
        Self {
            mass,
            radius,
            position,
            velocity,
            acceleration,
            color,
            is_new,
        }
    }

    pub fn display(&self, draw: &Draw) {
        draw.ellipse()
            .width(self.radius)
            .height(self.radius)
            .xy(self.position)
            .color(self.color);
    }
}
