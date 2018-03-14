use color::Color;
use std::mem;
use std::fmt;
use rand;
use rand::Rng;
use side::Side;
use corner::Corner;

/// A Rubick's cube
///
/// We define the front as white and the bottom as orange.
#[derive(Debug, PartialEq, Eq)]
pub struct Cube {
    /// Start on the bottom on the face (OW) and go clockwise _on the
    /// bottom_.  It is as if we look from below the cube.
    pub bottom_sides: [Side; 4],
    /// Start in the bottom left corner (OBW) and go clockwise _on the
    /// bottom_.  It is as if we look from below the cube.
    pub bottom_corners: [Corner; 4],
    /// Start on the left side of the face (WB) and go clockwise _on
    /// the top_.  It is as if we look from the right side of the
    /// piece.
    pub middle_sides: [Side; 4],
    /// Start on the top on thef face (RW) and go clockwise _on the
    /// top_.  It is as if we look from above the cube.
    pub top_sides: [Side; 4],
    /// Start in the top left corner (RWB) and go clockwise _on the
    /// top_.  It is as if we look from above the cube.
    pub top_corners: [Corner; 4],
}

impl fmt::Display for Cube {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "          {} {} {}

          {} {} {}
          {} {} {}
          {} {} {}

{}  {} {} {}  {} {} {}  {} {} {}  {}
{}  {} {} {}  {} {} {}  {} {} {}  {}
{}  {} {} {}  {} {} {}  {} {} {}  {}

          {} {} {}
          {} {} {}
          {} {} {}

          {} {} {}",
               self.top_corners[1].color(2), self.top_sides[2].color(1), self.top_corners[2].color(1),
               
               self.top_corners[1].color(0), self.top_sides[2].color(0), self.top_corners[2].color(0),
               self.top_sides[1].color(0), Color::Red, self.top_sides[3].color(0),
               self.top_corners[0].color(0), self.top_sides[0].color(0), self.top_corners[3].color(0),

               self.top_corners[1].color(2),
               self.top_corners[1].color(1), self.top_sides[1].color(1), self.top_corners[0].color(2),
               self.top_corners[0].color(1), self.top_sides[0].color(1), self.top_corners[3].color(2),
               self.top_corners[3].color(1), self.top_sides[3].color(1), self.top_corners[2].color(2),
               self.top_corners[2].color(1),

               self.middle_sides[1].color(1),
               self.middle_sides[1].color(0), Color::Blue, self.middle_sides[0].color(1),
               self.middle_sides[0].color(0), Color::White, self.middle_sides[3].color(1),
               self.middle_sides[3].color(0), Color::Green, self.middle_sides[2].color(1),
               self.middle_sides[2].color(0),

               self.bottom_corners[3].color(1),
               self.bottom_corners[3].color(2), self.bottom_sides[3].color(1), self.bottom_corners[0].color(1),
               self.bottom_corners[0].color(2), self.bottom_sides[0].color(1), self.bottom_corners[1].color(1),
               self.bottom_corners[1].color(2), self.bottom_sides[1].color(1), self.bottom_corners[2].color(1),
               self.bottom_corners[2].color(2),

               self.bottom_corners[0].color(0), self.bottom_sides[0].color(0), self.bottom_corners[1].color(0),
               self.bottom_sides[3].color(0), Color::Orange, self.bottom_sides[1].color(0),
               self.bottom_corners[3].color(0), self.bottom_sides[2].color(0), self.bottom_corners[2].color(0),

               self.bottom_corners[3].color(1), self.bottom_sides[2].color(1), self.bottom_corners[2].color(2),
        )
    }
}

