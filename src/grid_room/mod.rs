use std::collections::BTreeMap;

use grid::Grid;

use crate::{
    entity::Entity,
    error::MapError,
    utils::{MoveResult, Movement, Position, Size},
};

#[derive(Debug, Clone)]
pub struct GridRoom<'a> {
    pub name: String,
    pub grid: Grid<Option<&'a Entity>>,
    pub mobile_entities: BTreeMap<String, Position>,
}

impl<'a> GridRoom<'a> {
    pub fn new(name: String, size: Size) -> Self {
        Self {
            name,
            grid: Grid::new(size.height, size.width),
            mobile_entities: BTreeMap::new(),
        }
    }

    pub fn add_entity(
        &mut self,
        entity: &'a Entity,
        position: Position,
        is_static: bool,
    ) -> Result<(), MapError> {
        match self.grid.get_mut(position.y, position.x) {
            Some(cell) => {
                *cell = Some(entity);
            }
            None => return Err(MapError::OutOfBounds),
        };

        if !is_static {
            self.mobile_entities.insert(entity.name.clone(), position);
        }

        Ok(())
    }

    pub fn move_entity(
        &mut self,
        entity: &'a Entity,
        movement: Movement,
    ) -> Result<MoveResult, MapError> {
        let self_clone = self.clone();
        let start = self_clone
            .mobile_entities
            .get(&entity.name)
            .ok_or(MapError::OutOfBounds)?;

        let (end, traversed_tiles) = match movement.direction {
            crate::utils::Direction::Up => (
                Position {
                    x: start.x,
                    y: start.y + movement.distance,
                },
                (start.y..start.y + movement.distance)
                    .map(|y| Position {
                        x: start.x,
                        y: y + 1,
                    })
                    .collect::<Vec<Position>>(),
            ),
            crate::utils::Direction::Down => (
                Position {
                    x: start.x,
                    y: start.y - movement.distance,
                },
                ((start.y - movement.distance)..start.y)
                    .rev()
                    .map(|y| Position { x: start.x, y })
                    .collect(),
            ),
            crate::utils::Direction::Left => (
                Position {
                    x: start.x - movement.distance,
                    y: start.y,
                },
                ((start.x - movement.distance)..start.x)
                    .rev()
                    .map(|x| Position { x, y: start.y })
                    .collect(),
            ),
            crate::utils::Direction::Right => (
                Position {
                    x: start.x + movement.distance,
                    y: start.y,
                },
                (start.x..start.y + movement.distance)
                    .map(|x| Position {
                        x: x + 1,
                        y: start.y,
                    })
                    .collect(),
            ),
        };

        // println!("Start: {:#?}", start);
        // println!("End: {:#?}", end);
        // println!("Traversed tiles: {:#?}", traversed_tiles);

        for tile_index in 0..(traversed_tiles.len()) {
            let tile = &traversed_tiles[tile_index];
            let cloned_self = self.clone();
            match cloned_self.grid.get(tile.y, tile.x) {
                Some(cell) => {
                    if cell.is_some() {
                        println!("Collision!");
                        let resolved_tile = &traversed_tiles[tile_index - 1];
                        self.swap(start, resolved_tile)?;
                        return Ok(MoveResult::Collision(vec![entity, cell.unwrap()]));
                    }
                }
                None => {
                    if tile_index == 0 {
                        return Err(MapError::OutOfBounds);
                    } else {
                        let resolved_tile = &traversed_tiles[tile_index - 1];
                        self.swap(start, resolved_tile)?;
                        return Ok(MoveResult::Failure);
                    }
                }
            };
        }

        self.swap(start, &end)?;

        Ok(MoveResult::Success)
    }

    fn swap(&mut self, start: &Position, end: &Position) -> Result<(), MapError> {
        let entity = self.grid.get(start.y, start.x).unwrap().unwrap();
        match self.grid.get_mut(end.y, end.x) {
            Some(cell) => {
                *cell = Some(entity);
            }
            None => return Err(MapError::OutOfBounds),
        };

        match self.grid.get_mut(start.y, start.x) {
            Some(cell) => {
                *cell = None;
            }
            None => return Err(MapError::OutOfBounds),
        };

        self.mobile_entities
            .insert(entity.name.clone(), end.to_owned());

        Ok(())
    }
}

#[cfg(test)]
mod grid_room_tests {
    use crate::{
        entity::Entity,
        utils::{Direction, Movement, Position, Size},
    };

    use super::GridRoom;

    #[test]
    fn new() {
        let grid = GridRoom::new(
            "Test".to_string(),
            Size {
                width: 3,
                height: 3,
            },
        );
        println!("{:#?}", grid)
    }

    #[test]
    fn add_entity() {
        let mut grid = GridRoom::new(
            "Test".to_string(),
            Size {
                width: 3,
                height: 3,
            },
        );

        let entity = Entity {
            name: "Test".to_owned(),
        };

        grid.add_entity(&entity, Position { x: 1, y: 1 }, false)
            .unwrap();
        println!("{:#?}", grid)
    }

    #[test]
    fn move_entity() {
        let mut grid = GridRoom::new(
            "Test".to_string(),
            Size {
                width: 5,
                height: 5,
            },
        );

        let entity = Entity {
            name: "Test".to_owned(),
        };

        grid.add_entity(&entity, Position { x: 2, y: 2 }, false)
            .unwrap();

        grid.move_entity(
            &entity,
            Movement {
                distance: 2,
                direction: Direction::Up,
            },
        )
        .unwrap();
        println!("{:#?}", grid);

        grid.move_entity(
            &entity,
            Movement {
                distance: 2,
                direction: Direction::Down,
            },
        )
        .unwrap();
        println!("{:#?}", grid);
    }

    #[test]
    fn move_entity_fail() {
        let mut grid = GridRoom::new(
            "Test".to_string(),
            Size {
                width: 5,
                height: 5,
            },
        );

        let entity = Entity {
            name: "Test".to_owned(),
        };

        grid.add_entity(&entity, Position { x: 2, y: 2 }, false)
            .unwrap();

        let result = grid
            .move_entity(
                &entity,
                Movement {
                    distance: 3,
                    direction: Direction::Up,
                },
            )
            .unwrap();
        println!("{:#?}", result);
        println!("{:#?}", grid);
    }

    #[test]
    fn move_entity_collision() {
        let mut grid = GridRoom::new(
            "Test".to_string(),
            Size {
                width: 5,
                height: 5,
            },
        );

        let wall: Entity = Entity {
            name: "Bread Bandit".to_owned(),
        };

        grid.add_entity(&wall, Position { x: 2, y: 4 }, false)
            .unwrap();

        let entity = Entity {
            name: "Bread Cowboy".to_owned(),
        };

        grid.add_entity(&entity, Position { x: 2, y: 2 }, false)
            .unwrap();
        println!("{:#?}", grid);

        let result = grid
            .move_entity(
                &entity,
                Movement {
                    distance: 2,
                    direction: Direction::Up,
                },
            )
            .unwrap();
        println!("{:#?}", result);
        println!("{:#?}", grid.grid);
    }
}
