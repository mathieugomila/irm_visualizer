use crate::graph::fbo::FBO;

use std::{
    ffi::{CString, NulError},
    fs::read_to_string,
    ptr,
    string::FromUtf8Error,
};

use gl::types::{GLenum, GLint, GLuint};
use thiserror::Error;

use super::texture::TextureParameter;

#[derive(Debug, Error)]
pub enum ShaderError {
    #[error("Error while compiling shader: {0}")]
    CompilationError(String),
    #[error("Error while linking shaders: {0}")]
    LinkingError(String),
    #[error{"{0}"}]
    Utf8Error(#[from] FromUtf8Error),
    #[error{"{0}"}]
    NulError(#[from] NulError),
}

pub struct Shader {
    pub id: GLuint,
    pub fbo: Option<FBO>,
    pub file_name: CString,
}

impl Shader {
    pub unsafe fn new_with_fbo(file_name: CString, texture_parameter: TextureParameter) -> Self {
        let fbo = Some(FBO::new(texture_parameter));

        Self {
            id: 0,
            fbo: fbo,
            file_name: file_name,
        }
    }

    pub unsafe fn new_without_fbo(file_name: CString) -> Self {
        Self {
            id: 0,
            fbo: Option::None,
            file_name: file_name,
        }
    }

    pub unsafe fn compile(&mut self) {
        let vertex_shader_id_result = Self::create_shader_from_str(
            &read_to_string(format!(
                "assets/shaders/{}_vs.glsl",
                self.file_name.to_str().unwrap().to_owned()
            ))
            .unwrap(),
            gl::VERTEX_SHADER,
        );

        let fragment_shader_id_result = Self::create_shader_from_str(
            &read_to_string(format!(
                "assets/shaders/{}_fs.glsl",
                self.file_name.to_str().unwrap().to_owned()
            ))
            .unwrap(),
            gl::FRAGMENT_SHADER,
        );

        if self.id == 0 {
            if vertex_shader_id_result.is_err() {
                panic!("{}", vertex_shader_id_result.unwrap());
            }

            if fragment_shader_id_result.is_err() {
                panic!("{}", fragment_shader_id_result.unwrap());
            }
        }

        if !vertex_shader_id_result.is_err() && !fragment_shader_id_result.is_err() {
            let program_id = Self::create_program(
                vertex_shader_id_result.unwrap(),
                fragment_shader_id_result.unwrap(),
            )
            .expect(&format!(
                "Can not create shader {}",
                self.file_name.to_str().unwrap().to_owned()
            ));
            self.id = program_id;
        } else {
            println!("Error while reloading shader {:?}", self.file_name);
        }
    }

    unsafe fn create_shader_from_str(
        source_code: &str,
        shader_type: GLenum,
    ) -> Result<GLuint, ShaderError> {
        let source_code = CString::new(source_code)?;
        let current_shader_id = gl::CreateShader(shader_type);
        gl::ShaderSource(current_shader_id, 1, &source_code.as_ptr(), ptr::null());
        gl::CompileShader(current_shader_id);

        // check for shader compilation errors
        let mut success: GLint = 0;
        gl::GetShaderiv(current_shader_id, gl::COMPILE_STATUS, &mut success);

        if success == 1 {
            return Ok(current_shader_id);
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetShaderiv(current_shader_id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetShaderInfoLog(
                current_shader_id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            return Err(ShaderError::CompilationError(log));
        }
    }

    unsafe fn create_program(
        vertex_id: GLuint,
        fragment_id: GLuint,
    ) -> Result<GLuint, ShaderError> {
        let program_id = gl::CreateProgram();

        gl::AttachShader(program_id, vertex_id);
        gl::AttachShader(program_id, fragment_id);

        gl::LinkProgram(program_id);

        let mut success: GLint = 0;
        gl::GetProgramiv(program_id, gl::LINK_STATUS, &mut success);

        if success == 1 {
            Ok(program_id)
        } else {
            let mut error_log_size: GLint = 0;
            gl::GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut error_log_size);
            let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
            gl::GetProgramInfoLog(
                program_id,
                error_log_size,
                &mut error_log_size,
                error_log.as_mut_ptr() as *mut _,
            );

            error_log.set_len(error_log_size as usize);
            let log = String::from_utf8(error_log)?;
            Err(ShaderError::LinkingError(log))
        }
    }

    pub unsafe fn apply(&self) {
        gl::UseProgram(self.id);
        if self.fbo.is_some() {
            self.fbo.as_ref().unwrap().bind();
        }
    }

    pub unsafe fn stop(&self) {
        if self.fbo.is_some() {
            self.fbo.as_ref().unwrap().unbind();
        }
        gl::UseProgram(0);
    }

    pub unsafe fn get_attrib_location(&self, attrib: &str) -> Result<GLuint, NulError> {
        let attrib = CString::new(attrib)?;
        Ok(gl::GetAttribLocation(self.id, attrib.as_ptr()) as GLuint)
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}
