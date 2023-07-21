use super::*;

impl GamePlayState {
    pub fn tick_buttons(&mut self, buttons: Buttons) {
        if buttons.contains(Buttons::W) {
            self.player_bob();
            self.player.velocity = vec_add_vec(
                self.player.velocity,
                vec_mul_scalar(self.player.direction, PLAYER_ACCELERATION),
            )
        }
        if buttons.contains(Buttons::S) {
            self.player_bob();
            self.player.velocity = vec_add_vec(
                self.player.velocity,
                vec_mul_scalar(self.player.direction, -PLAYER_ACCELERATION),
            )
        }
        if buttons.contains(Buttons::A) {
            self.player_bob();
            self.player.velocity = vec_add_vec(
                self.player.velocity,
                vec_mul_scalar(
                    vec_normalize(vec3_cross_product(self.player.direction, [0.0, 1.0, 0.0])),
                    PLAYER_ACCELERATION,
                ),
            );
        }
        if buttons.contains(Buttons::D) {
            self.player_bob();
            self.player.velocity = vec_add_vec(
                self.player.velocity,
                vec_mul_scalar(
                    vec_normalize(vec3_cross_product(self.player.direction, [0.0, 1.0, 0.0])),
                    -PLAYER_ACCELERATION,
                ),
            );
        }
        if buttons.contains(Buttons::I)
            && self.ticks - self.player.last_bullet_time >= BULLET_COOLDOWN_THRESHOLD
        {
            self.spawn_bullet(self.player.position, self.player.direction);
            self.player.last_bullet_time = self.ticks;
        }
        if buttons.contains(Buttons::K) {
            self.player.yaw -= core::f32::consts::PI;
        }
        if buttons.contains(Buttons::J) {
            self.player.yaw += core::f32::consts::PI / 15.0;
        }
        if buttons.contains(Buttons::L) {
            self.player.yaw -= core::f32::consts::PI / 15.0;
        }
    }

    fn player_bob(&mut self) {
        self.player.bob_tick += 1;
        self.player.position[1] = (self.player.bob_tick as f32 * 0.8).sin() * 0.2 + 0.1
    }
}
