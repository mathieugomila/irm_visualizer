use std::{collections::HashMap, time::Instant};

use cgmath::Vector2;
use glutin::event::VirtualKeyCode;

#[derive(Debug)]
struct KeyInformation {
    pub time_pressed: Instant,
    pub is_pressed: bool,
    pub once: bool,
}

#[derive(Debug)]
struct MouseMoved {
    pub delta_mouse: Vector2<f32>,
}

pub struct InputManager {
    keys: HashMap<VirtualKeyCode, KeyInformation>,
    mouse_moved: MouseMoved,
}

impl InputManager {
    pub fn new() -> Self {
        let keys: HashMap<VirtualKeyCode, KeyInformation> = HashMap::new();

        Self {
            keys,
            mouse_moved: MouseMoved {
                delta_mouse: Vector2 { x: 0.0, y: 0.0 },
            },
        }
    }

    pub fn key_event_pressed(&mut self, input: VirtualKeyCode) {
        self.key_update(input, true);
    }

    pub fn key_event_released(&mut self, input: VirtualKeyCode) {
        self.key_update(input, false);
    }

    fn key_update(&mut self, input: VirtualKeyCode, is_pressed: bool) {
        let key_info = self.keys.get(&input);
        // If key has already been pressed
        if Option::is_some(&key_info) {
            let was_pressed = key_info.as_ref().unwrap().is_pressed;
            // If the key is pressed and was not pressed before
            if is_pressed && !was_pressed {
                self.keys.insert(
                    input,
                    KeyInformation {
                        time_pressed: Instant::now(),
                        is_pressed: true,
                        once: true,
                    },
                );
            }

            // If keys was released
            if !is_pressed && was_pressed {
                self.keys.insert(
                    input,
                    KeyInformation {
                        time_pressed: Instant::now(),
                        is_pressed: false,
                        once: false,
                    },
                );
            }

            return;
        }

        self.keys.insert(
            input,
            KeyInformation {
                time_pressed: Instant::now(),
                is_pressed: is_pressed,
                once: is_pressed,
            },
        );
    }

    pub fn update_mouse(&mut self, delta: (f64, f64)) {
        self.mouse_moved.delta_mouse += Vector2::new(delta.0 as f32, delta.1 as f32);
    }

    pub fn is_pressed(&mut self, virtual_key_code: VirtualKeyCode) -> bool {
        let key = self.keys.get(&virtual_key_code);
        if key.is_some() {
            if key.as_ref().unwrap().is_pressed {
                let key_updated = KeyInformation {
                    time_pressed: key.unwrap().time_pressed,
                    is_pressed: key.unwrap().is_pressed,
                    once: false,
                };
                self.keys.insert(virtual_key_code, key_updated);
                return true;
            }
            return false;
        }
        return false;
    }

    pub fn is_pressed_once(&mut self, virtual_key_code: VirtualKeyCode) -> bool {
        let key = self.keys.get(&virtual_key_code);
        if key.is_some() {
            if key.as_ref().unwrap().is_pressed && key.as_ref().unwrap().once {
                let key_updated = KeyInformation {
                    time_pressed: key.unwrap().time_pressed,
                    is_pressed: key.unwrap().is_pressed,
                    once: false,
                };
                self.keys.insert(virtual_key_code, key_updated);
                return true;
            }
            return false;
        }
        return false;
    }

    pub fn reset_delta(&mut self) {
        self.mouse_moved.delta_mouse = Vector2::new(0.0, 0.0);
    }

    pub fn get_delta(&self) -> Vector2<f32> {
        self.mouse_moved.delta_mouse
    }
}
