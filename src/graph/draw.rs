use std::ffi::CString;

use glutin::{event::VirtualKeyCode, window::Window, ContextWrapper, PossiblyCurrent};

use crate::{io::input_player::InputManager, player::player::Player, world::world_data::WorldData};

use super::{
    mesh::Mesh,
    texture::{Texture2D, TextureParameter},
};

pub struct Drawer {
    raymarching_quad: Option<Mesh>,
    previous_position_texture: Option<Texture2D>,
    previous_lighting_texture: Option<Texture2D>,
    lighting_quad: Option<Mesh>,
    filter_quad: Option<Mesh>,
}

impl Drawer {
    pub fn new() -> Self {
        return Drawer {
            raymarching_quad: Option::None,
            previous_position_texture: Option::None,
            previous_lighting_texture: Option::None,
            lighting_quad: Option::None,
            filter_quad: Option::None,
        };
    }

    pub unsafe fn load_content(&mut self, gl_context: &ContextWrapper<PossiblyCurrent, Window>) {
        let screen_size = (
            gl_context.window().inner_size().width as i32,
            gl_context.window().inner_size().height as i32,
        );

        self.raymarching_quad = Some(Mesh::new(
            "raymarching".to_string(),
            true,
            Some(TextureParameter::new_float_parameter(screen_size)),
        ));

        self.lighting_quad = Some(Mesh::new(
            "lighting".to_string(),
            true,
            Some(TextureParameter::new_float_parameter(screen_size)),
        ));

        self.filter_quad = Some(Mesh::new("filter".to_string(), false, Option::None));

        self.previous_position_texture =
            Some(Texture2D::new(TextureParameter::new_float_parameter((
                gl_context.window().inner_size().width as i32,
                gl_context.window().inner_size().height as i32,
            ))));

        self.previous_lighting_texture =
            Some(Texture2D::new(TextureParameter::new_float_parameter((
                gl_context.window().inner_size().width as i32,
                gl_context.window().inner_size().height as i32,
            ))));
    }

    pub fn update(&mut self, input_manager: &mut InputManager) {
        // Shaders are recompiled if F5 is pressed
        if input_manager.is_pressed_once(VirtualKeyCode::F5) {
            unsafe {
                self.raymarching_quad.as_mut().unwrap().shader.compile();
                self.lighting_quad.as_mut().unwrap().shader.compile();
                self.filter_quad.as_mut().unwrap().shader.compile();
            }
        }
    }

    pub unsafe fn draw(
        &self,
        gl_context: &ContextWrapper<PossiblyCurrent, Window>,
        player: &Player,
        time_since_beginning: f32,
        world_data: &WorldData,
    ) {
        // Copy old texture to new texture
        Texture2D::copy(
            self.raymarching_quad
                .as_ref()
                .expect("Can't find mesh raymarching_quad")
                .shader
                .fbo
                .as_ref()
                .expect("Can't find fbo raymarching_quad")
                .texture
                .id,
            self.previous_position_texture.as_ref().unwrap().id,
            (
                gl_context.window().inner_size().width as i32,
                gl_context.window().inner_size().height as i32,
            ),
        );

        Texture2D::copy(
            self.lighting_quad
                .as_ref()
                .expect("Can't find mesh lighting_quad")
                .shader
                .fbo
                .as_ref()
                .expect("Can't find fbo lighting_quad")
                .texture
                .id,
            self.previous_lighting_texture.as_ref().unwrap().id,
            (
                gl_context.window().inner_size().width as i32,
                gl_context.window().inner_size().height as i32,
            ),
        );

        // First pass : draw image in a fbo
        self.setup_uniforms_draw_pass_1(player, world_data);
        self.draw_pass_1(gl_context);

        // Second pass: use position draw (draw pass 1) to calculate lighting
        self.setup_uniforms_draw_pass_2(player, time_since_beginning);
        self.draw_pass_2(gl_context);

        // Third pass: use everything drawn before and render final image
        self.setup_uniforms_draw_pass_3(time_since_beginning, world_data);
        self.draw_pass_3(gl_context);
    }

