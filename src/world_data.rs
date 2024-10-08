use std::error::Error;

use bevy::utils::HashMap;
use chunk::{Chunk, ChunkNotFoundError};
use voxel::Voxel;

use crate::coordinates::{ChunkCoord, GlobalCoord, LocalCoord};

mod chunk;
mod voxel;

#[derive(Default)]
pub struct World {
    data: HashMap<ChunkCoord, Chunk>,
}

impl World {
    pub const CHUNK_SIZE: u8 = 32;

    pub fn new() -> Self {
        World {
            data: HashMap::default(),
        }
    }

    pub fn get_voxel(&self, global_position: &GlobalCoord) -> Result<&Voxel, Box<dyn Error>> {
        let chunk_position = ChunkCoord::from(global_position.to_owned());
        let local_position = LocalCoord::from(global_position.to_owned());

        match self.data.get(&chunk_position) {
            Some(chunk) => Ok(chunk.get_voxel(&local_position)?),
            None => Err(Box::new(ChunkNotFoundError(chunk_position))),
        }
    }

    pub fn set_voxel(
        &mut self,
        global_position: &GlobalCoord,
        voxel: Voxel,
    ) -> Result<(), Box<dyn Error>> {
        let chunk_position = ChunkCoord::from(global_position.to_owned());
        let local_position = LocalCoord::from(global_position.to_owned());

        match self.data.get_mut(&chunk_position) {
            Some(chunk) => chunk.set_voxel(&local_position, voxel)?,
            None => return Err(Box::new(ChunkNotFoundError(chunk_position))),
        }

        Ok(())
    }

    fn create_chunk(&mut self, chunk_position: &ChunkCoord) {
        // Generate chunk data here!
        let new_chunk = Chunk::new();
        self.set_chunk(chunk_position, new_chunk);
    }

    fn set_chunk(&mut self, chunk_position: &ChunkCoord, chunk: Chunk) {
        let _ = self.data.insert(chunk_position.to_owned(), chunk);
    }
}

#[cfg(test)]
mod tests {
    use quickcheck::quickcheck;

    use super::*;
    use crate::coordinates::*;

    quickcheck! {
        fn create_chunk_at(random_x: ChunkCoordType, random_y: ChunkCoordType, random_z: ChunkCoordType) -> bool {
            let mut world = World::new();
            let position = ChunkCoord::new(random_x, random_y, random_z);
            world.create_chunk(&position);
            world.data.get(&position).is_some()
        }
    }

    quickcheck! {
        fn set_chunk(random_x: ChunkCoordType, random_y: ChunkCoordType, random_z: ChunkCoordType) -> bool {
            let mut world = World::new();
            let position = ChunkCoord::new(random_x, random_y, random_z);
            let new_chunk = Chunk::new();
            world.set_chunk(&position, new_chunk);
            world.data.get(&position).is_some()
        }
    }

    quickcheck! {
        fn get_voxel_empty_world(random_x: GlobalCoordType, random_y: GlobalCoordType, random_z: GlobalCoordType) -> bool {
            let world = World::new();
            let position = GlobalCoord::new(random_x, random_y, random_z);
            world.get_voxel(&position).is_err()
        }
    }

    #[test]
    fn get_existing_voxel() {
        let mut world = World::new();
        world.create_chunk(&ChunkCoord::new(0, 0, 0));
        let position = GlobalCoord::new(1, 5, 3);
        let voxel_exists = world.get_voxel(&position).is_ok();
        assert!(voxel_exists);
    }

    quickcheck! {
        fn set_voxel_empty_world(random_x: GlobalCoordType, random_y: GlobalCoordType, random_z: GlobalCoordType) -> bool {
            let mut world = World::new();
            let position = GlobalCoord::new(random_x, random_y, random_z);
            let new_voxel = Voxel::new(0);
            world.set_voxel(&position, new_voxel).is_err()
        }
    }

    #[test]
    fn set_then_get_voxel() {
        let mut world = World::new();
        world.create_chunk(&ChunkCoord::new(0, 0, 0));
        let position = GlobalCoord::new(1, 5, 3);
        world.set_voxel(&position, Voxel::new(0)).unwrap();
        let voxel_exists = world.get_voxel(&position).is_ok();
        assert!(voxel_exists);
    }
}
