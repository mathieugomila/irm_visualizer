use gl::types::GLuint;

use crate::graph::texture::Texture2D;

use super::texture::TextureParameter;

pub struct FBO {
    pub fbo_id: GLuint,
    pub texture: Texture2D,
}

impl FBO {
    pub unsafe fn new(texture_parameter: TextureParameter) -> Self {
        let mut fbo_id: GLuint = 0;

        gl::GenFramebuffers(1, &mut fbo_id);

        let texture = Texture2D::new(texture_parameter);

        gl::BindFramebuffer(gl::FRAMEBUFFER, fbo_id);
        gl::FramebufferTexture2D(
            gl::FRAMEBUFFER,
            gl::COLOR_ATTACHMENT0,
            gl::TEXTURE_2D,
            texture.id,
            0,
        );
        gl::BindTexture(gl::TEXTURE_2D, 0);
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);

        Self {
            fbo_id: fbo_id,
            texture: texture,
        }
    }

    pub unsafe fn bind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, self.fbo_id);
    }

    pub unsafe fn unbind(&self) {
        gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
    }
}
