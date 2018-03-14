use color::Color;

/// Sides are described in clockwise fashion
#[derive(Debug, PartialEq, Eq)]
pub struct Side {
    colors: [Color; 2],
}

impl Side {
    pub fn new(color1: Color, color2: Color) -> Self {
        Side { colors: [color1, color2] }
    }

    pub fn flip(&mut self) {
        self.colors.swap(0, 1);
    }

    pub fn color(&self, i: usize) -> Color { self.colors[i] }
}
