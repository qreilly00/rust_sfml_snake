use crate::entity::*;

impl<'a> Entity<'a> {
    pub fn move_shape(&mut self, next_move: &Directions) {
        let next_move = *next_move;

        if next_move == Directions::DOWN  && self.move_direction != Directions::UP       { self.total_movement = Vector2f::new(0.0, self.move_distance); self.set_move_direction(next_move);}
        if next_move == Directions::RIGHT && self.move_direction != Directions::LEFT     { self.total_movement = Vector2f::new(self.move_distance, 0.0); self.set_move_direction(next_move);}
        if next_move == Directions::UP    && self.move_direction != Directions::DOWN     { self.total_movement = Vector2f::new(0.0, -self.move_distance); self.set_move_direction(next_move);}
        if next_move == Directions::LEFT  && self.move_direction != Directions::RIGHT    { self.total_movement = Vector2f::new(-self.move_distance, 0.0); self.set_move_direction(next_move);}

        self.shape.move_(self.total_movement);
    }
}
