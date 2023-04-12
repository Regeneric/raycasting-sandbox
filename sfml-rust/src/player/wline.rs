use sfml::{graphics::{Vertex, Color, RenderWindow, RenderTarget, VertexBuffer, PrimitiveType, VertexBufferUsage}, system::{Vector2f}};

pub struct WideLine {
    verticies: VertexBuffer,
    _thicnkess: f32,
    _color: Color
}
impl WideLine {
    pub fn new(p1: Vector2f, p2: Vector2f, t: f32, c: Color) -> Self {
        let direction = p2-p1;
        let unit_direction = direction/f32::sqrt(direction.x*direction.x + direction.y*direction.y);
        let unit_perp = Vector2f::new(-unit_direction.y, unit_direction.x);

        let mut offset = Vector2f::new(0.0, 0.0);
            offset.x = (t/2.0) * unit_perp.x;
            offset.y = (t/2.0) * unit_perp.y;

        let mut verts: [Vertex; 4] = Default::default();
            verts[0 as usize].position = p1 + offset;
            verts[1 as usize].position = p2 + offset;
            verts[2 as usize].position = p2 - offset;
            verts[3 as usize].position = p1 - offset;

        let mut vert_buf = VertexBuffer::new(PrimitiveType::QUADS, 4, VertexBufferUsage::STREAM);
        for vert in verts.iter_mut() {vert.color = c;}

        vert_buf.update(&verts, 0);
        WideLine {
            verticies: vert_buf, 
            _thicnkess: t, 
            _color: c 
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