    pub unsafe fn setup_uniforms_draw_pass_1(&self, player: &Player, world_data: &WorldData) {
        // Send orientation inversed matrix to shader
        self.raymarching_quad.as_ref().unwrap().send_uniform_mat4(
            "invert_mvp",
            player
                .get_uniform()
                .orientation_inversed
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        );

        // Send camera position to shader
        self.raymarching_quad
            .as_ref()
            .unwrap()
            .send_uniform_vec3("camera_position", player.get_eye_position().into());

        // Send 3D world data uniform
        let world_data_texture_name = CString::new("world_data_texture").unwrap();
        let world_data_texture_location = gl::GetUniformLocation(
            self.raymarching_quad.as_ref().unwrap().shader.id,
            world_data_texture_name.as_ptr(),
        );

        // Activate texture world data
        gl::ActiveTexture(gl::TEXTURE1);
        world_data.bind_texture();
        self.raymarching_quad.as_ref().unwrap().shader.apply();
        gl::Uniform1i(world_data_texture_location, 1);
        self.raymarching_quad.as_ref().unwrap().shader.stop();
        gl::ActiveTexture(gl::TEXTURE0);
    }

    pub unsafe fn setup_uniforms_draw_pass_2(&self, player: &Player, time_since_beginning: f32) {
        // Send previous mvp matrix to shader
        self.lighting_quad.as_ref().unwrap().send_uniform_mat4(
            "previous_mvp",
            player
                .get_uniform()
                .previous_mvp
                .iter()
                .flatten()
                .copied()
                .collect::<Vec<f32>>()
                .try_into()
                .unwrap(),
        );

        // Sending time to shader
        self.lighting_quad
            .as_ref()
            .unwrap()
            .send_uniform_f32("time", time_since_beginning);

        // Sending previous lighting texture to shader
        let previous_lighting_texture_name = CString::new("previous_lighting_texture").unwrap();
        let previous_lighting_texture_location = gl::GetUniformLocation(
            self.lighting_quad.as_ref().unwrap().shader.id,
            previous_lighting_texture_name.as_ptr(),
        );
        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(
            gl::TEXTURE_2D,
            self.previous_lighting_texture.as_ref().unwrap().id,
        );
        self.lighting_quad.as_ref().unwrap().shader.apply();
        gl::Uniform1i(previous_lighting_texture_location, 1);
        self.lighting_quad.as_ref().unwrap().shader.stop();

        // Sending previous position texture to shader
        let previous_position_texture_name = CString::new("previous_position_texture").unwrap();
        let previous_position_texture_location = gl::GetUniformLocation(
            self.lighting_quad.as_ref().unwrap().shader.id,
            previous_position_texture_name.as_ptr(),
        );
        gl::ActiveTexture(gl::TEXTURE2);
        gl::BindTexture(
            gl::TEXTURE_2D,
            self.previous_position_texture.as_ref().unwrap().id,
        );
        self.lighting_quad.as_ref().unwrap().shader.apply();
        gl::Uniform1i(previous_position_texture_location, 2);
        self.lighting_quad.as_ref().unwrap().shader.stop();

        // Sending current position texture to shader
        let current_position_texture = self
            .raymarching_quad
            .as_ref()
            .expect("Can't find mesh raymarching_quad")
            .shader
            .fbo
            .as_ref()
            .expect("Can't find fbo raymarching_quad")
            .texture
            .id;

        let current_position_texture_name = CString::new("current_position_texture").unwrap();
        let current_position_texture_location = gl::GetUniformLocation(
            self.lighting_quad.as_ref().unwrap().shader.id,
            current_position_texture_name.as_ptr(),
        );
        gl::ActiveTexture(gl::TEXTURE3);
        gl::BindTexture(gl::TEXTURE_2D, current_position_texture);
        self.lighting_quad.as_ref().unwrap().shader.apply();
        gl::Uniform1i(current_position_texture_location, 3);
        self.lighting_quad.as_ref().unwrap().shader.stop();
        gl::ActiveTexture(gl::TEXTURE0);
    }

