use clap::{Arg, Command};
use lsystem::LSystem;
use nannou::prelude::*;
use std::collections::HashMap;
use turtle::Turtle;

mod lsystem;
mod turtle;

const SPEED: usize = 2;

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

#[derive(Clone)]
struct Model {
    l_system: LSystem,
    progress: usize,
    speed: usize,
    finished: bool,
}

fn create_model(rules_x: String, rules_y: String, delta: f32, iterations: u32) -> Model {
    let axiom = "X".to_string();

    let mut rules = HashMap::new();
    rules.insert('X', rules_x);
    rules.insert('F', rules_y);

    let mut l_system = LSystem::new(axiom, rules, delta, 5.0);
    l_system.iterate(iterations);

    Model {
        l_system,
        progress: 0,
        speed: SPEED,
        finished: false,
    }
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 1200).view(view).build().unwrap();
    let (rules_x, rules_y, delta, iterations) = get_args();
    create_model(rules_x, rules_y, delta, iterations)
}

fn get_args() -> (String, String, f32, u32) {
    let matches = Command::new("L-System Tree Drawer")
        .version("1.0")
        .author("Sammy Nouri <sammynouri@gmail.com>")
        .about("Draws trees using an L-system")
        .arg(
            Arg::new("rules_x")
                .short('x')
                .long("rules_x")
                .value_name("RULES_X")
                .help("Sets the rules for X")
                .default_value("F[+X][-X]FX")
                .num_args(1),
        )
        .arg(
            Arg::new("rules_f")
                .short('f')
                .long("rules_f")
                .value_name("RULES_F")
                .help("Sets the rules for F")
                .num_args(1),
        )
        .arg(
            Arg::new("delta")
                .short('d')
                .long("delta")
                .value_name("DELTA")
                .help("Sets the delta angle")
                .default_value("25.7")
                .num_args(1),
        )
        .arg(
            Arg::new("iterations")
                .short('i')
                .long("iterations")
                .value_name("ITERATIONS")
                .help("Sets the number of iterations")
                .default_value("7")
                .num_args(1),
        )
        .get_matches();

    let rules_x: String = matches.get_one("rules_x").cloned().unwrap();

    let rules_f = matches
        .get_one("rules_f")
        .cloned()
        .unwrap_or("FF")
        .to_string();

    let delta: f32 = matches
        .get_one::<String>("delta")
        .cloned()
        .unwrap()
        .parse::<f32>()
        .unwrap();

    let iterations: u32 = matches
        .get_one::<String>("iterations")
        .cloned()
        .unwrap()
        .parse::<u32>()
        .unwrap();

    (rules_x, rules_f, delta, iterations)
}
