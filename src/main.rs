use crate::include::*;

use crate::directions::Directions;
use crate::entity::Entity;
use crate::snake::Snake;

mod include;
mod directions;
mod entity;
mod snake;

fn main() {
    let mut window = RenderWindow::new(
        (1280, 720),
        "SFML window",
        Style::CLOSE,
        &Default::default()
    );

    let mut clock = Clock::start();
    let mut total_time = clock.restart().as_seconds();

    let mut my_snake = Snake::new();
    my_snake.add_body_part();
    my_snake.add_body_part();
    my_snake.add_body_part();

    let mut active_direction: Directions = Directions::DOWN;

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

        if total_time >= *my_snake.get_head().get_move_time() {
            my_snake.move_body(&active_direction);
            total_time = 0.0;
        }

        window.clear(Color::BLACK);

        window.draw(my_snake.get_head().get_shape());
        for x in my_snake.get_body() {
            window.draw(x.get_shape());
        }

        window.display();
    }
}