impl Cube {
    pub fn new() -> Self {
        Cube {
            bottom_sides: [
                Side::new(Color::Orange, Color::White),
                Side::new(Color::Orange, Color::Green),
                Side::new(Color::Orange, Color::Yellow),
                Side::new(Color::Orange, Color::Blue),
            ],
            bottom_corners: [
                Corner::new(Color::Orange, Color::Blue, Color::White),
                Corner::new(Color::Orange, Color::White, Color::Green),
                Corner::new(Color::Orange, Color::Green, Color::Yellow),
                Corner::new(Color::Orange, Color::Yellow, Color::Blue),
            ],
            middle_sides: [
                Side::new(Color::White, Color::Blue),
                Side::new(Color::Blue, Color::Yellow),
                Side::new(Color::Yellow, Color::Green),
                Side::new(Color::Green, Color::White),
            ],
            top_sides: [
                Side::new(Color::Red, Color::White),
                Side::new(Color::Red, Color::Blue),
                Side::new(Color::Red, Color::Yellow),
                Side::new(Color::Red, Color::Green),
            ],
            top_corners: [
                Corner::new(Color::Red, Color::White, Color::Blue),
                Corner::new(Color::Red, Color::Blue, Color::Yellow),
                Corner::new(Color::Red, Color::Yellow, Color::Green),
                Corner::new(Color::Red, Color::Green, Color::White),
            ],
        }
    }

    fn rotate_random_(&mut self, i: i32) {
        match i {
            0 => self.rotate_front_clockwise(),
            1 => self.rotate_front_counter_clockwise(),
            2 => self.rotate_right_clockwise(),
            3 => self.rotate_right_counter_clockwise(),
            4 => self.rotate_top_clockwise(),
            5 => self.rotate_top_counter_clockwise(),
            6 => self.rotate_left_clockwise(),
            7 => self.rotate_left_counter_clockwise(),
            8 => self.rotate_bottom_clockwise(),
            9 => self.rotate_bottom_counter_clockwise(),
            10 => self.rotate_back_clockwise(),
            11 => self.rotate_back_counter_clockwise(),
            _ => unreachable!(),
        }
    }

    pub fn shuffle(&mut self) {
        let mut rand = rand::thread_rng();
        for _ in 0..rand.gen_range(15, 21) {
            self.rotate_random_(rand.gen_range(0, 12));
        }
    }

    pub fn rotate_random(&mut self) {
        self.rotate_random_(rand::thread_rng().gen_range(0, 12));
    }

    pub fn rotate_right_clockwise(&mut self) {
        // sides
        // a, d
        mem::swap(&mut self.middle_sides[3], &mut self.bottom_sides[1]);
        // d, c
        mem::swap(&mut self.bottom_sides[1], &mut self.middle_sides[2]);
        // c, b
        mem::swap(&mut self.middle_sides[2], &mut self.top_sides[3]);

        // corners
        // a, d
        mem::swap(&mut self.top_corners[3], &mut self.bottom_corners[1]);
        // d, c
        self.bottom_corners.swap(1, 2);
        // c, b
        mem::swap(&mut self.top_corners[2], &mut self.bottom_corners[2]);

        // fix alignments
        self.middle_sides[3].flip();
        self.top_sides[3].flip();
        self.top_corners[3].counter_clockwise();
        self.top_corners[2].clockwise();
        self.bottom_corners[1].clockwise();
        self.bottom_corners[2].counter_clockwise();
    }

    pub fn rotate_right_counter_clockwise(&mut self) {
        // fix alignments
        self.middle_sides[3].flip();
        self.top_sides[3].flip();
        self.top_corners[3].clockwise();
        self.top_corners[2].counter_clockwise();
        self.bottom_corners[1].counter_clockwise();
        self.bottom_corners[2].clockwise();

        // sides
        // a, b
        mem::swap(&mut self.middle_sides[3], &mut self.top_sides[3]);
        // b, c
        mem::swap(&mut self.top_sides[3], &mut self.middle_sides[2]);
        // c, d
        mem::swap(&mut self.middle_sides[2], &mut self.bottom_sides[1]);

        // corners
        // a, b
        self.top_corners.swap(2, 3);
        // b, c
        mem::swap(&mut self.top_corners[2], &mut self.bottom_corners[2]);
        // c, d
        self.bottom_corners.swap(2, 1);
    }

