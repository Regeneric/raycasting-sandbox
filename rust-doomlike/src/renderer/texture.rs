use sfml::graphics::RenderWindow;

use serde::{Deserialize};

use std::error::Error;
use std::fs::File;
use std::io::BufReader;

use super::pixel;


#[derive(Deserialize, Debug)]
pub struct Texture {
    // Wall start and end
    pub width: i32, 
    pub height: i32, 

    pub name: String,
    pub data: Vec<u8>,
}
impl Texture {
    pub fn new(w: i32, h: i32, n: String) -> Self {
        // Data loaded from JSON
        Texture{
            width: 0,
            height: 0,
            name: "".to_string(),
            data: vec![0; 1024],
        }
    }

    pub fn test_textures(t: i32, texture: &Vec<Texture>, window: &mut RenderWindow) {
        for y in 0..texture[t as usize].height {
            for x in 0..texture[t as usize].width {
                let p = ((y*3) * texture[t as usize].width + (x*3)) as usize;
                    let r: u8 = texture[t as usize].data[p+0];
                    let g: u8 = texture[t as usize].data[p+1];
                    let b: u8 = texture[t as usize].data[p+2];
                pixel(x as f32, y as f32,  r,g,b,  true, window);
            }
        }
    }

    pub fn texture_loader() -> Vec<Texture> {
        Self::data_loader().unwrap()
    }

    fn data_loader() -> Result<Vec<Texture>, Box<dyn Error>> {
        let mut textures: Vec<Texture> = Vec::new();
        
        for t in 0..2 {
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