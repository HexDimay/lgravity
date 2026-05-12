use std::sync::Arc;

use winit::window::Window;

pub struct State {
    window: Arc<Window>
}

impl State {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Self> {
        Ok(Self {
            window
        })
    }

    pub fn resize(&mut self, _width: u32, _height: u32) {
        let _size = self.window.inner_size();
        
    }

    pub fn update(&mut self) {
        
    }

    pub fn render(&mut self) {
        self.window.request_redraw();

        
    }
}
