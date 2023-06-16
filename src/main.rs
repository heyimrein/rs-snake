#![windows_subsystem = "windows"]

use macroquad::prelude::*;
use macroquad::rand::gen_range;


#[macroquad::main(window_conf)]
async fn main() {
    let win = window_conf();
    let grid_size = vec2(win.window_width as f32 / 16., win.window_height as f32 / 16.);
    let mut state = State {
        snake: Snake::new(),
        fruit: vec2(grid_size.x / 2., grid_size.x / 2.)
    };

    // Timer for moving the snake at a reasonable rate
    let mut timer = Timer {
        start: std::time::SystemTime::now(),
        prev_time: 0.,
        move_interval: 0.075,
        move_timer: 0.
    };

    // Game Loop
    let mut running = true;
    while running {
        clear_background(Color::from_rgba(34, 32, 52, 255));

        let now = timer.start.elapsed().unwrap().as_secs_f32();
        let delta = now - timer.prev_time;
        timer.prev_time = now;

        let snake = &mut state.snake;

        // Input Handling

        let mut prev_dir = vec2(0., 0.);
        if snake.segments.len() > 1 {
            prev_dir = snake.segments[0] - snake.segments[1];
        }

        if is_key_pressed(KeyCode::Up) && prev_dir != vec2(0., 1.) {
            snake.direction = vec2(0., -1.);
        }
        if is_key_pressed(KeyCode::Down) && prev_dir != vec2(0., -1.) {
            snake.direction = vec2(0., 1.);
        }
        if is_key_pressed(KeyCode::Left) && prev_dir != vec2(1., 0.) {
            snake.direction = vec2(-1., 0.);
        }
        if is_key_pressed(KeyCode::Right) && prev_dir != vec2(-1., 0.) {
            snake.direction = vec2(1., 0.);
        }

        // Draw fruit
        draw_circle(
            state.fruit.x * 16. + 8.,
            state.fruit.y * 16. + 8.,
            8.,
            Color::from_rgba(255, 0, 79, 255)
        );

        // Move Snake
        if timer.move_timer >= timer.move_interval {
            let mut future_pos = snake.direction + snake.segments[0];

            // Wrap position on screen edges
            if future_pos.x > grid_size.x - 1. {
                future_pos.x = 0.;
            } else if future_pos.x < 0. {
                future_pos.x = grid_size.x - 1.;
            } else if future_pos.y > grid_size.y - 1. {
                future_pos.y = 0.;
            } else if future_pos.y < 0. {
                future_pos.y = grid_size.y - 1.;
            }

            // Check for segment overlap
            for i in 0..snake.segments.len() {
                if future_pos == snake.segments[i] {
                    running = false;
                }
            }

            snake.segments.insert(0, future_pos);

            if future_pos != state.fruit {
                snake.segments.pop();
            } else {
                loop {
                    state.fruit = vec2(
                        gen_range(0., grid_size.x - 1.).round(),
                        gen_range(0., grid_size.y - 1.).round()
                    );

                    let mut absence_flag = false;
                    for i in 0..snake.segments.len() {
                        if state.fruit == snake.segments[i] {
                            absence_flag = true;
                            break;
                        }
                    }
                    if !absence_flag { break; };
                }
            }

            timer.move_timer = 0.;
        } else {
            timer.move_timer += delta;
        }

        // Draw Snake
        for i in 0..snake.segments.len() {
            let segment = &snake.segments[i];
            draw_circle(
                segment.x * 16. + 8.,
                segment.y * 16. + 8.,
                8.,
                Color::from_rgba(255, 253, 229, 255)
            )
        }

        next_frame().await;
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: String::from("snake"),
        window_width: 512,
        window_height: 512,
        ..Default::default()
    }
}


struct State {
    snake: Snake,
    fruit: Vec2
}


struct Snake {
    direction: Vec2,
    segments: Vec<Vec2>
}

impl Snake {
    fn new() -> Self {
        Snake {
            direction: vec2(1., 0.),
            segments: vec![vec2(0., 0.)]
        }
    }
}


/// Snake movement timer
struct Timer {
    start: std::time::SystemTime,
    prev_time: f32,
    move_interval: f32,
    move_timer: f32
}
