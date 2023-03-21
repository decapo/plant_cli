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

fn create_model(
    rules_x: String,
    rules_y: String,
    delta: f32,
    speed: usize,
    iterations: u32,
) -> Model {
    let axiom = "X".to_string();

    let mut rules = HashMap::new();
    rules.insert('X', rules_x);
    rules.insert('F', rules_y);

    let mut l_system = LSystem::new(axiom, rules, delta, 4.0);
    l_system.iterate(iterations);

    Model {
        l_system,
        progress: 0,
        speed,
        finished: false,
    }
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 1200).view(view).build().unwrap();
    let (rules_x, rules_y, delta, speed, iterations) = get_args();
    create_model(rules_x, rules_y, delta, speed, iterations)
}

// fn calculate_tree_height(l_system: &LSystem, distance: f32, angle: f32) -> f32 {
//     let mut max_y = 0.0;
//     let mut current_position = Point2::new(0.0, 0.0);
//     let mut current_angle = 90.0;
//     let mut state_stack = Vec::new();

//     for c in l_system.axiom.chars() {
//         match c {
//             'F' => {
//                 let new_position = current_position
//                     + vec2(
//                         distance * current_angle.to_radians().cos(),
//                         distance * current_angle.to_radians().sin(),
//                     );
//                 current_position = new_position;

//                 if current_position.y > max_y {
//                     max_y = current_position.y;
//                 }
//             }
//             '+' => {
//                 current_angle -= angle;
//             }
//             '-' => {
//                 current_angle += angle;
//             }
//             '[' => {
//                 state_stack.push((current_position, current_angle));
//             }
//             ']' => {
//                 if let Some(state) = state_stack.pop() {
//                     current_position = state.0;
//                     current_angle = state.1;
//                 }
//             }
//             _ => panic!("Invalid character in LSystem: {}", c),
//         }
//     }

//     max_y
// }
