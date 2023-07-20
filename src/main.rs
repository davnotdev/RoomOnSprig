#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    mono_font::{ascii, MonoTextStyle},
    pixelcolor::{Rgb565, Rgb888},
    prelude::*,
    text::Text,
};
use smallvec::{smallvec, SmallVec};
use trowel::{App, AppResult, Buttons};

#[allow(unused_imports)]
use micromath::F32Ext;

mod color;
mod graphics;
mod math;
mod models;

use color::Color;
use graphics::{Framebuffer, ProjectionData, RenderPass};
use math::{
    mat4_get_look_at, mat4_get_projection, mat4_identity, mat4_mul_mat4, mat4_mul_vec4,
    mat4_rotate, mat4_scale, mat4_translate, triangle_clip_plane, vec3_cross_product,
    vec3_into_vec4, vec4_into_vec3, vec4_scale_with_w, vec_add_scalar, vec_add_vec, vec_dot,
    vec_length, vec_mul_scalar, vec_normalize, vec_sub_vec, Mat4, Vec3, Vec4,
};

struct Game {
    tick: u64,
    frame: i32, // Frame count
    framebuffer: Framebuffer,
}

impl Game {
    pub fn new() -> Self {
        Self {
            tick: 0,
            frame: 0,
            framebuffer: Framebuffer::new(),
        }
    }
}

impl App for Game {
    fn init(&mut self) -> AppResult {
        Ok(())
    }

    fn update(&mut self, _buttons: Buttons) -> AppResult {
        self.frame += 1;
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        self.tick += 1;
        self.framebuffer.clear_color(Color::Gray0);
        self.framebuffer.clear_depth(core::u16::MAX);
        for i in -20..20 {
            let mat = mat4_identity();
            let mat = mat4_scale(mat, [2.0, 2.0, 2.0]);
            let mat = mat4_rotate(mat, self.tick as f32 / 10.0, [1.0, 1.0, 1.0]);
            let mat = mat4_translate(mat, [(3 * i) as f32, 0.0, -20.0]);
            self.framebuffer.render_pass(&RenderPass {
                camera_front: [0.0, 0.0, 1.0],
                camera_position: [0.0, 0.0, -10.0],
                triangles: models::cube(),
                model: mat,
                color: Some(Color::Red3),
                border_color: Some(Color::Gray1),
                enable_depth: true,
                projection: Some(ProjectionData {
                    fov_rad: core::f32::consts::PI / 2.0,
                    near: 0.1,
                    far: 50.0,
                }),
            });
        }
        let mat = mat4_identity();
        let mat = mat4_scale(mat, [2.0, 2.0, 2.0]);
        let mat = mat4_rotate(mat, self.tick as f32 / 10.0, [1.0, 1.0, 1.0]);
        let mat = mat4_translate(mat, [1.0, 0.0, 0.0]);
        self.framebuffer.render_pass(&RenderPass {
            camera_front: [0.0, 0.0, 1.0],
            camera_position: [0.0, 0.0, -10.0],
            triangles: models::cube(),
            model: mat,
            color: Some(Color::Red3),
            border_color: Some(Color::Gray1),
            enable_depth: true,
            projection: Some(ProjectionData {
                fov_rad: core::f32::consts::PI / 2.0,
                near: 0.1,
                far: 50.0,
            }),
        });
        self.framebuffer.flush(display);
        Ok(())
    }
}

#[trowel::entry]
fn main() {
    trowel::run(Game::new());
}
