use crate::include::*;

use crate::directions::Directions;

pub struct Entity<'a> {
    shape: RectangleShape<'a>,

    move_time: f32, // Seconds.
    move_distance: f32,
    total_movement: Vector2f,

    move_direction: Directions,
}

impl<'a> Entity<'a> {
    pub fn new(scale: f32) -> Self {
        Self {
            shape: {
                let mut rect = RectangleShape::with_size(Vector2f::new(scale, scale));
                rect.set_position(Vector2f::new(0.0, 0.0));
                rect
            },

            move_time: 1.0,
            move_distance: scale,
            total_movement: Vector2f::new(0.0, 0.0),

            move_direction: Directions::DOWN,
        }
    }

    pub fn get_shape(&self) -> &RectangleShape<'a> { &self.shape }

    pub fn get_move_time(&self) -> &f32 { &self.move_time }
    pub fn get_shape_position(&self) -> Vector2f { self.shape.position() }

    pub fn adjust_shape_position(&mut self, pos: Vector2f) { self.shape.set_position(pos); }
    fn set_move_direction(&mut self, md: Directions) { self.move_direction = md; }
}

// Associated methods.
mod move_shape;
