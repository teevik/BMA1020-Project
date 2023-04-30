mod vek_extension;

use crate::vek_extension::Vec2Extension;
use itertools::Itertools;
use nannou::color::{Alpha, Lch};
use nannou::event::Update;
use nannou::geom::{pt2, Range, Rect};
use nannou::noise::{NoiseFn, Perlin};
use nannou::prelude::{map_range, Inv};
use nannou::rand::random_range;
use nannou::{App, Draw, Frame};
use nannou_egui::egui::Checkbox;
use nannou_egui::{self, egui, Egui};
use std::f32::consts::PI;
use vek::{LineSegment2, Vec2};

fn main() {
    nannou::app(model).update(update).run();
}

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

struct Settings {
    debug_mode: bool,
    enable_trails: bool,
    simulation_speed: f32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            debug_mode: false,
            enable_trails: true,
            simulation_speed: 1.,
        }
    }
}

struct Model {
    settings: Settings,
    egui: Egui,
    debug_draw: Draw,
    perlin: Perlin,
    boundary: Rect,
    ants: Vec<Ant>,
}

fn model(app: &App) -> Model {
    fn raw_window_event(_app: &App, model: &mut Model, event: &nannou::winit::event::WindowEvent) {
        // Let egui handle things like keyboard and mouse input.
        model.egui.handle_raw_event(event);
    }

    let window_id = app
        .new_window()
        .view(view)
        .raw_event(raw_window_event)
        .build()
        .unwrap();
    let window = app.window(window_id).unwrap();

    let egui = Egui::from_window(&window);

    let boundary = app.window_rect();

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

    let debug_draw = Draw::new();
    let perlin = Perlin::new();

    Model {
        settings: Default::default(),
        egui,
        debug_draw,
        perlin,
        boundary,
        ants,
    }
}

fn update(app: &App, model: &mut Model, update: Update) {
    let egui = &mut model.egui;
    let settings = &mut model.settings;
    let debug_draw = &model.debug_draw;

    model.boundary = app.window_rect();

    let delta_time = update.since_last.as_secs_f32() * settings.simulation_speed;
    let boundary = model.boundary;

    egui.set_elapsed_time(update.since_start);
    let ctx = egui.begin_frame();

    egui::Window::new("Settings").show(&ctx, |ui| {
        let Settings {
            debug_mode,
            enable_trails,
            simulation_speed,
        } = settings;

        ui.add(Checkbox::new(debug_mode, "Debug mode"));

        ui.add(Checkbox::new(enable_trails, "Enable trails"));

        ui.label("Simulation speed");
        ui.add(egui::Slider::new(simulation_speed, 0.1..=3.));
    });

    let previous_ants = model.ants.clone();

    for (ant_index, ant) in model.ants.iter_mut().enumerate() {
        // Random rotation by perlin
        ant.direction
            .rotate_z(model.perlin.get([app.time as f64, ant_index as f64]) as f32 * delta_time);

        let diff = ant.position - [app.mouse.x, app.mouse.y];

        if diff.magnitude() < 100. {
            ant.direction += diff.normalized() * 10. - diff.magnitude();
        }

        // (0. ..= 100.).l
        // let pog = map_range(diff.magnitude(), 100., 0., 0., 1.);

        // ant.position += diff.normalized() * diff.magnitude_squared().inv() * 100.;

        // Check if out of bounds
        if !boundary.contains(ant.position.as_glam()) {
            ant.direction += (-ant.position).normalized() / 10.;
        }

        ant.direction.normalize();

        let steps = 10;
        let cone = (PI / 8.) / steps as f32;

        let ray_intersects_other_ant = |i| {
            let start = ant.position;
            let end = ant.position + ant.direction.rotated_z(cone * i as f32) * RAY_LENGTH;

            let line_segment = LineSegment2 { start, end };

            if settings.debug_mode {
                debug_draw.line().start(start.as_glam()).end(end.as_glam());
            }

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

        if let Some(index) = ray_index {
            let target_angle = cone * index as f32;

            let max_rotation = 10000.;
            let multiplier = 55.;

            ant.direction.rotate_z(
                target_angle.clamp(-max_rotation, max_rotation) * multiplier * delta_time,
            );
        }

        // Update ant position
        ant.position += ant.direction * ant.speed * delta_time;
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    let Settings {
        debug_mode,
        enable_trails,
        ..
    } = model.settings;

    let background_color = Lch::new(50., 50., 30.);

    if app.elapsed_frames() == 1 || !enable_trails {
        draw.background().color(background_color);
    } else {
        draw.rect().wh(app.window_rect().wh()).color(Alpha {
            color: background_color,
            alpha: 0.02,
        });
    }

    for ant in &model.ants {
        let scale = 0.05;

        let fifty = scale * 50.0;
        let thirty_three = scale * 33.0;

        let a = pt2(-fifty, thirty_three);
        let b = pt2(fifty, 0.0);
        let c = pt2(-fifty, -thirty_three);

        let rotation = ant.direction.angle();

        draw.tri()
            .xy(ant.position.as_glam())
            .points(a, b, c)
            .rotate(rotation);

        if debug_mode {
            draw.ellipse()
                .xy(ant.position.as_glam())
                .radius(ANT_COLLIDER_RADIUS);
        }
    }

    draw.to_frame(app, &frame).unwrap();
    model.debug_draw.to_frame(app, &frame).unwrap();
    model.egui.draw_to_frame(&frame).unwrap();
}
