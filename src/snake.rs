//use crate::include::*;

use crate::directions::Directions;
use crate::Entity;

pub struct Snake<'a> {
    head: Entity<'a>,
    body: Vec<Entity<'a>>,
}

impl<'a> Snake<'a> {
    pub fn new(size: f32) -> Self{
        Self {
            head: Entity::new(size),
            body: Vec::new(),
        }
    }

    pub fn move_body(&mut self, next_move: &Directions) {
        let mut prev_position = self.head.get_shape_position();
        self.head.move_shape(next_move);

        for x in &mut self.body {
            let prev_position_body = x.get_shape_position();

            x.set_shape_position(prev_position);

            prev_position = prev_position_body;
        }
    }

    pub fn add_body_part(&mut self, size: f32) {
        self.body.push(Entity::new(size));
    }

    pub fn get_head(&self) -> &Entity { &self.head }
    pub fn get_body(&self) -> &Vec<Entity> { &self.body }
}
