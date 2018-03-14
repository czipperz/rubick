#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Direction {
    Front,
    Back,
    Left,
    Right,
    Top,
    Bottom,
    Middle,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Times {
    Clockwise,
    CounterClockwise,
    Double,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Movement {
    pub direction: Direction,
    pub times: Times,
}
