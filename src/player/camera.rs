use cgmath::{
    ortho,
    perspective,
    Matrix4,
    Quaternion,
    Rad,
    Rotation,
    Rotation3,
    SquareMatrix,
    Vector2,
    Vector3,
};

#[derive(Debug)]
pub struct Camera
{
    position:            Vector3<f32>,
    rotation:            Vector2<f32>,
    forward:             Vector3<f32>,
    view_matrix:         Matrix4<f32>,
    perspective_matrix:  Matrix4<f32>,
    previous_mvp_matrix: Matrix4<f32>,
    orthogonal_matrix:   Matrix4<f32>,
}

#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct CameraUniform
{
    pub mvp:                        [[f32; 4]; 4],
    pub previous_mvp:               [[f32; 4]; 4],
    pub orientation_inversed:       [[f32; 4]; 4],
    pub mvp_ortho:                  [[f32; 4]; 4],
    pub orientation_inversed_ortho: [[f32; 4]; 4],
    pub position:                   [f32; 4],
}

impl Camera
{
    pub fn new(position: Vector3<f32>, aspect_ratio: f32) -> Camera
    {
        Camera { position:            position,
                 rotation:            Vector2 { x: 0.0, y: 0.0 },
                 forward:             Vector3 { x: 0.0, y: 0.0, z: 1.0 },
                 view_matrix:         Matrix4::look_at_rh(cgmath::Point3 { x: position.x, y: position.y, z: position.z },
                                                          cgmath::Point3 { x: position.x + 0.0, y: position.y + 0.0, z: position.z + 1.0 },
                                                          Vector3::new(0.0, 1.0, 0.0)),
                 perspective_matrix:  perspective(Rad(1.0), aspect_ratio, 0.001, 50.0),
                 previous_mvp_matrix: Matrix4::identity(),
                 orthogonal_matrix:   ortho(-25.0, 25.0, -25.0, 25.0, 0.001, 50.0), }
    }

    pub fn set_position(&mut self, new_position: Vector3<f32>)
    {
        self.position = new_position;
    }

    pub fn _translate_camera(&mut self, delta_movement: Vector3<f32>)
    {
        self.position += delta_movement;
    }

    pub fn rotate_forward(&mut self, rot: Vector2<f32>)
    {
        self.rotation += rot;

        if self.rotation.y > std::f32::consts::FRAC_PI_2 - 0.01
        {
            self.rotation.y = std::f32::consts::FRAC_PI_2 - 0.01
        }

        if self.rotation.y < -std::f32::consts::FRAC_PI_2 + 0.01
        {
            self.rotation.y = -std::f32::consts::FRAC_PI_2 + 0.01
        }

        let rotation_quaternion_x: Quaternion<f32> = Rotation3::from_angle_x(Rad(self.rotation.y));
        let rotation_quaternion_y: Quaternion<f32> = Rotation3::from_angle_y(Rad(-self.rotation.x));
        let rotation_quaternion = rotation_quaternion_y * rotation_quaternion_x;
        self.forward = rotation_quaternion.rotate_vector(Vector3::unit_z());
    }

    pub fn recalculate_matrix(&mut self)
    {
        self.previous_mvp_matrix = self.get_perspective_mvp_matrix();
        self.view_matrix =
            Matrix4::look_at_rh(cgmath::Point3 { x: self.position.x, y: self.position.y, z: self.position.z },
                                cgmath::Point3 { x: self.position.x + self.forward.x, y: self.position.y + self.forward.y, z: self.position.z + self.forward.z },
                                Vector3::new(0.0, 1.0, 0.0));
    }

    pub fn get_perspective_mvp_matrix(&self) -> Matrix4<f32>
    {
        let mvp = self.perspective_matrix * self.view_matrix;
        mvp
    }

    pub fn get_relative_persp_mvp_matrix(&self) -> Matrix4<f32>
    {
        self.perspective_matrix
        * Matrix4::look_at_rh(cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 },
                              cgmath::Point3 { x: self.forward.x, y: self.forward.y, z: self.forward.z },
                              Vector3::new(0.0, 1.0, 0.0))
    }

    pub fn get_relative_ortho_mvp_matrix(&self) -> Matrix4<f32>
    {
        self.orthogonal_matrix
        * Matrix4::look_at_rh(cgmath::Point3 { x: 0.0, y: 0.0, z: 0.0 },
                              cgmath::Point3 { x: self.forward.x, y: self.forward.y, z: self.forward.z },
                              Vector3::new(0.0, 1.0, 0.0))
    }

    pub fn get_ortho_mvp_matrix(&self) -> Matrix4<f32>
    {
        let mvp = self.orthogonal_matrix * self.view_matrix;
        mvp
    }

    pub fn get_forward(&self) -> Vector3<f32>
    {
        self.forward
    }

    pub fn get_position(&self) -> Vector3<f32>
    {
        self.position
    }

    pub fn _set_forward(&mut self, forward: Vector3<f32>)
    {
        self.forward = forward;
    }

    pub fn get_uniform(&self) -> CameraUniform
    {
        CameraUniform { mvp:                        self.get_perspective_mvp_matrix().into(),
                        previous_mvp:               self.previous_mvp_matrix.into(),
                        orientation_inversed:       self.get_relative_persp_mvp_matrix().invert().unwrap().into(),
                        mvp_ortho:                  self.get_ortho_mvp_matrix().into(),
                        orientation_inversed_ortho: self.get_relative_ortho_mvp_matrix().invert().unwrap().into(),
                        position:                   [self.position.x, self.position.y, self.position.z, 0.0], }
    }
}
