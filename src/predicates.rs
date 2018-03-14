use cube::Cube;
use corner::Corner;
use side::Side;
use color::Color;

pub fn num_bottom_solved(cube: &Cube) -> usize {
    8 - [cube.bottom_corners[0] == Corner::new(Color::Orange, Color::Blue, Color::White),
         cube.bottom_corners[1] == Corner::new(Color::Orange, Color::White, Color::Green),
         cube.bottom_corners[2] == Corner::new(Color::Orange, Color::Green, Color::Yellow),
         cube.bottom_corners[3] == Corner::new(Color::Orange, Color::Yellow, Color::Blue),
         cube.bottom_sides[0]   == Side::new(Color::Orange, Color::White),
         cube.bottom_sides[1]   == Side::new(Color::Orange, Color::Green),
         cube.bottom_sides[2]   == Side::new(Color::Orange, Color::Yellow),
         cube.bottom_sides[3]   == Side::new(Color::Orange, Color::Blue)]
        .into_iter().filter(|x| **x).count()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_num_bottom_solved() {
        let mut cube = Cube::new();
        assert_eq!(0, num_bottom_solved(&cube));
        cube.rotate_top_clockwise();
        assert_eq!(0, num_bottom_solved(&cube));
        cube.rotate_front_clockwise();
        assert_eq!(3, num_bottom_solved(&cube));
        cube.rotate_right_clockwise();
        assert_eq!(5, num_bottom_solved(&cube));
        cube.rotate_top_clockwise();
        assert_eq!(5, num_bottom_solved(&cube));
        cube.rotate_right_counter_clockwise();
        assert_eq!(3, num_bottom_solved(&cube));
        cube.rotate_top_counter_clockwise();
        assert_eq!(3, num_bottom_solved(&cube));
        cube.rotate_front_counter_clockwise();
        assert_eq!(0, num_bottom_solved(&cube));
    }
}
