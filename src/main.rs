use nannou::{draw::properties::Srgba, event::WindowEvent, prelude::*};

mod particle;
use particle::Particle;

const WINDOW_WIDTH: u32 = 1000;
const WINDOW_HEIGHT: u32 = 800;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    particles: Vec<Particle>,
    settings: Settings,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .size(WINDOW_WIDTH, WINDOW_HEIGHT)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .event(event)
        .build()
        .unwrap();

    let settings = Settings::new();
    let particles = Vec::new();

    Model {
        particles,
        settings,
    }
}

fn mouse_pressed(_app: &App, model: &mut Model, mouse_button: MouseButton) {
    if mouse_button == MouseButton::Left {
        let mass = random_range(1.0, 10.0);
        let position = _app.mouse.position();
        let velocity = Vec2::default();
        let acceleration = Vec2::default();
        let mut particle = Particle::new(mass, position, velocity, acceleration);
        particle.color = Srgba::new(
            (random_f32() + 1.0) * 0.5,
            (random_f32() + 1.0) * 0.5,
            (random_f32() + 1.0) * 0.5,
            0.5,
        );
        model.particles.push(particle);
    }
}

fn event(app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(button) => {
            if button == MouseButton::Left {
                model.settings.is_mouse_pressed = true;
                model.settings.mouse_start_position = app.mouse.position();
            }
        }
        MouseReleased(button) => {
            if button == MouseButton::Left {
                model.settings.is_mouse_pressed = false;
                model.settings.mouse_end_position = app.mouse.position()
            }
        }
        _ => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let max_length_screen_size = Vec2::new(WINDOW_WIDTH as f32, WINDOW_HEIGHT as f32).length();

    if model.settings.is_mouse_pressed {
        model.settings.mouse_end_position = app.mouse.position();
    }

    for particle in model.particles.iter_mut() {
        if model.settings.is_mouse_pressed && particle.is_new {
            // no velocity applied while mouse pressed and particle is new
            particle.velocity = Vec2::new(0.0, 0.0);
        }

        if !model.settings.is_mouse_pressed && particle.is_new {
            particle.is_new = false;

            // create a direction vector from particle position to the mouse position
            let direction = model.settings.mouse_end_position - particle.position;

            // create the velocity vector and remap its strength
            let strength_factor = 40.0;
            let velocity_strength = (direction.length() / max_length_screen_size) * strength_factor;

            // apply the velocity
            particle.velocity = direction.normalize() * velocity_strength;
        }

        // apply velocity to particle
        particle.velocity += particle.acceleration;
        particle.position += particle.velocity;

        // set window min and max
        let window_x_min = -(WINDOW_WIDTH as f32 - particle.radius) * 0.5;
        let window_x_max = (WINDOW_WIDTH as f32 - particle.radius) * 0.5;
        let window_y_min = -(WINDOW_HEIGHT as f32 - particle.radius) * 0.5;
        let window_y_max = (WINDOW_HEIGHT as f32 - particle.radius) * 0.5;

        // fix border collision
        if particle.position.x < window_x_min {
            particle.velocity.x *= -1.0;
            particle.position.x = window_x_min;
        } else if particle.position.x > window_x_max {
            particle.velocity.x *= -1.0;
            particle.position.x = window_x_max;
        }
        if particle.position.y < window_y_min {
            particle.velocity.y *= -1.0;
            particle.position.y = window_y_min;
        } else if particle.position.y > window_y_max {
            particle.velocity.y *= -1.0;
            particle.position.y = window_y_max;
        }
    }

    // logic to solve collisions between particles
    for i in 0..model.particles.len() {
        let (left, right) = model.particles.split_at_mut(i + 1);
        let particle_a = &mut left[i];
        for particle_b in right {
            // get normal vector
            let normal = particle_b.position - particle_a.position;

            // get unit normal and tangent vectors
            let unit_normal = normal.normalize();
            let unit_tangent = Vec2::new(-unit_normal.y, unit_normal.x);

            // calculate distance
            let distance = normal.length() * 2.0;

            // if colliding
            if distance < (particle_a.radius + particle_b.radius) {
                // logic to fix overlap when colliding
                let overlap = distance - (particle_a.radius + particle_b.radius);
                let dir = unit_normal * overlap * 0.5;
                particle_a.position += dir;
                particle_b.position -= dir;

                // calculate dot product for normal and tangent vectors
                let normal_dot_a = particle_a.velocity.dot(unit_normal);
                let tangent_dot_a = particle_a.velocity.dot(unit_tangent);
                let normal_dot_b = particle_b.velocity.dot(unit_normal);
                let tangent_dot_b = particle_b.velocity.dot(unit_tangent);

                // formula to find the new velocities after collision
                let num_a = normal_dot_a * (particle_a.mass - particle_b.mass)
                    + 2.0 * particle_b.mass * normal_dot_b;
                let den_a = particle_a.mass + particle_b.mass;
                let num_b = normal_dot_b * (particle_b.mass - particle_a.mass)
                    + 2.0 * particle_a.mass * normal_dot_a;
                let den_b = particle_a.mass + particle_b.mass;
                let normal_dot_a_prime = num_a / den_a;
                let normal_dot_b_prime = num_b / den_b;
                let tangent_dot_a_prime = tangent_dot_a;
                let tangent_dot_b_prime = tangent_dot_b;
                let normal_a_velocity = unit_normal * normal_dot_a_prime;
                let tangent_a_velocity = unit_tangent * tangent_dot_a_prime;
                let normal_b_velocity = unit_normal * normal_dot_b_prime;
                let tangent_b_velocity = unit_tangent * tangent_dot_b_prime;
                particle_a.velocity = normal_a_velocity + tangent_a_velocity;
                particle_b.velocity = normal_b_velocity + tangent_b_velocity;
            }
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    // draw particles
    for particle in model.particles.iter() {
        particle.display(&draw);
    }

    // draw arrow
    if model.settings.is_mouse_pressed {
        draw.arrow().weight(5.0).points(
            model.particles.last().unwrap().position,
            model.settings.mouse_end_position,
        );
    };

    draw.to_frame(app, &frame).unwrap();
}

struct Settings {
    is_mouse_pressed: bool,
    mouse_start_position: Vec2,
    mouse_end_position: Vec2,
}

impl Settings {
    fn new() -> Self {
        let is_mouse_pressed = false;
        let mouse_start_position = Vec2::default();
        let mouse_end_position = Vec2::default();
        Self {
            is_mouse_pressed,
            mouse_start_position,
            mouse_end_position,
        }
    }
}
