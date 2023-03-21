use nannou::prelude::*;

pub struct Turtle<'a> {
    draw: &'a mut Draw,
    position: Point2,
    angle: f32,
}

impl<'a> Turtle<'a> {
    pub fn new(draw: &'a mut Draw, position: (f32, f32)) -> Self {
        Self {
            draw,
            position: pt2(position.0, position.1),
            angle: 90.0,
        }
    }

    pub fn forward(&mut self, distance: f32) {
        let new_position = self.position
            + vec2(
                distance * self.angle.to_radians().cos(),
                distance * self.angle.to_radians().sin(),
            );
        self.draw
            .line()
            .start(self.position)
            .end(new_position)
            .color(YELLOWGREEN)
            .stroke_weight(2.0);
        self.position = new_position;
    }

    pub fn right(&mut self, angle: f32) {
        self.angle -= angle;
    }

    pub fn left(&mut self, angle: f32) {
        self.angle += angle;
    }

    pub fn get_state(&self) -> (Point2, f32) {
        (self.position, self.angle)
    }

    pub fn set_state(&mut self, state: (Point2, f32)) {
        self.position = state.0;
        self.angle = state.1;
    }
}
