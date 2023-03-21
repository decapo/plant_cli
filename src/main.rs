use nannou::prelude::*;
use std::collections::HashMap;

fn main() {
    nannou::app(model).update(update).run();
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

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
    model.l_system.draw(&mut turtle);
    draw.to_frame(app, &frame).unwrap();
}

struct LSystem {
    axiom: String,
    rules: HashMap<char, String>,
    angle: f32,
    distance: f32,
}

impl LSystem {
    fn new(axiom: String, rules: HashMap<char, String>, angle: f32, distance: f32) -> Self {
        Self {
            axiom,
            rules,
            angle,
            distance,
        }
    }

    fn iterate(&mut self, iterations: u32) {
        for _ in 0..iterations {
            let mut new_axiom = String::new();
            for c in self.axiom.chars() {
                match self.rules.get(&c) {
                    Some(replacement) => new_axiom.push_str(replacement),
                    None => new_axiom.push(c),
                }
            }
            self.axiom = new_axiom;
        }
    }

    fn draw(&self, turtle: &mut Turtle) {
        let mut state_stack = Vec::new();
        let angle = self.angle;
        let distance = self.distance;

        for c in self.axiom.chars() {
            match c {
                'F' => {
                    turtle.forward(distance);
                }
                '+' => {
                    turtle.right(angle);
                }
                '-' => {
                    turtle.left(angle);
                }
                '[' => {
                    state_stack.push(turtle.get_state());
                }
                ']' => {
                    if let Some(state) = state_stack.pop() {
                        turtle.set_state(state);
                    }
                }
                _ => {}
            }
        }
    }
}

struct Model {
    l_system: LSystem,
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    let axiom = "X".to_string();
    let mut rules = HashMap::new();
    rules.insert('X', "F-[[X]+X]+F[+FX]-X".to_string());
    rules.insert('F', "FF".to_string());

    let mut l_system = LSystem::new(axiom, rules, 22.5, 5.0);
    l_system.iterate(5); // Increase the number of iterations for a more complex structure
    Model { l_system }
}

struct Turtle<'a> {
    draw: &'a mut Draw,
    position: Point2,
    angle: f32,
}

impl<'a> Turtle<'a> {
    fn new(draw: &'a mut Draw, position: (f32, f32)) -> Self {
        Self {
            draw,
            position: pt2(position.0, position.1),
            angle: 90.0,
        }
    }

    fn forward(&mut self, distance: f32) {
        let new_position = self.position
            + vec2(
                distance * self.angle.to_radians().cos(),
                distance * self.angle.to_radians().sin(),
            );
        self.draw
            .line()
            .start(self.position)
            .end(new_position)
            .color(LIGHTGREEN)
            .stroke_weight(2.0);
        self.position = new_position;
    }

    fn right(&mut self, angle: f32) {
        self.angle -= angle;
    }

    fn left(&mut self, angle: f32) {
        self.angle += angle;
    }

    fn get_state(&self) -> (Point2, f32) {
        (self.position, self.angle)
    }

    fn set_state(&mut self, state: (Point2, f32)) {
        self.position = state.0;
        self.angle = state.1;
    }
}

