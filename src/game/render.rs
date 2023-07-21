use super::*;

impl GamePlayState {
    pub fn render(&self, fb: &mut Framebuffer) {
        fb.clear_color(Color::Gray2);
        fb.clear_depth(core::f32::MAX);

        let camera_position = self.player.position;
        let camera_front = vec_normalize(self.player.direction);

        let projection = ProjectionData {
            fov_rad: core::f32::consts::FRAC_PI_2,
            near: 0.1,
            far: 70.0,
        };

        let far_projection = ProjectionData {
            fov_rad: core::f32::consts::FRAC_PI_2,
            near: 0.1,
            far: 200.0,
        };

        for enemy in &self.enemies {
            let mv = mat4_identity();
            let mv = mat4_scale(mv, [1.5, -2.0, 1.5]);
            let mv = mat4_rotate(
                mv,
                (enemy.color as u8 as usize * self.ticks) as f32,
                [0.0, 1.0, 0.0],
            );
            let mv = mat4_translate(mv, vec_add_vec(enemy.position, [0.0, 2.0, 0.0]));

            fb.render_pass(&RenderPass {
                camera_front,
                camera_position,
                triangles: models::person(),
                model: mv,
                color: Some(enemy.color),
                border_color: Some(enemy.color),
                enable_depth: true,
                projection: Some(projection),
            })
        }

        for wall in &self.walls {
            let mv = mat4_identity();
            let mv = mat4_scale(mv, wall.scale);
            let mv = mat4_translate(mv, wall.position);

            fb.render_pass(&RenderPass {
                camera_front,
                camera_position,
                triangles: models::cube(),
                model: mv,
                color: Some(Color::Gray1),
                border_color: Some(Color::Gray0),
                enable_depth: true,
                projection: Some(projection),
            })
        }

        for bullet in &self.bullets {
            let mv = mat4_identity();
            let mv = mat4_translate(mv, vec_add_vec(bullet.position, [0.0, 0.25, 0.0]));
            let mv = mat4_scale(mv, [0.35, 0.35, 0.35]);

            fb.render_pass(&RenderPass {
                camera_front,
                camera_position,
                triangles: models::cube(),
                model: mv,
                color: None,
                border_color: Some(Color::Yellow6),
                enable_depth: true,
                projection: Some(projection),
            })
        }

        for explosion in &self.explosions {
            let mv = mat4_identity();
            let mv = mat4_translate(mv, explosion.position);
            let mv = mat4_scale(mv, vec_mul_scalar([1.0, 1.0, 1.0], explosion.size_scalar));

            fb.render_pass(&RenderPass {
                camera_front,
                camera_position,
                triangles: models::cube(),
                model: mv,
                color: Some(Color::Orange9),
                border_color: None,
                enable_depth: true,
                projection: Some(projection),
            })
        }

        if let Some(medkit) = self.medkit {
            let mv = mat4_identity();
            let mv = mat4_scale(mv, [1.2, 0.3, 1.2]);
            let mv = mat4_rotate(mv, self.ticks as f32 / 10.0, [0.0, 1.0, 0.0]);
            let mv = mat4_translate(mv, vec_add_vec(medkit.position, [0.0, 3.0, 0.0]));

            fb.render_pass(&RenderPass {
                camera_front,
                camera_position,
                triangles: models::cube(),
                model: mv,
                color: Some(Color::Red3),
                border_color: Some(Color::Red3),
                enable_depth: false,
                projection: Some(far_projection),
            });
        }
    }
}
