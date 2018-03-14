extern crate rand;

mod color;
mod cube;
mod corner;
mod side;
mod predicates;
use cube::Cube;
use std::io;

fn main() {
    let mut cube = Cube::new();
    let mut input = String::new();
    loop {
        input.clear();
        println!("{}", cube);
        io::stdin().read_line(&mut input).unwrap();
        let mut iter = input.chars();
        let mut c = match iter.next() {
            Some(c) => c,
            None => break,
        };
        loop {
            c = c.to_uppercase().next().unwrap();
            if c.is_whitespace() {
            } else if c == 'R' {
                match iter.next() {
                    Some('\'') => cube.rotate_right_counter_clockwise(),
                    Some(ch) => {
                        cube.rotate_right_clockwise();
                        c = ch;
                        continue;
                    },
                    None => {
                        cube.rotate_right_clockwise();
                        break;
                    },
                }
            } else if c == 'U' {
                match iter.next() {
                    Some('\'') => cube.rotate_top_counter_clockwise(),
                    Some(ch) => {
                        cube.rotate_top_clockwise();
                        c = ch;
                        continue;
                    },
                    None => {
                        cube.rotate_top_clockwise();
                        break;
                    },
                }
            } else if c == 'F' {
                match iter.next() {
                    Some('\'') => cube.rotate_front_counter_clockwise(),
                    Some(ch) => {
                        cube.rotate_front_clockwise();
                        c = ch;
                        continue;
                    },
                    None => {
                        cube.rotate_front_clockwise();
                        break;
                    },
                }
            } else if c == 'L' {
                match iter.next() {
                    Some('\'') => cube.rotate_left_counter_clockwise(),
                    Some(ch) => {
                        cube.rotate_left_clockwise();
                        c = ch;
                        continue;
                    },
                    None => {
                        cube.rotate_left_clockwise();
                        break;
                    },
                }
            } else if c == 'D' {
                match iter.next() {
                    Some('\'') => cube.rotate_bottom_counter_clockwise(),
                    Some(ch) => {
                        cube.rotate_bottom_clockwise();
                        c = ch;
                        continue;
                    },
                    None => {
                        cube.rotate_bottom_clockwise();
                        break;
                    },
                }
            } else if c == 'B' {
                match iter.next() {
                    Some('\'') => cube.rotate_back_counter_clockwise(),
                    Some(ch) => {
                        cube.rotate_back_clockwise();
                        c = ch;
                        continue;
                    },
                    None => {
                        cube.rotate_back_clockwise();
                        break;
                    },
                }
            } else if c == '!' {
                return;
            } else if c == '#' {
                cube = Cube::new();
            } else if c == '*' {
                cube.shuffle();
            } else {
                panic!("Invalid character, {}", c);
            }
            c = match iter.next() {
                Some(c) => c,
                None => break,
            };
        }
    }
}
