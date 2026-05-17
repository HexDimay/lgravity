use std::{cell::RefCell, rc::Rc, sync::Arc};

use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, MouseScrollDelta, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::PhysicalKey,
    window::Window,
};

use crate::{
    components::{camera::Camera, world::World},
    render::{RenderCamera, RenderWorld},
    state::State,
};

pub struct App {
    world: World,
    world_render_data: Option<RenderWorld>,
    camera: Rc<RefCell<Camera>>,
    camera_render_data: Option<RenderCamera>,
    pub state: Option<State>,
}

impl App {
    pub fn new() -> Self {
        Self {
            world: World::new(100, 100),
            world_render_data: None,
            camera: Rc::new(RefCell::new(Camera::new(100.0, [0.0, 0.0], [0.0, 0.0]))),
            camera_render_data: None,
            state: None,
        }
    }
}

impl ApplicationHandler<State> for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        self.world.randomize_mass(1000.0..1000.1);

        let window_attributes = Window::default_attributes();
        let window = Arc::new(event_loop.create_window(window_attributes).unwrap());
        let size = window.inner_size();
        self.state = Some(pollster::block_on(State::new(window)).unwrap());
        let device = &self.state.as_ref().unwrap().device;

        self.camera
            .borrow_mut()
            .set_screen_size([size.width as f32, size.height as f32]);
        self.camera_render_data = Some(
            RenderCamera::new(self.camera.clone())
                .create_uniform_buffer(device)
                .create_bind_group_layout(device)
                .create_bind_group(device),
        );

        self.world_render_data = Some(
            RenderWorld::new(
                &self.state.as_ref().unwrap().config,
                device,
                self.world.width() * self.world.height(),
                self.camera_render_data.as_ref().unwrap(),
            )
            .unwrap(),
        );
        self.world_render_data
            .as_mut()
            .unwrap()
            .init_data_world(&self.world);
        self.world_render_data
            .as_mut()
            .unwrap()
            .create_vertex_buffer(device);
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, event: State) {
        self.state = Some(event);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let state = match &mut self.state {
            Some(canvas) => canvas,
            None => return,
        };

        match event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                self.camera
                    .borrow_mut()
                    .set_screen_size([size.width as f32, size.height as f32]);
                state.resize(
                    size.width,
                    size.height,
                    self.camera_render_data.as_ref().unwrap(),
                );
            }
            WindowEvent::RedrawRequested => {
                state.update();
                crate::logic::update_world(&mut self.world);
                self.world_render_data.as_mut().unwrap().update_data(&self.world);
                match state.render(
                    self.world_render_data.as_ref().unwrap(),
                    self.camera_render_data.as_ref().unwrap(),
                ) {
                    Ok(_) => {}
                    Err(e) => {
                        log::error!("{e}");
                        event_loop.exit();
                    }
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        physical_key: PhysicalKey::Code(code),
                        state: key_state,
                        ..
                    },
                ..
            } => {
                state.handle_key(event_loop, code, key_state.is_pressed());
                if self.camera.borrow_mut().handle_input_key(code, key_state.is_pressed()) {
                    state.update_camera_buffer(self.camera_render_data.as_ref().unwrap());
                }
                
                log::info!("Camera pos: {:?}", self.camera.borrow().position());
            },
            WindowEvent::MouseWheel { delta, .. } => {
                if let MouseScrollDelta::LineDelta(y0, y1) = delta {
                    self.camera.borrow_mut().scale += y0 - y1;
                    if self.camera.borrow_mut().scale < 0.005 {
                        self.camera.borrow_mut().scale = 0.005;
                    }
                    log::info!("Camera scale: {:?}", self.camera.borrow().scale);
                    
                    state.update_camera_buffer(self.camera_render_data.as_ref().unwrap());
                }
            },
            _ => {}
        }
    }
}