    pub fn rotate_top_clockwise(&mut self) {
        self.top_sides.swap(0, 3);
        self.top_sides.swap(3, 2);
        self.top_sides.swap(2, 1);

        self.top_corners.swap(0, 3);
        self.top_corners.swap(3, 2);
        self.top_corners.swap(2, 1);
    }

    pub fn rotate_top_counter_clockwise(&mut self) {
        self.top_sides.swap(0, 1);
        self.top_sides.swap(1, 2);
        self.top_sides.swap(2, 3);

        self.top_corners.swap(0, 1);
        self.top_corners.swap(1, 2);
        self.top_corners.swap(2, 3);
    }

    pub fn rotate_front_clockwise(&mut self) {
        // a, d
        mem::swap(&mut self.top_corners[0], &mut self.bottom_corners[0]);
        // d, c
        self.bottom_corners.swap(0, 1);
        // c, b
        mem::swap(&mut self.bottom_corners[1], &mut self.top_corners[3]);

        // a, d
        mem::swap(&mut self.middle_sides[0], &mut self.bottom_sides[0]);
        // d, c
        mem::swap(&mut self.bottom_sides[0], &mut self.middle_sides[3]);
        // c, b
        mem::swap(&mut self.middle_sides[3], &mut self.top_sides[0]);

        self.bottom_corners[0].clockwise();
        self.bottom_corners[1].counter_clockwise();
        self.top_corners[0].counter_clockwise();
        self.top_corners[3].clockwise();

        self.middle_sides[0].flip();
        self.top_sides[0].flip();
    }

    pub fn rotate_front_counter_clockwise(&mut self) {
        self.bottom_corners[0].counter_clockwise();
        self.bottom_corners[1].clockwise();
        self.top_corners[0].clockwise();
        self.top_corners[3].counter_clockwise();

        self.middle_sides[0].flip();
        self.top_sides[0].flip();

        // a, b
        self.top_corners.swap(0, 3);
        // b, c
        mem::swap(&mut self.top_corners[3], &mut self.bottom_corners[1]);
        // c, d
        self.bottom_corners.swap(1, 0);

        // a, b
        mem::swap(&mut self.middle_sides[0], &mut self.top_sides[0]);
        // b, c
        mem::swap(&mut self.top_sides[0], &mut self.middle_sides[3]);
        // c, d
        mem::swap(&mut self.middle_sides[3], &mut self.bottom_sides[0]);
    }

    pub fn rotate_bottom_clockwise(&mut self) {
        self.bottom_sides.swap(0, 3);
        self.bottom_sides.swap(3, 2);
        self.bottom_sides.swap(2, 1);

        self.bottom_corners.swap(0, 3);
        self.bottom_corners.swap(3, 2);
        self.bottom_corners.swap(2, 1);
    }

    pub fn rotate_bottom_counter_clockwise(&mut self) {
        self.bottom_sides.swap(0, 1);
        self.bottom_sides.swap(1, 2);
        self.bottom_sides.swap(2, 3);

        self.bottom_corners.swap(0, 1);
        self.bottom_corners.swap(1, 2);
        self.bottom_corners.swap(2, 3);
    }

    pub fn rotate_left_clockwise(&mut self) {
        self.top_corners.swap(0, 1);
        mem::swap(&mut self.top_corners[1], &mut self.bottom_corners[3]);
        self.bottom_corners.swap(3, 0);

        mem::swap(&mut self.top_sides[1], &mut self.middle_sides[1]);
        mem::swap(&mut self.middle_sides[1], &mut self.bottom_sides[3]);
        mem::swap(&mut self.bottom_sides[3], &mut self.middle_sides[0]);

        self.bottom_corners[0].counter_clockwise();
        self.bottom_corners[3].clockwise();
        self.top_corners[0].clockwise();
        self.top_corners[1].counter_clockwise();

        self.middle_sides[1].flip();
        self.top_sides[1].flip();
    }

