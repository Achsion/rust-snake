#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum FieldType {
    EMPTY,
    APPLE,
    SNAKE
}

impl FieldType {
    pub(crate) fn get_printable(field_type: FieldType) -> &'static str {
        match field_type {
            FieldType::APPLE => "◀▶",
            FieldType::SNAKE => "▓▓",
            _ => "░░"
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub(crate) field_type: FieldType,
    pub longevity: u32
}

impl Field {
    pub fn new() -> Field {
        Field {
            field_type: FieldType::EMPTY,
            longevity: 0
        }
    }

    pub fn tick(&mut self) {
        if self.longevity == 0 || self.field_type == FieldType::EMPTY {
            return;
        }

        self.longevity -= 1;
        if self.longevity == 0 {
            self.field_type = FieldType::EMPTY;
        }
    }

    pub fn set(&mut self, field_type: FieldType, longevity: u32) {
        self.field_type = field_type;
        self.longevity = longevity;
    }
}