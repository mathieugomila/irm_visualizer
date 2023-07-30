use gl::types::{GLenum, GLuint};

pub struct Texture2D {
    pub id: GLuint,
}

#[derive(Debug)]
pub struct TextureParameter {
    pub screen_size: (i32, i32),
    pub internal_format: GLenum,
    pub format_type: GLenum,
}

impl Texture2D {
    pub unsafe fn new(parameters: TextureParameter) -> Self {
        let mut texture_id: GLuint = 0;
        gl::GenTextures(1, &mut texture_id);
        gl::BindTexture(gl::TEXTURE_2D, texture_id);
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_S,
            gl::CLAMP_TO_BORDER as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_T,
            gl::CLAMP_TO_BORDER as i32,
        );
        gl::TexParameteri(
            gl::TEXTURE_2D,
            gl::TEXTURE_WRAP_R,
            gl::CLAMP_TO_BORDER as i32,
        );
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        gl::TexImage2D(
            gl::TEXTURE_2D,
            0,
            parameters.internal_format as i32,
            parameters.screen_size.0 as i32,
            parameters.screen_size.1 as i32,
            0,
            gl::RGBA,
            parameters.format_type,
            std::ptr::null(),
        );

        Self { id: texture_id }
    }

    pub unsafe fn copy(texture_id_src: GLuint, texture_id_dst: GLuint, window_size: (i32, i32)) {
        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, texture_id_src);

        gl::ActiveTexture(gl::TEXTURE1);
        gl::BindTexture(gl::TEXTURE_2D, texture_id_dst);

        gl::CopyImageSubData(
            texture_id_src,
            gl::TEXTURE_2D,
            0,
            0,
            0,
            0,
            texture_id_dst,
            gl::TEXTURE_2D,
            0,
            0,
            0,
            0,
            window_size.0,
            window_size.1,
            1,
        );

        gl::ActiveTexture(gl::TEXTURE0);
    }
}

impl TextureParameter {
    pub fn new_float_parameter(screen_size: (i32, i32)) -> Self {
        TextureParameter {
            screen_size: screen_size,
            internal_format: gl::RGBA32F,
            format_type: gl::FLOAT,
        }
    }

    pub fn _new_unsigned_byte_parameter(screen_size: (i32, i32)) -> Self {
        TextureParameter {
            screen_size: screen_size,
            internal_format: gl::RGBA,
            format_type: gl::UNSIGNED_BYTE,
        }
    }
}
