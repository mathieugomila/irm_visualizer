use cgmath::{InnerSpace, Quaternion, Rad, Rotation, Rotation3, Vector3};
use glutin::event::VirtualKeyCode;

use crate::io::input_player::InputManager;

use super::camera::{Camera, CameraUniform};

const EYES_HEIGHT: f32 = 0.20;
const MOUSE_SENSIBILITY: f32 = 0.003;
const SPEED: f32 = 0.2;

pub struct Player {
    player_position: Vector3<f32>,
    pub camera: Camera,
}

impl Player {
    pub fn new(spawn_position: Vector3<f32>, aspect_ratio: f32) -> Self {
        Player {
            player_position: spawn_position,
            camera: Camera::new(
                spawn_position + Vector3::new(0.0, EYES_HEIGHT, 0.0),
                aspect_ratio,
            ),
        }
    }

    pub fn update(&mut self, input_manager: &mut InputManager, time_since_last_update: f32) {
        // Move player and head using keyboard/mouse input
        self.update_movement_body(input_manager, time_since_last_update);
        self.update_movement_head(input_manager);
        input_manager.reset_delta();

        // Set camera position according to the position of player
        self.camera
            .set_position(self.player_position + Vector3::new(0.0, EYES_HEIGHT, 0.0));
    }

    fn update_movement_body(
        &mut self,
        input_manager: &mut InputManager,
        time_since_last_update: f32,
    ) {
        // Move camera using WASD inputs
        let mut multiplicator = 1.0;
        if input_manager.is_pressed(VirtualKeyCode::LShift) {
            multiplicator = 5.0;
        }

        if input_manager.is_pressed(VirtualKeyCode::Space) {
            self.move_up(time_since_last_update * multiplicator);
        }
        if input_manager.is_pressed(VirtualKeyCode::LControl) {
            self.move_down(time_since_last_update * multiplicator);
        }
        if input_manager.is_pressed(VirtualKeyCode::Z) {
            self.move_forward(time_since_last_update * multiplicator);
        }
        if input_manager.is_pressed(VirtualKeyCode::S) {
            self.move_backward(time_since_last_update * multiplicator);
        }
        if input_manager.is_pressed(VirtualKeyCode::Q) {
            self.move_left(time_since_last_update * multiplicator);
        }
        if input_manager.is_pressed(VirtualKeyCode::D) {
            self.move_right(time_since_last_update * multiplicator);
        }
    }

    fn update_movement_head(&mut self, input_manager: &InputManager) {
        // Move camera using mouse input information
        self.camera
            .rotate_forward(input_manager.get_delta() * MOUSE_SENSIBILITY);
    }

    fn move_forward(&mut self, time_since_last_update: f32) {
        let mut forward_without_y = Vector3::new(
            self.camera.get_forward().x,
            0.0,
            self.camera.get_forward().z,
        );
        forward_without_y = InnerSpace::normalize(forward_without_y);

        let movement_vector_x = Vector3::new(
            SPEED * forward_without_y.x * time_since_last_update,
            0.0,
            0.0,
        );
        let movement_vector_z = Vector3::new(
            0.0,
            0.0,
            SPEED * forward_without_y.z * time_since_last_update,
        );

        self.player_position += movement_vector_x;
        self.player_position += movement_vector_z;
    }

    fn move_backward(&mut self, time_since_last_update: f32) {
        self.move_forward(-time_since_last_update);
    }

    fn move_left(&mut self, time_since_last_update: f32) {
        let mut forward_without_y = Vector3::new(
            self.camera.get_forward().x,
            0.0,
            self.camera.get_forward().z,
        );
        forward_without_y = InnerSpace::normalize(forward_without_y);
        let rotation_left: Quaternion<f32> =
            Rotation3::from_angle_y(Rad(std::f32::consts::FRAC_PI_2));
        let left_vector = rotation_left.rotate_vector(forward_without_y);

        let movement_vector_x =
            Vector3::new(SPEED * left_vector.x * time_since_last_update, 0.0, 0.0);
        let movement_vector_z =
            Vector3::new(0.0, 0.0, SPEED * left_vector.z * time_since_last_update);

        self.player_position += movement_vector_x;
        self.player_position += movement_vector_z;
    }

    fn move_right(&mut self, time_since_last_update: f32) {
        self.move_left(-time_since_last_update);
    }

    fn move_up(&mut self, time_since_last_update: f32) {
        self.player_position += SPEED * time_since_last_update * Vector3::unit_y()
    }

    fn move_down(&mut self, time_since_last_update: f32) {
        self.move_up(-time_since_last_update);
    }

    pub fn _get_position(&self) -> Vector3<f32> {
        return self.player_position;
    }

    pub fn get_eye_position(&self) -> Vector3<f32> {
        return self.camera.get_position();
    }

    pub fn _teleport(&mut self, position: Vector3<f32>) {
        self.player_position = position;
        self.camera
            .set_position(position + Vector3::new(0.0, EYES_HEIGHT, 0.0))
    }

    pub fn get_uniform(&self) -> CameraUniform {
        self.camera.get_uniform()
    }
}
