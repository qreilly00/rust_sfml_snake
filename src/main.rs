mod include;
mod entity;

use crate::include::*;
use crate::entity::Entity;

#[derive(PartialEq, Clone, Copy)]
pub enum Directions {DOWN, RIGHT, UP, LEFT}

fn main() {
    let mut window = RenderWindow::new(
        (1280, 720),
        "SFML window",
        Style::CLOSE,
        &Default::default()
    );

    let mut clock = Clock::start();
    let mut total_time = clock.restart().as_seconds();

    let mut head = Entity::new();

    let mut active_direction: Directions = Directions::DOWN;
    head.set_move_direction(active_direction.clone());

    while window.is_open() {

        total_time += clock.restart().as_seconds();

        while let Some(event) = window.poll_event() {
            match event {
                Event::Closed | Event::KeyPressed { code: Key::Escape, .. } => window.close(),

                Event::KeyPressed { code: Key::S, .. } => active_direction = Directions::DOWN,
                Event::KeyPressed { code: Key::D, .. } => active_direction = Directions::RIGHT,
                Event::KeyPressed { code: Key::W, .. } => active_direction = Directions::UP,
                Event::KeyPressed { code: Key::A, .. } => active_direction = Directions::LEFT,

                _ => {}
            }
        }

        if total_time >= *head.get_move_time() {
            head.set_move_direction(active_direction.clone());

            head.move_shape();

            head.set_prev_move_direction((head.get_move_direction()).clone());
            total_time = 0.0;
        }

        window.clear(Color::BLACK);
        window.draw(head.get_shape());
        window.display();
    }
}
