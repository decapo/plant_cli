use lsystem::LSystem;
use nannou::prelude::*;
use std::collections::HashMap;
use turtle::Turtle;

mod lsystem;
mod turtle;

const SPEED: usize = 4;

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
    draw.background().color(BLACK);
    let mut turtle = Turtle::new(
        &mut draw,
        (
            app.window_rect().left() + app.window_rect().w() / 2.0,
            app.window_rect().bottom(),
        ),
    );
    model.l_system.draw(&mut turtle, model.progress);
    draw.to_frame(app, &frame).unwrap();
}

struct Model {
    l_system: LSystem,
    progress: usize,
    speed: usize,
    finished: bool,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 1000).view(view).build().unwrap();
    let axiom = "X".to_string();
    let mut rules = HashMap::new();
    rules.insert('X', "F-[[X]+X]+F[+FX]-X".to_string());
    rules.insert('F', "FF".to_string());

    let mut l_system = LSystem::new(axiom, rules, 22.5, 5.0);
    l_system.iterate(6); // Increase the number of iterations for a more complex structure
    Model {
        l_system,
        progress: 0,
        speed: SPEED,
        finished: false,
    }
}
