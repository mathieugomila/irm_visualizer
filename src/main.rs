use std::time::Instant;

use cgmath::Vector3;
use glutin::{
    dpi::PhysicalSize,
    event::{DeviceEvent, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
    Api, ContextBuilder, ContextWrapper, GlRequest, PossiblyCurrent,
};
use graph::draw::Drawer;
use io::input_player::InputManager;
use player::player::Player;
use world::world_data::WorldData;

mod graph;
mod io;
mod player;
mod world;

fn main() {
    unsafe {
        let event_loop = EventLoop::new();
        let mut game = Game::new(&event_loop);
        game.load_content();
        // Infinite loop of the code
        event_loop.run(move |event, _, control_flow| {
            *control_flow = game.update();

            match event {
                Event::LoopDestroyed => (),
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(physical_size) => game.gl_context.resize(physical_size),
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: glutin::event::ElementState::Pressed,
                                ..
                            },
                        ..
                    } => {
                        game.input_manager.key_event_pressed(key);
                    }
                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                virtual_keycode: Some(key),
                                state: glutin::event::ElementState::Released,
                                ..
                            },
                        ..
                    } => {
                        game.input_manager.key_event_released(key);
                    }
                    _ => (),
                },
                Event::DeviceEvent { event, .. } => match event {
                    DeviceEvent::MouseMotion { delta } => game.input_manager.update_mouse(delta),
                    _ => {}
                },
                Event::MainEventsCleared => {
                    game.draw();
                }
                Event::RedrawRequested(_) => {
                    game.draw();
                }
                _ => (),
            }
        });
    }
}

struct Game {
    gl_context: ContextWrapper<PossiblyCurrent, Window>,
    input_manager: InputManager,
    time_last_update: Instant,
    time_last_draw: Instant,
    time_since_beginning: f32,
    camera: Player,
    world_data: WorldData,
    drawer: Drawer,
}

impl Game {
    unsafe fn new(event_loop: &EventLoop<()>) -> Self {
        let input_manager: InputManager = InputManager::new();
        let window = WindowBuilder::new()
            .with_title("Best jeu ever")
            .with_inner_size(PhysicalSize::new(1024, 768));

        let gl_context = ContextBuilder::new()
            .with_depth_buffer(24)
            .with_vsync(true)
            .with_multisampling(4)
            .with_gl(GlRequest::Specific(Api::OpenGl, (4, 0)))
            .build_windowed(window, &event_loop)
            .expect("Cannot create windowed context");

        let gl_context = unsafe {
            gl_context
                .make_current()
                .expect("Failed to make context current")
        };
        gl::load_with(|ptr| gl_context.get_proc_address(ptr) as *const _);

        Game {
            gl_context: gl_context,
            input_manager: input_manager,
            time_last_update: Instant::now(),
            time_last_draw: Instant::now(),
            time_since_beginning: 0.0,
            camera: Player::new(Vector3::new(0.0, 0.0, 0.0), 1.0),
            world_data: WorldData::new(),
            drawer: Drawer::new(),
        }
    }

    unsafe fn load_content(&mut self) {
        let aspect_ratio = (self.gl_context.window().inner_size().width as f32)
            / (self.gl_context.window().inner_size().height as f32);
        self.camera = Player::new(Vector3::new(0.0, 0.0, 0.0), aspect_ratio);

        //self.world_data.generate_bottle();
        self.world_data.generate_ground();
        //self.world_data.generate_random();

        self.drawer.load_content(&self.gl_context);
    }

    fn update(&mut self) -> ControlFlow {
        // Quit game when escape is pressed
        if self.input_manager.is_pressed(VirtualKeyCode::Escape) {
            self.end();
            return ControlFlow::Exit;
        }

        // Update time
        let time_since_last_update = self.time_last_update.elapsed().as_secs_f32();
        self.time_last_update = Instant::now();
        self.time_since_beginning += time_since_last_update;

        // Update drawer
        self.drawer.update(&mut self.input_manager);

        return ControlFlow::Poll;
    }

    unsafe fn draw(&mut self) {
        // Update time
        let time_since_last_draw = self.time_last_draw.elapsed().as_secs_f32();
        self.time_last_draw = Instant::now();

        // Update camera
        self.camera
            .update(&mut self.input_manager, time_since_last_draw);
        self.camera.camera.recalculate_matrix();

        gl::ClearColor(0.5, 0.5, 0.5, 1.0);
        gl::Clear(gl::COLOR_BUFFER_BIT);

        self.drawer.draw(
            &self.gl_context,
            &self.camera,
            self.time_since_beginning,
            &self.world_data,
        );

        self.gl_context.swap_buffers().unwrap();
    }

    pub fn end(&self) {}
}