    pub fn rotate_left_counter_clockwise(&mut self) {
        self.bottom_corners[0].clockwise();
        self.bottom_corners[3].counter_clockwise();
        self.top_corners[0].counter_clockwise();
        self.top_corners[1].clockwise();

        self.middle_sides[1].flip();
        self.top_sides[1].flip();

        mem::swap(&mut self.top_corners[0], &mut self.bottom_corners[0]);
        self.bottom_corners.swap(0, 3);
        mem::swap(&mut self.bottom_corners[3], &mut self.top_corners[1]);

        mem::swap(&mut self.top_sides[1], &mut self.middle_sides[0]);
        mem::swap(&mut self.middle_sides[0], &mut self.bottom_sides[3]);
        mem::swap(&mut self.bottom_sides[3], &mut self.middle_sides[1]);
    }

    pub fn rotate_back_clockwise(&mut self) {
        self.top_corners.swap(1, 2);
        mem::swap(&mut self.top_corners[2], &mut self.bottom_corners[2]);
        self.bottom_corners.swap(2, 3);

        mem::swap(&mut self.top_sides[2], &mut self.middle_sides[2]);
        mem::swap(&mut self.middle_sides[2], &mut self.bottom_sides[2]);
        mem::swap(&mut self.bottom_sides[2], &mut self.middle_sides[1]);

        self.top_corners[1].clockwise();
        self.top_corners[2].counter_clockwise();
        self.bottom_corners[2].clockwise();
        self.bottom_corners[3].counter_clockwise();

        self.top_sides[2].flip();
        self.middle_sides[2].flip();
    }

    pub fn rotate_back_counter_clockwise(&mut self) {
        self.top_corners[1].counter_clockwise();
        self.top_corners[2].clockwise();
        self.bottom_corners[2].counter_clockwise();
        self.bottom_corners[3].clockwise();

        self.top_sides[2].flip();
        self.middle_sides[2].flip();

        self.top_corners.swap(2, 1);
        mem::swap(&mut self.top_corners[1], &mut self.bottom_corners[3]);
        self.bottom_corners.swap(3, 2);

        mem::swap(&mut self.top_sides[2], &mut self.middle_sides[1]);
        mem::swap(&mut self.middle_sides[1], &mut self.bottom_sides[2]);
        mem::swap(&mut self.bottom_sides[2], &mut self.middle_sides[2]);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        assert_is_solved(&Cube::new());
    }

    fn assert_is_solved(cube: &Cube) {
        assert_eq!(cube.bottom_sides,
                   [
                       Side::new(Color::Orange, Color::White),
                       Side::new(Color::Orange, Color::Green),
                       Side::new(Color::Orange, Color::Yellow),
                       Side::new(Color::Orange, Color::Blue),
                   ]);
        assert_eq!(cube.bottom_corners,
                   [
                       Corner::new(Color::Orange, Color::Blue, Color::White),
                       Corner::new(Color::Orange, Color::White, Color::Green),
                       Corner::new(Color::Orange, Color::Green, Color::Yellow),
                       Corner::new(Color::Orange, Color::Yellow, Color::Blue),
                   ]);
        assert_eq!(cube.middle_sides,
                   [
                       Side::new(Color::White, Color::Blue),
                       Side::new(Color::Blue, Color::Yellow),
                       Side::new(Color::Yellow, Color::Green),
                       Side::new(Color::Green, Color::White),
                   ]);
        assert_eq!(cube.top_sides,
                   [
                       Side::new(Color::Red, Color::White),
                       Side::new(Color::Red, Color::Blue),
                       Side::new(Color::Red, Color::Yellow),
                       Side::new(Color::Red, Color::Green),
                   ]);
        assert_eq!(cube.top_corners,
                   [
                       Corner::new(Color::Red, Color::White, Color::Blue),
                       Corner::new(Color::Red, Color::Blue, Color::Yellow),
                       Corner::new(Color::Red, Color::Yellow, Color::Green),
                       Corner::new(Color::Red, Color::Green, Color::White),
                   ]);
    }