    pub unsafe fn setup_uniforms_draw_pass_3(
        &self,
        time_since_beginning: f32,
        world_data: &WorldData,
    ) {
        // Sending time to shader
        self.filter_quad
            .as_ref()
            .unwrap()
            .send_uniform_f32("time", time_since_beginning);

        let world_data_texture_name = CString::new("world_data_texture").unwrap();
        let world_data_texture_location = gl::GetUniformLocation(
            self.raymarching_quad.as_ref().unwrap().shader.id,
            world_data_texture_name.as_ptr(),
        );

        // Activate texture world data
        gl::ActiveTexture(gl::TEXTURE1);
        world_data.bind_texture();
        self.filter_quad.as_ref().unwrap().shader.apply();
        gl::Uniform1i(world_data_texture_location, 1);
        self.filter_quad.as_ref().unwrap().shader.stop();

        // Sending current lighting texture
        let current_lighting_texture_name = CString::new("current_lighting_texture").unwrap();
        let current_lighting_texture_location = gl::GetUniformLocation(
            self.filter_quad.as_ref().unwrap().shader.id,
            current_lighting_texture_name.as_ptr(),
        );
        gl::ActiveTexture(gl::TEXTURE2);
        gl::BindTexture(
            gl::TEXTURE_2D,
            self.lighting_quad
                .as_ref()
                .unwrap()
                .shader
                .fbo
                .as_ref()
                .unwrap()
                .texture
                .id,
        );
        self.filter_quad.as_ref().unwrap().shader.apply();
        gl::Uniform1i(current_lighting_texture_location, 2);
        self.filter_quad.as_ref().unwrap().shader.stop();

        // Sending current position texture
        let current_position_texture_name = CString::new("current_position_texture").unwrap();
        let current_position_texture_location = gl::GetUniformLocation(
            self.filter_quad.as_ref().unwrap().shader.id,
            current_position_texture_name.as_ptr(),
        );
        gl::ActiveTexture(gl::TEXTURE3);
        gl::BindTexture(
            gl::TEXTURE_2D,
            self.raymarching_quad
                .as_ref()
                .unwrap()
                .shader
                .fbo
                .as_ref()
                .unwrap()
                .texture
                .id,
        );
        self.filter_quad.as_ref().unwrap().shader.apply();
        gl::Uniform1i(current_position_texture_location, 3);
        self.filter_quad.as_ref().unwrap().shader.stop();
        gl::ActiveTexture(gl::TEXTURE0);
    }

    pub unsafe fn draw_pass_1(&self, gl_context: &ContextWrapper<PossiblyCurrent, Window>) {
        gl::Viewport(
            0,
            0,
            (gl_context.window().inner_size().width) as i32,
            (gl_context.window().inner_size().height) as i32,
        );
        self.raymarching_quad.as_ref().unwrap().draw();
    }

    pub unsafe fn draw_pass_2(&self, gl_context: &ContextWrapper<PossiblyCurrent, Window>) {
        gl::Viewport(
            0,
            0,
            (gl_context.window().inner_size().width) as i32,
            (gl_context.window().inner_size().height) as i32,
        );
        self.lighting_quad.as_ref().unwrap().draw();
    }

    pub unsafe fn draw_pass_3(&self, gl_context: &ContextWrapper<PossiblyCurrent, Window>) {
        gl::Viewport(
            0,
            0,
            (gl_context.window().inner_size().width) as i32,
            (gl_context.window().inner_size().height) as i32,
        );
        self.filter_quad.as_ref().unwrap().draw();
    }
}
