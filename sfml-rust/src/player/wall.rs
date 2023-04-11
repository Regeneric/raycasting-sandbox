use sfml::{graphics::{RenderWindow, RectangleShape, Color, Shape, Transformable, RenderTarget}, system::Vector2f};

pub struct Wall {
    pub cell: i32, 
    pub width: i32, 
    pub height: i32, 

    pub grid: Vec<i32>,
}
impl Wall {
    pub fn new(c: i32, w: i32, h: i32) -> Self {
        Wall {
            // 1D array for easy indexing without nested loops
            grid: Vec::from([1,1,1,1,1,1,1,1,
                             4,0,1,0,0,0,0,2,
                             4,0,1,0,0,0,0,2,
                             4,0,1,0,0,2,2,2,
                             4,0,0,0,0,0,0,2,
                             4,0,4,0,0,3,0,2,
                             4,0,0,0,0,0,0,2,
                             3,3,3,3,3,3,3,3]),
            width: w, 
            height: h,
            cell: c
        }
    }

    pub fn draw(&self, window: &mut RenderWindow) {
        for y in 0..self.height {
            for x in 0..self.width {
                let mut wall = RectangleShape::new();
                if self.grid[(y * self.width + x) as usize] > 0 {wall.set_fill_color(Color::BLACK);}
                else {wall.set_fill_color(Color::WHITE);}
                
                let xs = (x * self.cell) as f32;
                let ys = (y * self.cell) as f32;

                wall.set_size(Vector2f::new((self.cell-1) as f32, (self.cell-1) as f32));
                wall.set_position(Vector2f::new(xs, ys));

                window.draw(&wall);
            }
        }
    }
}