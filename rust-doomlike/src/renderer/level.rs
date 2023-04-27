use serde::{Deserialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use super::Wall;
use super::Sector;
use super::RENDER_W;

#[derive(Deserialize, Debug)]
pub struct Level {
    sectors: i32,
    sectors_data: Vec<Sector>,

    walls: i32,
    walls_data: Vec<Wall>,
}
impl Level {
    fn data_loader() -> Result<Level, Box<dyn Error>> {
        let file = File::open("src/levels/level.json")?;
        let reader = BufReader::new(file);
        let mut level: Level = serde_json::from_reader(reader).expect("Bad JSON file");
        
        for sector in level.sectors_data.iter_mut() {
            sector.surf_arr = vec![0; RENDER_W as usize];
        }

        Ok(level)
    }

    pub fn level_loader() -> (i32, i32, Vec<Sector>, Vec<Wall>) {
        let level = Self::data_loader().unwrap();
        (level.sectors, level.walls,  level.sectors_data, level.walls_data)
    }
}