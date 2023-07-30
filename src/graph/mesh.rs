use super::{buffers::Vertex, buffers::VAO, shader::Shader, texture::TextureParameter};

pub struct Mesh {
    pub shader: Shader,
    pub vao: VAO,
}

use std::ffi::CString;

#[rustfmt::skip]
const BASIC_QUAD: [Vertex; 6] = [
    Vertex([-1.0, -1.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
    Vertex([1.0, -1.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
    Vertex([-1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0]),
    Vertex([-1.0, 1.0], [1.0, 0.0, 0.0], [0.0, 0.0]),
    Vertex([1.0, -1.0], [0.0, 1.0, 0.0], [0.0, 0.0]),
    Vertex([1.0, 1.0], [0.0, 0.0, 1.0], [0.0, 0.0])
];

#[macro_export]
macro_rules! set_attribute {
    ($vbo:ident, $pos:tt, $t:ident :: $field:tt) => {{
        let dummy = core::mem::MaybeUninit::<$t>::uninit();
        let dummy_ptr = dummy.as_ptr();
        let member_ptr = core::ptr::addr_of!((*dummy_ptr).$field);
        const fn size_of_raw<T>(_: *const T) -> usize {
            core::mem::size_of::<T>()
        }
        let member_offset = member_ptr as i32 - dummy_ptr as i32;
        $vbo.set_attribute::<$t>(
            $pos,
            (size_of_raw(member_ptr) / core::mem::size_of::<f32>()) as i32,
            member_offset,
        )
    }};
}

impl Mesh {
    pub unsafe fn new(
        shader_name: String,
        with_fbo: bool,
        texture_parameter: Option<TextureParameter>,
    ) -> Self {
        // Load shader
        let mut shader = Shader::new_without_fbo(CString::new(shader_name.clone()).unwrap());
        if with_fbo {
            shader = Shader::new_with_fbo(
                CString::new(shader_name.clone()).unwrap().clone(),
                texture_parameter.unwrap(),
            );
        }

        shader.compile();
        let vao = VAO::new(&BASIC_QUAD);
        let pos_attrib = shader
            .get_attrib_location("position")
            .expect("can't get attrib location for position");
        set_attribute!(vao, pos_attrib, Vertex::0);

        Self {
            shader: shader,
            vao: vao,
        }
    }

    pub unsafe fn draw(&self) {
        self.shader.apply();
        self.vao.bind();
        gl::DrawArrays(
            gl::TRIANGLES,
            0,
            self.vao.vbo.nbr_of_vertices.try_into().unwrap(),
        );
        self.vao.unbind();
        self.shader.stop();
    }

    pub unsafe fn send_uniform_mat4(&self, attribute_name: &str, value: [f32; 16]) {
        self.shader.apply();
        let attribute_name_cstring = CString::new(attribute_name).unwrap();
        let transform_location =
            gl::GetUniformLocation(self.shader.id, attribute_name_cstring.as_ptr());
        gl::UniformMatrix4fv(transform_location, 1, gl::FALSE, value.as_ptr());
        self.shader.stop();
    }

    pub unsafe fn _send_uniform_vec2(&self, attribute_name: &str, value: [f32; 2]) {
        self.shader.apply();
        let attribute_name_cstring = CString::new(attribute_name).unwrap();
        let transform_location =
            gl::GetUniformLocation(self.shader.id, attribute_name_cstring.as_ptr());
        gl::Uniform2fv(transform_location, 1, value.as_ptr());
        self.shader.stop();
    }

    pub unsafe fn send_uniform_vec3(&self, attribute_name: &str, value: [f32; 3]) {
        self.shader.apply();
        let attribute_name_cstring = CString::new(attribute_name).unwrap();
        let transform_location =
            gl::GetUniformLocation(self.shader.id, attribute_name_cstring.as_ptr());
        gl::Uniform3fv(transform_location, 1, value.as_ptr());
        self.shader.stop();
    }

    pub unsafe fn _send_uniform_i32(&self, attribute_name: &str, value: i32) {
        self.shader.apply();
        let attribute_name_cstring = CString::new(attribute_name).unwrap();
        let transform_location =
            gl::GetUniformLocation(self.shader.id, attribute_name_cstring.as_ptr());
        gl::Uniform1i(transform_location, value);
        self.shader.stop();
    }

    pub unsafe fn send_uniform_f32(&self, attribute_name: &str, value: f32) {
        self.shader.apply();
        let attribute_name_cstring = CString::new(attribute_name).unwrap();
        let transform_location =
            gl::GetUniformLocation(self.shader.id, attribute_name_cstring.as_ptr());
        gl::Uniform1f(transform_location, value);
        self.shader.stop();
    }
}
