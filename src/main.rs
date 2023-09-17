use speedy2d::{color::Color, dimen::*, font::*, shape::Rectangle, window::*, *};
use std::time::Instant;

use food::Food;
use snake::*;
use utils::*;

mod food;
mod snake;
mod utils;

pub struct SnakeGfxSettings {
    pub grid_size: IVec2,
    pub cell_size: i32,
    pub cell_padding: i32,
    pub snake_head_color: Color,
    pub snake_color: Color,
    pub food_color: Color,
    pub text_font: Font,
}

impl SnakeGfxSettings {
    pub fn update(&mut self, window_size: UVec2) {
        self.grid_size = IVec2::new(
            window_size.x as i32 / self.cell_size,
            window_size.y as i32 / self.cell_size,
        );
    }

    /// i.e. no padding
    pub fn get_full_cell_rect(&self, position: IVec2) -> Rectangle {
        let position = position * self.cell_size;
        let size = IVec2::new(self.cell_size, self.cell_size);
        Rectangle::new(position.into_f32(), position.into_f32() + size.into_f32())
    }

    pub fn draw_cell(&self, graphics: &mut Graphics2D, position: IVec2, color: Color) {
        let position = position * self.cell_size + IVec2::new(self.cell_padding, self.cell_padding);
        let size = IVec2::new(
            self.cell_size - self.cell_padding * 2,
            self.cell_size - self.cell_padding * 2,
        );
        let rect = Rectangle::new(position.into_f32(), position.into_f32() + size.into_f32());
        graphics.draw_rectangle(rect, color);
    }
}

struct SnakeGame {
    snake_gfx_settings: SnakeGfxSettings,
    snake: Snake,
    foods: Vec<Food>,
    delay: f32,
    prev_time: Instant,
}

impl SnakeGame {
    fn move_snake(&mut self) {
        self.prev_time = Instant::now();
        let ate = self
            .snake
            .move_forward(&self.foods, self.snake_gfx_settings.grid_size);

        match ate {
            SnakeMove::Ate(i) => {
                let mut food = self.foods.remove(i);
                food.replace(self.snake_gfx_settings.grid_size, &self.foods);
                self.foods.push(food);
                self.delay *= 0.98;
            }
            SnakeMove::Died => {
                self.snake.reset();
                self.delay = 0.2;
            }
            _ => {}
        }
    }
}

impl WindowHandler for SnakeGame {
    fn on_draw(&mut self, helper: &mut WindowHelper<()>, graphics: &mut Graphics2D) {
        if self.prev_time.elapsed().as_secs_f32() >= self.delay {
            self.move_snake();
        }

        graphics.clear_screen(Color::BLACK);

        self.snake.draw(graphics, &self.snake_gfx_settings);
        for food in &self.foods {
            food.draw(graphics, &self.snake_gfx_settings);
        }

        graphics.draw_text(
            Vec2::new(10., 10.),
            Color::WHITE,
            &self.snake_gfx_settings.text_font.layout_text(
                &format!("{}", self.snake.get_score()),
                32.,
                TextOptions::new().with_wrap_to_width(500., TextAlignment::Left),
            ),
        );

        helper.request_redraw();
    }

    fn on_resize(&mut self, _helper: &mut WindowHelper<()>, size_pixels: UVec2) {
        self.snake_gfx_settings.update(size_pixels);
    }

    fn on_key_down(
        &mut self,
        _helper: &mut WindowHelper<()>,
        _virtual_key_code: Option<VirtualKeyCode>,
        scancode: KeyScancode,
    ) {
        if match scancode {
            KEY_UP | KEY_W => self.snake.set_direction(Direction::Up),
            KEY_DOWN | KEY_S => self.snake.set_direction(Direction::Down),
            KEY_LEFT | KEY_A => self.snake.set_direction(Direction::Left),
            KEY_RIGHT | KEY_D => self.snake.set_direction(Direction::Right),
            _ => false,
        } {
            self.move_snake();
        }
    }
}

pub fn main() {
    let window = Window::new_centered("Snake", (800, 600)).unwrap();
    let mut snake = SnakeGame {
        snake_gfx_settings: SnakeGfxSettings {
            grid_size: IVec2::new(0, 0),
            cell_size: 20,
            cell_padding: 2,
            snake_head_color: Color::YELLOW,
            snake_color: Color::GREEN,
            food_color: Color::RED,
            text_font: Font::new(include_bytes!("../assets/NotoSans-Regular.ttf")).unwrap(),
        },
        snake: Snake::new(IVec2::new(0, 0), Direction::Right, 4),
        foods: Vec::new(),
        delay: 0.2,
        prev_time: Instant::now(),
    };
    snake.snake_gfx_settings.update(UVec2::new(800, 600));
    snake.delay = 0.2;
    for _ in 0..5 {
        snake
            .foods
            .push(Food::new(snake.snake_gfx_settings.grid_size, &snake.foods));
    }
    window.run_loop(snake)
}
