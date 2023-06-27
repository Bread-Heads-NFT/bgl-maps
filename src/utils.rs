use crate::entity::Entity;

#[derive(Debug, Clone)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: usize,
    pub y: usize,
}

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Movement {
    pub distance: usize,
    pub direction: Direction,
}

#[derive(Debug, Clone)]
pub enum MoveResult<'a> {
    Success,
    Failure,
    Collision(Vec<&'a Entity>),
}
