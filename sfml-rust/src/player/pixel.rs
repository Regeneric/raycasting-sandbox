use sfml::{graphics::{Vertex, Color, RenderWindow, RenderTarget, VertexBuffer, PrimitiveType, VertexBufferUsage}, system::{Vector2f}};

pub struct Pixel {
    pub verts: [Vertex; 4],
    pub verticies: VertexBuffer,
    _thicnkess: f32,
    _color: Color
}
impl Pixel {
    pub fn new(pos: Vector2f, thick: f32, col: Color) -> Self {
        let mut vt: [Vertex; 4] = Default::default();
            vt[0 as usize].position = pos;
            vt[1 as usize].position = Vector2f::new(pos.x + thick, pos.y);
            vt[2 as usize].position = Vector2f::new(pos.x + thick, pos.y + thick);
            vt[3 as usize].position = Vector2f::new(pos.x, pos.y + thick);

        let mut vert_buf = VertexBuffer::new(PrimitiveType::QUADS, 4, VertexBufferUsage::STREAM);
        for vert in vt.iter_mut() {vert.color = col;}

        vert_buf.update(&vt, 0);
        Pixel {
            verts: vt,
            verticies: vert_buf, 
            _thicnkess: thick, 
            _color: col 
        }
    }


    // What the fuck is `dyn`??
    // pub fn draw(&self, target: &mut dyn RenderTarget, states: &mut RenderStates) -> () {
    //     target.draw(&self.verticies);
    // }

    pub fn draw(&self, window: &mut RenderWindow) -> () {
        window.draw(&self.verticies);
    }

    // I think I need methods to do this here - it's a GPU VertexBuffer that needs to be modified
    // pub fn color() {}
    // pub fn size() {}
}