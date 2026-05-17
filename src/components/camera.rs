use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Debug, Clone, Copy, Zeroable, Pod)]
pub struct Camera {
    scale: f32,
    _padding: [u8; 4],
    position: [f32; 2],
    screen_size: [f32; 2],
}

impl Camera {
    pub fn new(scale: f32, position: [f32; 2], screen_size: [f32; 2]) -> Self {
        Self {
            scale,
            _padding: [0; 4],
            position,
            screen_size,
        }
    }

    pub fn scale(&self) -> f32 {
        self.scale
    }

    pub fn position(&self) -> [f32; 2] {
        self.position
    }

    /// [width, height]
    pub fn screen_size(&self) -> [f32; 2] {
        self.screen_size
    }

    pub fn get_mut_position(&mut self) -> &mut [f32; 2] {
        &mut self.position
    }

    pub fn get_mut_scale(&mut self) -> &mut f32 {
        &mut self.scale
    }

    pub fn set_screen_size(&mut self, screen_size: [f32; 2]) {
        self.screen_size = screen_size;
    }
}
