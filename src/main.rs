use crate::args::get_args;
use lsystem::LSystem;
use nannou::prelude::*;
use std::collections::HashMap;
use turtle::Turtle;

mod args;
mod lsystem;
mod turtle;

fn main() {
    nannou::app(model).update(update).run();
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if !model.finished {
        model.progress += model.speed;
        if model.progress >= model.l_system.axiom.len() {
            model.finished = true;
            model.progress = model.l_system.axiom.len();
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    draw.background().color(model.bg_color);
    let mut turtle = Turtle::new(
        &mut draw,
        (
            app.window_rect().left() + app.window_rect().w() / 2.0,
            app.window_rect().bottom(),
        ),
        model.tree_color,
    );
    model.l_system.draw(&mut turtle, model.progress);
    draw.to_frame(app, &frame).unwrap();
}

#[derive(Clone)]
struct Model {
    l_system: LSystem,
    progress: usize,
    speed: usize,
    finished: bool,
    bg_color: Rgb<u8>,
    tree_color: Rgb<u8>,
}

#[allow(clippy::too_many_arguments)]
fn create_model(
    rules_x: String,
    rules_y: String,
    delta: f32,
    speed: usize,
    iterations: u32,
    bg_color: Rgb<u8>,
    tree_color: Rgb<u8>,
    scaling_factor: f32,
) -> Model {
    let axiom = "X".to_string();

    let mut rules = HashMap::new();
    rules.insert('X', rules_x);
    rules.insert('F', rules_y);

    let mut l_system = LSystem::new(axiom, rules, delta, scaling_factor * 4.0);
    l_system.iterate(iterations);

    Model {
        l_system,
        progress: 0,
        speed,
        finished: false,
        bg_color,
        tree_color,
    }
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 1200).view(view).build().unwrap();
    let (rules_x, rules_y, delta, speed, iterations, bg_color, tree_color, scaling_factor) =
        get_args();
    create_model(
        rules_x,
        rules_y,
        delta,
        speed,
        iterations,
        bg_color,
        tree_color,
        scaling_factor,
    )
}
