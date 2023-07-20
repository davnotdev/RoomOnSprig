use super::*;

impl GamePlayState {
    pub fn update(&mut self, buttons: Buttons) {
        if !self.player.dead {
            self.ticks += 1;

            self.tick_buttons(buttons);
            self.tick_player();
            self.tick_enemies();
            self.tick_bullets();
            self.tick_medkit();
            self.tick_explosions();
            self.tick_poison();
            self.tick_enemy_spawn();
            self.tick_medkit_heal_popup();
        } else if self.player.direction[1] >= core::f32::consts::FRAC_PI_2 {
            self.player.direction[1] -= 0.2;
        }
    }

    fn tick_player(&mut self) {
        //  Look in the direction you should be looking
        self.player.direction[0] = self.player.yaw.cos();
        self.player.direction[2] = self.player.yaw.sin();
        self.player.direction = vec_normalize(self.player.direction);

        //  Cap the speed.
        if vec_length(self.player.velocity) > PLAYER_MAX_VELOCITY {
            self.player.velocity =
                vec_mul_scalar(vec_normalize(self.player.velocity), PLAYER_MAX_VELOCITY)
        }

        //  Friction.
        self.player.velocity = vec_add_vec(
            self.player.velocity,
            vec_mul_scalar(vec_normalize(self.player.velocity), -PLAYER_FRICTION_SCALAR),
        );

        //  Move forward unless if there's a wall (then you should bounce!).
        //  todo!()
        let next_position = vec_add_vec(self.player.position, self.player.velocity);
        self.player.position = next_position;
    }

    fn tick_enemies(&mut self) {
        self.enemies.retain_mut(|enemy| {
            //  Add speed. They will always outrun you. (Not really.)
            enemy.speed += ENEMY_SPEED_INCREMENT_SCALAR;

            //  Get closer if enemy isn't already too close.
            if vec_distance(self.player.position, enemy.position) >= ENEMY_REACH {
                let direction = vec_normalize(vec_sub_vec(self.player.position, enemy.position));
                //  Adding dodge makes things more interesting.
                let dodge_direction = vec_mul_scalar(
                    vec3_cross_product(direction, [0.0, 1.0, 0.0]),
                    (self.ticks as f32 * 6.0 * enemy.dodge_entropy).sin() * enemy.dodge_entropy,
                );
                let next_position =
                    vec_add_vec(enemy.position, vec_mul_scalar(direction, enemy.speed));
                let next_position = vec_add_vec(next_position, dodge_direction);
                //  Don't go through walls. Go around instead. (Or try to anyway.)
                //  todo!()
                enemy.position = next_position;
            } else {
                //  BAM. Attack the player.
                self.player.health -= ENEMY_DAMAGE;
            }

            enemy.health > 0.0
        });

        self.enemies
            .clone()
            .iter()
            .enumerate()
            .for_each(|(idx, enemy)| {
                //  BAM. Attacked by player.
                if self.hit_by_bullet(enemy.position) {
                    self.enemies[idx].health -= BULLET_DAMAGE;
                }
            });
    }

    fn tick_bullets(&mut self) {
        self.bullets.retain_mut(|bullet| {
            bullet.position = vec_add_vec(
                bullet.position,
                vec_mul_scalar(bullet.direction, BULLET_SPEED),
            );
            vec_distance(bullet.position, bullet.origin) <= BULLET_MAX_DISTANCE
        });
    }

    fn tick_medkit(&mut self) {
        if let Some(medkit) = self.medkit {
            //  Pick up the medkit if it's close enough.
            if vec_distance(self.player.position, medkit.position) <= MEDKIT_PICKUP_RANGE {
                self.player.health += MEDKIT_HEAL_AMOUNT;
                if self.player.health > PLAYER_MAX_HEALTH {
                    self.player.health = PLAYER_MAX_HEALTH;
                }
                self.medkit = None;
            }
        } else {
            self.spawn_medkit(None);
        }
    }

    fn tick_explosions(&mut self) {
        //  BOOM!
        self.explosions.retain_mut(|explosion| {
            explosion.size_scalar += EXPLOSION_GROWTH_INCREMENT;
            //  If too big, remove.
            explosion.size_scalar <= EXPLOSION_MAX_SIZE
        });
    }

    fn tick_poison(&mut self) {
        self.player.health -= PLAYER_POISON_TICK * (get_stage_number(self.ticks) + 1) as f32
    }

    fn tick_medkit_heal_popup(&mut self) {
        //  Popup go up.
        //  todo!()
        //  Popup go away.
        //  todo!()
    }

    fn tick_enemy_spawn(&mut self) {
        //  Spawn enemies based on stage #
        let spawn_param = ENEMY_SPAWN_FREQUENCY_STAGES[get_stage_number(self.ticks)];
        //  TODO Replace this!
        if (self.ticks as f32 % 8.0) < spawn_param {
            self.spawn_enemy();
        }
    }
}
