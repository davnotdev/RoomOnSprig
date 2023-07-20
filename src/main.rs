#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_std)]
#![cfg_attr(all(target_arch = "arm", target_os = "none"), no_main)]

use embedded_graphics::{
    draw_target::DrawTarget,
    image::{Image, ImageRawLE},
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
mod game;
mod graphics;
mod math;
mod models;

use color::Color;
use game::GamePlayState;
use graphics::{Framebuffer, ProjectionData, RenderPass};
use math::{
    mat4_get_look_at, mat4_get_projection, mat4_identity, mat4_mul_mat4, mat4_mul_vec4,
    mat4_rotate, mat4_scale, mat4_translate, triangle_clip_plane, vec3_cross_product,
    vec3_into_vec4, vec4_into_vec3, vec4_scale_with_w, vec_add_scalar, vec_add_vec, vec_distance,
    vec_dot, vec_length, vec_mul_scalar, vec_normalize, vec_sub_vec, Mat4, Vec3, Vec4,
};

struct Game {
    framebuffer: Framebuffer,
    game: GamePlayState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            framebuffer: Framebuffer::new(),
            game: GamePlayState::new(),
        }
    }
}

impl App for Game {
    fn init(&mut self) -> AppResult {
        self.game.init();
        Ok(())
    }

    fn update(&mut self, buttons: Buttons) -> AppResult {
        self.game.update(buttons);
        Ok(())
    }

    fn draw<T, E>(&mut self, display: &mut T) -> AppResult
    where
        T: DrawTarget<Color = Rgb565, Error = E>,
    {
        self.game.render(&mut self.framebuffer);
        self.framebuffer.flush(display);
        Ok(())
    }
}

#[trowel::entry]
fn main() {
    trowel::run(Game::new());
}
