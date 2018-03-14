use color::Color;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Corner {
    colors: [Color; 3],
}

impl Corner {
    pub fn new(color1: Color, color2: Color, color3: Color) -> Self {
        Corner { colors: [color1, color2, color3] }
    }

    pub fn clockwise(&mut self) {
        self.colors.swap(0, 2);
        self.colors.swap(2, 1);
    }

    pub fn counter_clockwise(&mut self) {
        self.colors.swap(0, 1);
        self.colors.swap(1, 2);
    }

    pub fn color(&self, i: usize) -> Color { self.colors[i] }
}
