use crate::field::{Field, FieldType};
use rand::Rng;
use crate::direction::Direction;

pub struct Board {
    width: u32,
    height: u32,
    fields: Vec<Field>,
    head_idx: u32,
    pub(crate) direction: Direction,
    pub(crate) snake_length: u32,
    pub(crate) game_over: bool
}

impl Board {
    pub fn new(width: u32, height: u32) -> Self {
        let head_idx = rand::thread_rng().gen_range(0..(width * height));

        let mut return_obj = Self {
            width,
            height,
            fields: vec![Field::new(); (width * height) as usize],
            head_idx,
            direction: Direction::None,
            snake_length: 1,
            game_over: false
        };

        return_obj.generate_apple_field();

        return return_obj;
    }

    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    fn get_coordinates(&self, idx: u32) -> [u32; 2] {
        return [idx % self.width, idx / self.width];
    }

    pub fn tick(&mut self) {
        if self.direction == Direction::None || self.game_over {
            return;
        }

        let head_coordinates = self.get_coordinates(self.head_idx);

        match self.direction {
            Direction::Left => {
                if head_coordinates[0] <= 0 {
                    self.game_over = true;
                    return;
                }
                self.head_idx -= 1;
            }
            Direction::Right => {
                if head_coordinates[0] >= self.width - 1 {
                    self.game_over = true;
                    return;
                }
                self.head_idx += 1;
            }
            Direction::Up => {
                if head_coordinates[1] <= 0 {
                    self.game_over = true;
                    return;
                }
                self.head_idx -= self.width;
            }
            Direction::Down => {
                if head_coordinates[1] >= self.height - 1 {
                    self.game_over = true;
                    return;
                }
                self.head_idx += self.width;
            }
            _ => {}
        }

        if self.fields[self.head_idx as usize].field_type == FieldType::APPLE {
            self.snake_length += 1;

            for field in self.fields.iter_mut() {
                if field.field_type == FieldType::SNAKE {
                    field.longevity += 1;
                }
            }

            if self.snake_length < self.height * self.width {
                self.generate_apple_field();
            }
        }

        for field in self.fields.iter_mut() {
            field.tick();
        }

        if self.fields[self.head_idx as usize].field_type == FieldType::SNAKE {
            self.game_over = true;
            return;
        }

        self.fields[self.head_idx as usize].set(FieldType::SNAKE, self.snake_length);
    }

    fn generate_apple_field(&mut self) {
        let mut apple_idx: usize;

        loop {
            apple_idx = rand::thread_rng().gen_range(0..(self.width * self.height)) as usize;

            if self.fields[apple_idx].field_type == FieldType::EMPTY {
                break;
            }
        }

        self.fields[apple_idx].set(FieldType::APPLE, 0);
    }

    pub fn row_as_string(&self, row: u32) -> Option<String> {
        if row >= self.height {
            return None
        }

        let mut row_string = String::new();
        let start = self.get_index(row, 0);
        let end = self.get_index(row, self.width);
        let line = &self.fields[start..end];
        for column in 0..line.len() {
            let field = line[column];
            let symbol = if self.get_index(row, column as u32) as u32 == self.head_idx { "██" } else { FieldType::get_printable(field.field_type) };
            row_string.push_str(symbol);
        }
        return Some(row_string)
    }
}
