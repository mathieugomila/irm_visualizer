use gl::types::{
    GLint,
    GLsizeiptr,
    GLuint,
};

pub type Pos = [f32; 2];
pub type Color = [f32; 3];
pub type TextureCoords = [f32; 2];

#[repr(C, packed)]
#[derive(Clone, Copy)]
pub struct Vertex(pub Pos, pub Color, pub TextureCoords);

pub struct VBO
{
    pub id:              GLuint,
    target:              GLuint,
    pub nbr_of_vertices: GLuint,
}

impl VBO
{
    pub unsafe fn new(target: GLuint) -> Self
    {
        let mut id: GLuint = 0;
        gl::GenBuffers(1, &mut id);
        Self { id, target, nbr_of_vertices: 0 }
    }

    pub unsafe fn bind(&self)
    {
        gl::BindBuffer(self.target, self.id);
    }

    pub unsafe fn unbind(&self)
    {
        gl::BindBuffer(self.target, 0);
    }

    pub unsafe fn set_data<D>(&mut self, data: &[D], usage: GLuint)
    {
        self.bind();
        let (_, data_bytes, _) = data.align_to::<u8>();
        gl::BufferData(self.target, data_bytes.len() as GLsizeiptr, data_bytes.as_ptr() as *const _, usage);
        self.nbr_of_vertices = data.len().try_into().unwrap();
        self.unbind();
    }
}

impl Drop for VBO
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteBuffers(1, [self.id].as_ptr());
        }
    }
}

pub struct VAO
{
    pub id:  GLuint,
    pub vbo: VBO,
}

impl VAO
{
    pub unsafe fn new<D>(data: &[D]) -> Self
    {
        let mut id: GLuint = 0;

        // generate vao
        gl::GenVertexArrays(1, &mut id);
        gl::BindVertexArray(id);

        // create vbo
        let mut vbo = VBO::new(gl::ARRAY_BUFFER);
        vbo.set_data(data, gl::STATIC_DRAW);
        Self { id, vbo: vbo }
    }

    pub unsafe fn bind(&self)
    {
        gl::BindVertexArray(self.id);
        self.vbo.bind();
    }

    pub unsafe fn unbind(&self)
    {
        self.vbo.unbind();
        gl::BindVertexArray(0);
    }

    pub unsafe fn set_attribute<V: Sized>(&self, attrib_pos: GLuint, components: GLint, offset: GLint)
    {
        self.bind();
        gl::VertexAttribPointer(attrib_pos, components, gl::FLOAT, gl::FALSE, std::mem::size_of::<V>() as GLint, offset as *const _);
        gl::EnableVertexAttribArray(attrib_pos);
        self.unbind();
    }
}

impl Drop for VAO
{
    fn drop(&mut self)
    {
        unsafe {
            gl::DeleteVertexArrays(1, [self.id].as_ptr());
        }
    }
}
