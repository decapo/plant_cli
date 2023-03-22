use crate::turtle::Turtle;
use std::collections::HashMap;

#[derive(Clone)]
pub struct LSystem {
    pub axiom: String,
    rules: HashMap<char, String>,
    angle: f32,
    distance: f32,
}

impl LSystem {
    pub fn new(axiom: String, rules: HashMap<char, String>, angle: f32, distance: f32) -> Self {
        Self {
            axiom,
            rules,
            angle,
            distance,
        }
    }

    pub fn iterate(&mut self, iterations: u32) {
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

    pub fn draw(&self, turtle: &mut Turtle, progress: usize) {
        let mut state_stack = Vec::new();
        let angle = self.angle;
        let distance = self.distance;

        for (chars_drawn, c) in self.axiom.chars().enumerate() {
            if chars_drawn >= progress {
                break;
            }
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
                'X' => {}
                _ => panic!("Invalid character in LSystem: {}", c),
            }
        }
    }
}
