use itertools::Itertools;
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).simple_window(view).run();
}

const ANT_VELOCITY: f32 = 100.;

struct Ant {
    position: Vec2,
    direction: Vec2,
}

struct Model {
    boundary: Rect,
    ants: Vec<Ant>,
}

fn model(app: &App) -> Model {
    let boundary = app.window_rect();

    let ants = (0..100)
        .map(|_| {
            let position = Vec2::new(
                random_range(boundary.x.start, boundary.x.end),
                random_range(boundary.y.start, boundary.y.end),
            );

            let angle = random_range(-PI, PI);
            let direction = Vec2::new(angle.cos(), angle.sin());

            Ant {
                position,
                direction,
            }
        })
        .collect_vec();

    Model { boundary, ants }
}

fn update(_app: &App, model: &mut Model, update: Update) {
    let delta_time = update.since_last.as_secs_f32();
    let boundary = model.boundary;

    for ant in &mut model.ants {
        ant.position += ant.direction * ANT_VELOCITY * delta_time;

        if !boundary.x.contains(ant.position.x) {
            ant.direction.x *= -1.;
            ant.position.x = ant.position.x.clamp(boundary.x.start, boundary.x.end);
        }

        if !boundary.y.contains(ant.position.y) {
            ant.direction.y *= -1.;
            ant.position.y = ant.position.y.clamp(boundary.y.start, boundary.y.end);
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(PLUM);

    for ant in &model.ants {
        let scale = 0.2;

        let fifty = scale * 50.0;
        let thirty_three = scale * 33.0;

        let a = pt2(-fifty, thirty_three);
        let b = pt2(fifty, 0.0);
        let c = pt2(-fifty, -thirty_three);

        draw.tri()
            .xy(ant.position)
            .points(a, b, c)
            .rotate(ant.direction.angle());
    }

    draw.to_frame(app, &frame).unwrap();
}
