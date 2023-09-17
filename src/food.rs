use speedy2d::{dimen::IVec2, Graphics2D};

use super::SnakeGfxSettings;

pub struct Food {
    pub position: IVec2,
}

impl Food {
    pub fn new(grid_size: IVec2, foods: &Vec<Food>) -> Self {
        let mut food = Self {
            position: IVec2 {
                x: rand::random::<i32>().abs() % grid_size.x,
                y: rand::random::<i32>().abs() % grid_size.y,
            },
        };
        while foods.iter().any(|f| f.position == food.position) {
            food.position = IVec2 {
                x: rand::random::<i32>().abs() % grid_size.x,
                y: rand::random::<i32>().abs() % grid_size.y,
            };
        }

        food
    }

    pub fn replace(&mut self, grid_size: IVec2, foods: &Vec<Food>) {
        self.position = IVec2 {
            x: rand::random::<i32>().abs() % grid_size.x,
            y: rand::random::<i32>().abs() % grid_size.y,
        };
        while foods.iter().any(|food| food.position == self.position) {
            self.position = IVec2 {
                x: rand::random::<i32>().abs() % grid_size.x,
                y: rand::random::<i32>().abs() % grid_size.y,
            };
        }
    }

    pub fn draw(&self, graphics: &mut Graphics2D, settings: &SnakeGfxSettings) {
        settings.draw_cell(graphics, self.position, settings.food_color);
    }
}
