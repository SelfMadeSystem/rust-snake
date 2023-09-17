use std::collections::VecDeque;

use speedy2d::{dimen::IVec2, Graphics2D};

use crate::utils::Direction;

use super::{food::Food, SnakeGfxSettings};

pub struct Snake {
    pub start_position: IVec2,
    pub start_direction: Direction,
    pub start_length: usize,
    pub direction: Direction,
    pub body: VecDeque<IVec2>,
}

impl Snake {
    pub fn new(start_position: IVec2, start_direction: Direction, start_length: usize) -> Self {
        let mut body = VecDeque::new();

        for i in 0..start_length {
            body.push_back(start_position - start_direction.to_ivec2() * i as i32);
        }

        Self {
            start_position,
            start_direction,
            start_length,
            direction: start_direction,
            body,
        }
    }
}

pub enum SnakeMove {
    Died,
    Ate(usize),
    Nothing,
}

impl Snake {
    pub fn get_score(&self) -> usize {
        self.body.len() - self.start_length
    }

    pub fn reset(&mut self) {
        self.body = VecDeque::new();
        self.direction = self.start_direction;

        for i in 0..self.start_length {
            self.body
                .push_back(self.start_position - self.start_direction.to_ivec2() * i as i32);
        }
    }

    pub fn move_forward(&mut self, foods: &Vec<Food>, grid_size: IVec2) -> SnakeMove {
        let new_head = *self.body.front().unwrap() + self.direction.to_ivec2();

        if new_head.x < 0 || new_head.y < 0 {
            return SnakeMove::Died;
        }

        if new_head.x >= grid_size.x || new_head.y >= grid_size.y {
            return SnakeMove::Died;
        }

        for (index, position) in self.body.iter().enumerate() {
            if index != 0 && *position == new_head {
                return SnakeMove::Died;
            }
        }

        self.body.push_front(new_head);

        for (index, food) in foods.iter().enumerate() {
            if food.position == new_head {
                return SnakeMove::Ate(index);
            }
        }

        self.body.pop_back();

        SnakeMove::Nothing
    }

    pub fn set_direction(&mut self, direction: Direction) -> bool {
        if !self.direction.is_adjacent(direction) {
            return false;
        }

        self.direction = direction;

        true
    }

    pub fn draw(&self, graphics: &mut Graphics2D, settings: &SnakeGfxSettings) {
        for position in &self.body {
            settings.draw_cell(graphics, *position, settings.snake_color);
        }

        settings.draw_cell(
            graphics,
            *self
                .body
                .front()
                .expect("There should be at least 1 item in the body"),
            settings.snake_head_color,
        );
    }
}
