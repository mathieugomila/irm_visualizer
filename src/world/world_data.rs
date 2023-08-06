use cgmath::{Vector3, Vector4};
use gl::types::GLuint;

use bracket_noise::prelude::*;
use rand::Rng;

pub const WORLD_SIZE: usize = 256;

#[derive(Clone, Copy, Debug)]
pub struct Bloc {
    pub color: Vector4<u8>,
}

pub struct WorldData {
    blocs: Vec<Vec<Vec<Bloc>>>,
    world_data_texture: WorldDataTexture,
}

pub struct WorldDataTexture {
    pub texture_id: GLuint,
    buffer: Box<[u8]>,
}

impl WorldData {
    pub unsafe fn new() -> Self {
        let blocs = vec![
            vec![
                vec![
                    Bloc {
                        color: Vector4 {
                            x: 0,
                            y: 0,
                            z: 0,
                            w: 0
                        }
                    };
                    WORLD_SIZE
                ];
                WORLD_SIZE
            ];
            WORLD_SIZE
        ];

        Self {
            blocs: blocs,
            world_data_texture: WorldDataTexture::new(),
        }
    }

    pub fn generate_random(&mut self) {
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                for z in 0..WORLD_SIZE {
                    let random = ((72.56 * x as f32 + 98.51 * y as f32 + 83.58 * z as f32).sin()
                        * 48965.0)
                        .fract();

                    if random < -0.950 {
                        self.change_bloc_without_regen(
                            Vector3 {
                                x: x as i32,
                                y: y as i32,
                                z: z as i32,
                            },
                            Bloc {
                                color: Vector4 {
                                    x: 255,
                                    y: 0,
                                    z: 0,
                                    w: 255,
                                },
                            },
                        );
                    }
                    if y == 98 {
                        self.change_bloc_without_regen(
                            Vector3 {
                                x: x as i32,
                                y: y as i32,
                                z: z as i32,
                            },
                            Bloc {
                                color: Vector4 {
                                    x: 0,
                                    y: 255,
                                    z: 0,
                                    w: 255,
                                },
                            },
                        );
                    }
                }
            }
        }
        self.regenerate_texture();
    }

    pub fn generate_bottle(&mut self) {
        for x in 0..WORLD_SIZE {
            for y in 0..WORLD_SIZE {
                for z in 0..WORLD_SIZE {
                    if self.is_board(Vector3::new(x as i32, y as i32, z as i32)) {
                        if !(y == WORLD_SIZE - 1
                            && ((x as i32 - WORLD_SIZE as i32 / 2).pow(2)
                                + (z as i32 - WORLD_SIZE as i32 / 2).pow(2))
                                < (WORLD_SIZE as i32 / 3).pow(2))
                        {
                            self.change_bloc_without_regen(
                                Vector3 {
                                    x: x as i32,
                                    y: y as i32,
                                    z: z as i32,
                                },
                                Bloc {
                                    color: Vector4 {
                                        x: 0,
                                        y: 0,
                                        z: 255,
                                        w: 255,
                                    },
                                },
                            );
                        }
                    }
                }
            }
        }
        self.regenerate_texture();
    }

    fn is_board(&self, pos: Vector3<i32>) -> bool {
        return pos.x == 0
            || pos.y == 0
            || pos.z == 0
            || pos.x == WORLD_SIZE as i32 - 1
            || pos.y == WORLD_SIZE as i32 - 1
            || pos.z == WORLD_SIZE as i32 - 1;
    }

    pub fn generate_ground(&mut self) {
        let mut rng = rand::thread_rng();
        let mut perlin_noise = FastNoise::seeded(rng.gen_range(0..std::u64::MAX));
        perlin_noise.set_frequency(0.007);
        perlin_noise.set_noise_type(NoiseType::PerlinFractal);
        perlin_noise.set_fractal_octaves(8);

        let frequency = 0.37;

        for x in 0..WORLD_SIZE {
            for z in 0..WORLD_SIZE {
                let height = ((0.5
                    * (perlin_noise.get_noise3d(
                        x as f32 * frequency,
                        0.0 as f32,
                        z as f32 * frequency,
                    ) + 1.0))
                    * (WORLD_SIZE - 1) as f32
                    - (WORLD_SIZE / 4) as f32) as usize;

                for y in 0..height {
                    if !self.is_board(Vector3::new(x as i32, y as i32, z as i32)) {
                        self.change_bloc_without_regen(
                            Vector3 {
                                x: x as i32,
                                y: y as i32,
                                z: z as i32,
                            },
                            Bloc {
                                color: Vector4 {
                                    x: x as u8,
                                    y: y as u8,
                                    z: z as u8,
                                    w: 255,
                                },
                            },
                        );
                    }
                }
            }
        }
        println!("Ground generated");
        self.regenerate_texture();
    }

    fn change_bloc_without_regen(&mut self, pos: Vector3<i32>, bloc: Bloc) {
        if Self::is_outside_world(pos) {
            return;
        }
        self.blocs[pos.x as usize][pos.y as usize][pos.z as usize] = bloc;
        self.world_data_texture.change_id(pos, bloc, false);
    }

    pub fn regenerate_texture(&self) {
        unsafe {
            self.world_data_texture.regenerate_texture();
        }
    }

    pub unsafe fn bind_texture(&self) {
        self.world_data_texture.bind_texture();
    }

    pub fn is_outside_world(position: Vector3<i32>) -> bool {
        return position.x < 0
            || position.y < 0
            || position.z < 0
            || position.x >= WORLD_SIZE as i32
            || position.y >= WORLD_SIZE as i32
            || position.z >= WORLD_SIZE as i32;
    }
}

