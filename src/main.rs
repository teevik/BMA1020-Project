mod vek_extension;

use crate::vek_extension::Vec2Extension;
use itertools::Itertools;
use nannou::color::{Alpha, Lch};
use nannou::event::Update;
use nannou::geom::pt2;
use nannou::noise::{NoiseFn, Perlin};
use nannou::rand::random_range;
use nannou::{App, Frame};
use std::f32::consts::PI;
use vek::{LineSegment2, Vec2};

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

const SIMULATION_SPEED: f32 = 1.;
const ENABLE_TRAILS: bool = true;
const ANT_COLLIDER_RADIUS: f32 = 2.;
const RAY_LENGTH: f32 = 50.;
const AMOUNT_OF_ANTS: usize = 1000;

#[derive(Clone)]
struct Ant {
    id: usize,
    position: Vec2<f32>,
    direction: Vec2<f32>,
    speed: f32,
}

struct Model {
    perlin: Perlin,
    ants: Vec<Ant>,
}

fn model(app: &App) -> Model {
    let boundary = app.window_rect();

    // Initialize ants
    let ants = (0..AMOUNT_OF_ANTS)
        .map(|id| {
            let position = Vec2::new(
                random_range(boundary.x.start, boundary.x.end),
                random_range(boundary.y.start, boundary.y.end),
            );

            let angle = random_range(-PI, PI);
            let direction = Vec2::new(angle.cos(), angle.sin());
            let speed = random_range(20., 100.);

            Ant {
                id,
                position,
                direction,
                speed,
            }
        })
        .collect_vec();

    let perlin = Perlin::new();

    Model { perlin, ants }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let boundary = app.window_rect();

    let delta_time = update.since_last.as_secs_f32() * SIMULATION_SPEED;

    // Clone of ants to simplify code and avoid iteration order mattering
    let previous_ants = model.ants.clone();

    for (ant_index, ant) in model.ants.iter_mut().enumerate() {
        // Random rotation by perlin
        ant.direction
            .rotate_z(model.perlin.get([app.time as f64, ant_index as f64]) as f32 * delta_time);

        // Avoid mouse
        let diff = ant.position - [app.mouse.x, app.mouse.y];

        if diff.magnitude() < 100. {
            ant.direction += diff.normalized() * 5. * diff.magnitude();
            ant.direction.normalize();
        }

        // Check if out of bounds
        if !boundary.contains(ant.position.to_glam()) {
            ant.direction += (-ant.position).normalized() / 10.;
            ant.direction.normalize();
        }

        // Scan for ants
        let steps = 10;
        let cone = (PI / 8.) / steps as f32;

        let ray_intersects_other_ant = |i| {
            let start = ant.position;
            let end = ant.position + ant.direction.rotated_z(cone * i as f32) * RAY_LENGTH;

            let line_segment = LineSegment2 { start, end };

            let intersects = previous_ants
                .iter()
                .filter(|other| {
                    ant.position.distance_squared(other.position) < (RAY_LENGTH * RAY_LENGTH)
                })
                .filter(|other| other.id != ant.id)
                .any(|other| line_segment.distance_to_point(other.position) < ANT_COLLIDER_RADIUS);

            intersects
        };

        // Check if ant can see another
        // Iterator that goes [0, -1, 1, -2, 2, ...] to make sure ray prioritizes ant directly in front
        let mut ray_indices = [0]
            .into_iter()
            .chain((1..=steps).flat_map(|i| [-1, 1].into_iter().map(move |side| i * side)));

        let ray_index = ray_indices.find(|&index| ray_intersects_other_ant(index));

        // Found ant in front
        if let Some(index) = ray_index {
            let target_angle = cone * index as f32;

            let max_rotation = 10.;
            let multiplier = 55.;

            ant.direction.rotate_z(
                (target_angle * multiplier).clamp(-max_rotation, max_rotation) * delta_time,
            );
        }

        // Update ant position
        ant.position += ant.direction * ant.speed * delta_time;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    let background_color = Lch::new(50., 50., 30.);

    // Set background color, with opacity if trails are enabled
    if app.elapsed_frames() == 1 || !ENABLE_TRAILS {
        draw.background().color(background_color);
    } else {
        draw.rect().wh(app.window_rect().wh()).color(Alpha {
            color: background_color,
            alpha: 0.02,
        });
    }

    // Draw ants
    for ant in &model.ants {
        let scale = 0.05;

        let fifty = scale * 50.0;
        let thirty_three = scale * 33.0;

        let a = [-fifty, thirty_three];
        let b = [fifty, 0.0];
        let c = [-fifty, -thirty_three];

        let rotation = ant.direction.angle();

        draw.tri()
            .xy(ant.position.to_glam())
            .points(a, b, c)
            .rotate(rotation);
    }

    draw.to_frame(app, &frame).unwrap();
}
