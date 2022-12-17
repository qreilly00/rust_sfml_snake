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
    pub fn new() -> Self {
        Self {
            shape: {
                let mut rect = RectangleShape::with_size(Vector2f::new(64.0, 64.0));
                rect.set_position(Vector2f::new(256.0, 256.0));
                rect
            },

            move_time: 1.0,
            move_distance: 64.0,
            total_movement: {Vector2f::new(0.0, 0.0)},

            move_direction: Directions::DOWN,
        }
    }

    pub fn get_shape(&self) -> &RectangleShape<'a> { &self.shape }

    pub fn get_move_time(&self) -> &f32 { &self.move_time }
    //pub fn get_move_direction(&self) -> &Directions { &self.move_direction }
    pub fn get_shape_position(&self) -> Vector2f { self.shape.position() }

    pub fn set_shape_position(&mut self, pos: Vector2f) { self.shape.set_position(pos); }
    fn set_move_direction(&mut self, md: Directions) { self.move_direction = md; }
}

// Associated methods.
mod move_shape;