    #[test]
    fn test_rotate_right_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_right_clockwise();
        assert_eq!(cube.bottom_sides,
                   [
                       Side::new(Color::Orange, Color::White),
                       Side::new(Color::Yellow, Color::Green),
                       Side::new(Color::Orange, Color::Yellow),
                       Side::new(Color::Orange, Color::Blue),
                   ]);
        assert_eq!(cube.bottom_corners,
                   [
                       Corner::new(Color::Orange, Color::Blue, Color::White),
                       Corner::new(Color::Yellow, Color::Orange, Color::Green),
                       Corner::new(Color::Yellow, Color::Green, Color::Red),
                       Corner::new(Color::Orange, Color::Yellow, Color::Blue),
                   ]);
        assert_eq!(cube.middle_sides,
                   [
                       Side::new(Color::White, Color::Blue),
                       Side::new(Color::Blue, Color::Yellow),
                       Side::new(Color::Red, Color::Green),
                       Side::new(Color::Green, Color::Orange),
                   ]);
        assert_eq!(cube.top_sides,
                   [
                       Side::new(Color::Red, Color::White),
                       Side::new(Color::Red, Color::Blue),
                       Side::new(Color::Red, Color::Yellow),
                       Side::new(Color::White, Color::Green),
                   ]);
        assert_eq!(cube.top_corners,
                   [
                       Corner::new(Color::Red, Color::White, Color::Blue),
                       Corner::new(Color::Red, Color::Blue, Color::Yellow),
                       Corner::new(Color::White, Color::Red, Color::Green),
                       Corner::new(Color::White, Color::Green, Color::Orange),
                   ]);
    }

    #[test]
    fn test_rotate_right_counter_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_right_clockwise();
        cube.rotate_right_counter_clockwise();
        assert_is_solved(&cube);
    }

    #[test]
    fn test_rotate_top_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_top_clockwise();
        assert_eq!(cube.bottom_sides,
                   [
                       Side::new(Color::Orange, Color::White),
                       Side::new(Color::Orange, Color::Green),
                       Side::new(Color::Orange, Color::Yellow),
                       Side::new(Color::Orange, Color::Blue),
                   ]);
        assert_eq!(cube.bottom_corners,
                   [
                       Corner::new(Color::Orange, Color::Blue, Color::White),
                       Corner::new(Color::Orange, Color::White, Color::Green),
                       Corner::new(Color::Orange, Color::Green, Color::Yellow),
                       Corner::new(Color::Orange, Color::Yellow, Color::Blue),
                   ]);
        assert_eq!(cube.middle_sides,
                   [
                       Side::new(Color::White, Color::Blue),
                       Side::new(Color::Blue, Color::Yellow),
                       Side::new(Color::Yellow, Color::Green),
                       Side::new(Color::Green, Color::White),
                   ]);
        assert_eq!(cube.top_sides,
                   [
                       Side::new(Color::Red, Color::Green),
                       Side::new(Color::Red, Color::White),
                       Side::new(Color::Red, Color::Blue),
                       Side::new(Color::Red, Color::Yellow),
                   ]);
        assert_eq!(cube.top_corners,
                   [
                       Corner::new(Color::Red, Color::Green, Color::White),
                       Corner::new(Color::Red, Color::White, Color::Blue),
                       Corner::new(Color::Red, Color::Blue, Color::Yellow),
                       Corner::new(Color::Red, Color::Yellow, Color::Green),
                   ]);
    }

    #[test]
    fn test_rotate_top_counter_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_top_clockwise();
        cube.rotate_top_counter_clockwise();
        assert_is_solved(&cube);
    }

    #[test]
    fn test_rotate_bottom_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_bottom_clockwise();
        assert_eq!(cube.bottom_sides,
                   [
                       Side::new(Color::Orange, Color::Blue),
                       Side::new(Color::Orange, Color::White),
                       Side::new(Color::Orange, Color::Green),
                       Side::new(Color::Orange, Color::Yellow),
                   ]);
        assert_eq!(cube.bottom_corners,
                   [
                       Corner::new(Color::Orange, Color::Yellow, Color::Blue),
                       Corner::new(Color::Orange, Color::Blue, Color::White),
                       Corner::new(Color::Orange, Color::White, Color::Green),
                       Corner::new(Color::Orange, Color::Green, Color::Yellow),
                   ]);
        assert_eq!(cube.middle_sides,
                   [
                       Side::new(Color::White, Color::Blue),
                       Side::new(Color::Blue, Color::Yellow),
                       Side::new(Color::Yellow, Color::Green),
                       Side::new(Color::Green, Color::White),
                   ]);
        assert_eq!(cube.top_sides,
                   [
                       Side::new(Color::Red, Color::White),
                       Side::new(Color::Red, Color::Blue),
                       Side::new(Color::Red, Color::Yellow),
                       Side::new(Color::Red, Color::Green),
                   ]);
        assert_eq!(cube.top_corners,
                   [
                       Corner::new(Color::Red, Color::White, Color::Blue),
                       Corner::new(Color::Red, Color::Blue, Color::Yellow),
                       Corner::new(Color::Red, Color::Yellow, Color::Green),
                       Corner::new(Color::Red, Color::Green, Color::White),
                   ]);
    }

    #[test]
    fn test_rotate_bottom_counter_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_bottom_clockwise();
        cube.rotate_bottom_counter_clockwise();
        assert_is_solved(&cube);
    }

    #[test]
    fn test_rotate_front_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_front_clockwise();
        assert_eq!(cube.bottom_sides,
                   [
                       Side::new(Color::Green, Color::White),
                       Side::new(Color::Orange, Color::Green),
                       Side::new(Color::Orange, Color::Yellow),
                       Side::new(Color::Orange, Color::Blue),
                   ]);
        assert_eq!(cube.bottom_corners,
                   [
                       Corner::new(Color::Green, Color::Orange, Color::White),
                       Corner::new(Color::Green, Color::White, Color::Red),
                       Corner::new(Color::Orange, Color::Green, Color::Yellow),
                       Corner::new(Color::Orange, Color::Yellow, Color::Blue),
                   ]);
        assert_eq!(cube.middle_sides,
                   [
                       Side::new(Color::White, Color::Orange),
                       Side::new(Color::Blue, Color::Yellow),
                       Side::new(Color::Yellow, Color::Green),
                       Side::new(Color::Red, Color::White),
                   ]);
        assert_eq!(cube.top_sides,
                   [
                       Side::new(Color::Blue, Color::White),
                       Side::new(Color::Red, Color::Blue),
                       Side::new(Color::Red, Color::Yellow),
                       Side::new(Color::Red, Color::Green),
                   ]);
        assert_eq!(cube.top_corners,
                   [
                       Corner::new(Color::Blue, Color::White, Color::Orange),
                       Corner::new(Color::Red, Color::Blue, Color::Yellow),
                       Corner::new(Color::Red, Color::Yellow, Color::Green),
                       Corner::new(Color::Blue, Color::Red, Color::White),
                   ]);
    }

    #[test]
    fn test_rotate_front_counter_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_front_clockwise();
        cube.rotate_front_counter_clockwise();
        assert_is_solved(&cube);
    }

    #[test]
    fn test_rotate_left_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_left_clockwise();
        assert_eq!(cube.bottom_sides,
                   [
                       Side::new(Color::Orange, Color::White),
                       Side::new(Color::Orange, Color::Green),
                       Side::new(Color::Orange, Color::Yellow),
                       Side::new(Color::White, Color::Blue),
                   ]);
        assert_eq!(cube.bottom_corners,
                   [
                       Corner::new(Color::White, Color::Blue, Color::Red),
                       Corner::new(Color::Orange, Color::White, Color::Green),
                       Corner::new(Color::Orange, Color::Green, Color::Yellow),
                       Corner::new(Color::White, Color::Orange, Color::Blue),
                   ]);
        assert_eq!(cube.middle_sides,
                   [
                       Side::new(Color::Red, Color::Blue),
                       Side::new(Color::Blue, Color::Orange),
                       Side::new(Color::Yellow, Color::Green),
                       Side::new(Color::Green, Color::White),
                   ]);
        assert_eq!(cube.top_sides,
                   [
                       Side::new(Color::Red, Color::White),
                       Side::new(Color::Yellow, Color::Blue),
                       Side::new(Color::Red, Color::Yellow),
                       Side::new(Color::Red, Color::Green),
                   ]);
        assert_eq!(cube.top_corners,
                   [
                       Corner::new(Color::Yellow, Color::Red, Color::Blue),
                       Corner::new(Color::Yellow, Color::Blue, Color::Orange),
                       Corner::new(Color::Red, Color::Yellow, Color::Green),
                       Corner::new(Color::Red, Color::Green, Color::White),
                   ]);
    }

    #[test]
    fn test_rotate_left_counter_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_left_clockwise();
        cube.rotate_left_counter_clockwise();
        assert_is_solved(&cube);
    }

    #[test]
    fn test_rotate_back_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_back_clockwise();
        assert_eq!(cube.bottom_sides,
                   [
                       Side::new(Color::Orange, Color::White),
                       Side::new(Color::Orange, Color::Green),
                       Side::new(Color::Blue, Color::Yellow),
                       Side::new(Color::Orange, Color::Blue),
                   ]);
        assert_eq!(cube.bottom_corners,
                   [
                       Corner::new(Color::Orange, Color::Blue, Color::White),
                       Corner::new(Color::Orange, Color::White, Color::Green),
                       Corner::new(Color::Blue, Color::Orange, Color::Yellow),
                       Corner::new(Color::Blue, Color::Yellow, Color::Red),
                   ]);
        assert_eq!(cube.middle_sides,
                   [
                       Side::new(Color::White, Color::Blue),
                       Side::new(Color::Red, Color::Yellow),
                       Side::new(Color::Yellow, Color::Orange),
                       Side::new(Color::Green, Color::White),
                   ]);
        assert_eq!(cube.top_sides,
                   [
                       Side::new(Color::Red, Color::White),
                       Side::new(Color::Red, Color::Blue),
                       Side::new(Color::Green, Color::Yellow),
                       Side::new(Color::Red, Color::Green),
                   ]);
        assert_eq!(cube.top_corners,
                   [
                       Corner::new(Color::Red, Color::White, Color::Blue),
                       Corner::new(Color::Green, Color::Red, Color::Yellow),
                       Corner::new(Color::Green, Color::Yellow, Color::Orange),
                       Corner::new(Color::Red, Color::Green, Color::White),
                   ]);
    }

    #[test]
    fn rotate_back_counter_clockwise() {
        let mut cube = Cube::new();
        cube.rotate_back_clockwise();
        cube.rotate_back_counter_clockwise();
        assert_is_solved(&cube);
    }

    #[test]
    fn test_display() {
        let mut cube = Cube::new();
        assert_eq!("
          Y Y Y

          R R R
          R R R
          R R R

Y  B B B  W W W  G G G  Y
Y  B B B  W W W  G G G  Y
Y  B B B  W W W  G G G  Y

          O O O
          O O O
          O O O

          Y Y Y",
                   format!("\n{}", cube));

        cube.rotate_front_clockwise();
        assert_eq!("
          Y Y Y

          R R R
          R R R
          B B B

Y  B B O  W W W  R G G  Y
Y  B B O  W W W  R G G  Y
Y  B B O  W W W  R G G  Y

          G G G
          O O O
          O O O

          Y Y Y",
                   format!("\n{}", cube));

        cube.rotate_right_clockwise();
        assert_eq!("
          Y Y B

          R R W
          R R W
          B B W

Y  B B O  W W G  R R R  B
Y  B B O  W W O  G G G  R
Y  B B O  W W O  G G G  R

          G G Y
          O O Y
          O O Y

          Y Y R",
                   format!("\n{}", cube));

        cube.rotate_left_clockwise();
        assert_eq!("
          O Y B

          Y R W
          Y R W
          Y B W

O  B B B  R W G  R R R  B
O  B B B  R W O  G G G  R
G  O O O  B W O  G G G  R

          W G Y
          W O Y
          W O Y

          G Y R",
                   format!("\n{}", cube));
    }
}
