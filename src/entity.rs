use crate::include::*;
use crate::Directions;

pub struct Entity<'a> {
    shape: RectangleShape<'a>,

    move_time: f32, // Seconds.
    move_distance: f32,
    total_movement: Vector2f,

    move_direction: Directions,
    prev_move_direction: Directions,
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
            prev_move_direction: Directions::DOWN,
        }
    }

    pub fn move_shape(&mut self) {
        if self.move_direction == Directions::DOWN  && self.prev_move_direction != Directions::UP       { self.total_movement = Vector2f::new(0.0, self.move_distance); }
        if self.move_direction == Directions::RIGHT && self.prev_move_direction != Directions::LEFT     { self.total_movement = Vector2f::new(self.move_distance, 0.0); }
        if self.move_direction == Directions::UP    && self.prev_move_direction != Directions::DOWN     { self.total_movement = Vector2f::new(0.0, -self.move_distance); }
        if self.move_direction == Directions::LEFT  && self.prev_move_direction != Directions::RIGHT    { self.total_movement = Vector2f::new(-self.move_distance, 0.0); }

        self.shape.move_(self.total_movement);
    }

    pub fn get_shape(&self) -> &RectangleShape<'a> { &self.shape }
    pub fn get_move_time(&self) -> &f32 { &self.move_time }
    pub fn get_move_direction(&self) -> &Directions { &self.move_direction }

    pub fn set_move_direction(&mut self, md: Directions) { self.move_direction = md; }
    pub fn set_prev_move_direction(&mut self, pmd: Directions) { self.prev_move_direction = pmd; }
}
