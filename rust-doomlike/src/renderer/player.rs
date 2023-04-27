use sfml::{
    system::{Vector3i},
    window::{Key}
};


pub struct Player {
    pub pos: Vector3i,
    pub angle: i32,
    pub look_up_down: i32,

    pub cos: [f32; 360],
    pub sin: [f32; 360],
}

impl Player {
    pub fn new() -> Self {
        // Stored sine and cosine values 
        let mut cos_buff: [f32; 360] = [0.0; 360];
        let mut sin_buff: [f32; 360] = [0.0; 360];
        for x in 0..360 {
            // Converting to radians
            cos_buff[x] = f32::cos(f32::to_radians(x as f32));
            sin_buff[x] = f32::sin(f32::to_radians(x as f32));
        }

        Player {
            pos: Vector3i::new(-37, -190, 5),   // X - left/right   Y - close/far   Z - up/down
            angle: 25,
            look_up_down: 0,
            cos: cos_buff,
            sin: sin_buff,
        }
    }


    pub fn advance(&mut self, key: Key, velocity: i32) -> () {
        // I can do `self.angle as usize` as well but I'm trying to be carefull
        let delta_x = (self.sin[usize::try_from(self.angle).unwrap()] * velocity as f32) as i32;
        let delta_y = (self.cos[usize::try_from(self.angle).unwrap()] * velocity as f32) as i32;

        match key {
            // Move forward/backward and rotate
            Key::W => {
                self.pos.x = self.pos.x + delta_x as i32;
                self.pos.y = self.pos.y + delta_y as i32;
            },
            Key::S => {
                self.pos.x = self.pos.x - delta_x as i32;
                self.pos.y = self.pos.y - delta_y as i32;
            },
            Key::A => {
                self.angle = self.angle + 4;
                if self.angle > 359 {self.angle = self.angle - 360;}
            },
            Key::D => {
                self.angle = self.angle - 4;
                if self.angle <   0 {self.angle = self.angle + 360;}
            },

            // Move up and down in Z axis
            Key::E  => self.pos.z = self.pos.z - 4,
            Key::Q  => self.pos.z = self.pos.z + 4,

            Key::Down   => self.look_up_down = self.look_up_down + 1,
            Key::Up => self.look_up_down = self.look_up_down - 1,
            
            // Strafe right/left
            Key::Left => {
                self.pos.x = self.pos.x + delta_y as i32;
                self.pos.y = self.pos.y - delta_x as i32;
            },
            Key::Right  => {
                self.pos.x = self.pos.x - delta_y as i32;
                self.pos.y = self.pos.y + delta_x as i32;
            },

            _ => return
        }
    }
}