impl WorldDataTexture {
    pub unsafe fn new() -> Self {
        let mut texture_id: GLuint = 0;
        let buffer = vec![0u8; WORLD_SIZE * WORLD_SIZE * WORLD_SIZE * 4].into_boxed_slice();
        unsafe {
            gl::GenTextures(1, &mut texture_id);
            gl::BindTexture(gl::TEXTURE_3D, texture_id);
            gl::TexParameteri(
                gl::TEXTURE_3D,
                gl::TEXTURE_WRAP_S,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_3D,
                gl::TEXTURE_WRAP_T,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(
                gl::TEXTURE_3D,
                gl::TEXTURE_WRAP_R,
                gl::CLAMP_TO_BORDER as i32,
            );
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
            gl::TexParameteri(gl::TEXTURE_3D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
            gl::TexImage3D(
                gl::TEXTURE_3D,
                0,
                gl::RGBA8 as i32,
                WORLD_SIZE as i32,
                WORLD_SIZE as i32,
                WORLD_SIZE as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                buffer.as_ptr() as *const std::ffi::c_void,
            );
        }

        Self {
            texture_id: texture_id,
            buffer: buffer,
        }
    }

    pub unsafe fn regenerate_texture(&self) {
        gl::BindTexture(gl::TEXTURE_3D, self.texture_id);
        gl::TexImage3D(
            gl::TEXTURE_3D,
            0,
            gl::RGBA8 as i32,
            WORLD_SIZE as i32,
            WORLD_SIZE as i32,
            WORLD_SIZE as i32,
            0,
            gl::RGBA,
            gl::UNSIGNED_BYTE,
            self.buffer.as_ptr() as *const std::ffi::c_void,
        );
    }

    pub fn change_id(&mut self, pos: Vector3<i32>, bloc: Bloc, do_regenerate: bool) {
        self.buffer[4
            * (pos.x as usize
                + WORLD_SIZE * pos.y as usize
                + WORLD_SIZE * WORLD_SIZE * pos.z as usize)
            + 0] = bloc.color.x;
        self.buffer[4
            * (pos.x as usize
                + WORLD_SIZE * pos.y as usize
                + WORLD_SIZE * WORLD_SIZE * pos.z as usize)
            + 1] = bloc.color.y;
        self.buffer[4
            * (pos.x as usize
                + WORLD_SIZE * pos.y as usize
                + WORLD_SIZE * WORLD_SIZE * pos.z as usize)
            + 2] = bloc.color.z;
        self.buffer[4
            * (pos.x as usize
                + WORLD_SIZE * pos.y as usize
                + WORLD_SIZE * WORLD_SIZE * pos.z as usize)
            + 3] = bloc.color.w;
        if do_regenerate {
            unsafe {
                self.regenerate_texture();
            }
        }
    }

    pub unsafe fn bind_texture(&self) {
        gl::BindTexture(gl::TEXTURE_3D, self.texture_id);
    }
}
