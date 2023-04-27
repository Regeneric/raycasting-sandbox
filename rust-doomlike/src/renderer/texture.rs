use serde::{Deserialize};

use std::error::Error;
use std::fs::{File, read_dir};
use std::io::BufReader;


#[derive(Deserialize, Debug)]
pub struct Texture {
    // Wall start and end
    pub width: i32, 
    pub height: i32, 

    pub name: String,
    pub data: Vec<u8>,
}
impl Texture {
    pub fn texture_loader() -> Vec<Texture> {
        Self::data_loader().unwrap()
    }

    fn data_loader() -> Result<Vec<Texture>, Box<dyn Error>> {
        let mut textures: Vec<Texture> = Vec::new();
        let all_files = read_dir("src/textures/").unwrap();
        let iter_to = all_files.count();

        for t in 0..iter_to/2 {
            let path = "src/textures/T";
            let num = t.to_string();
            let ext = ".json";
            let full_path = [path, &num, ext].join("");

            let file = File::open(full_path)?;
            let reader = BufReader::new(file);
            let texture: Texture = serde_json::from_reader(reader).expect("Bad JSON file");
            textures.push(texture);
        }

        Ok(textures)
    }
}