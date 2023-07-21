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

    #[inline(always)]
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
        let next_position = vec_add_vec(self.player.position, self.player.velocity);
        if let Some(collision_wall) = Self::get_collision_wall(&self.walls, next_position) {
            //  Over-engineered bouncing.
            let wall_to_player =
                vec_normalize(vec_sub_vec(collision_wall.position, self.player.position));

            //  Well, our walls are never rotated anyway ¯\_ (ツ)_/¯.
            const POSSIBLE_NORMALS: &[Vec3] = &[
                [-1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, 1.0],
            ];
            let mut normal = POSSIBLE_NORMALS[0];
            let mut min_dist = vec_distance(POSSIBLE_NORMALS[0], wall_to_player);
            for &n in POSSIBLE_NORMALS {
                let dist = vec_distance(n, wall_to_player);
                if dist < min_dist {
                    min_dist = dist;
                    normal = n;
                }
            }
            let reflected = vec_add_vec(
                vec_mul_scalar(normal, -2.0 * vec_dot(normal, self.player.velocity)),
                self.player.velocity,
            );
            self.player.velocity = vec_mul_scalar(reflected, PLAYER_WALL_BOUNCE_SCALAR);
        } else {
            self.player.position = next_position;
        }
    }

    #[inline(always)]
    fn tick_enemies(&mut self) {
        self.enemies.iter_mut().for_each(|enemy| {
            //  Add speed. They will always outrun you. (Not really.)
            enemy.speed += ENEMY_SPEED_INCREMENT_SCALAR;

            //  Get closer if enemy isn't already too close.
            if vec_distance(self.player.position, enemy.position) >= ENEMY_REACH {
                let direction = vec_normalize(vec_sub_vec(self.player.position, enemy.position));
                //  Adding dodge makes things more interesting.
                let dodge_direction = vec_mul_scalar(
                    vec3_cross_product(direction, [0.0, 1.0, 0.0]),
                    (self.ticks as f32 * enemy.dodge_entropy).sin() * enemy.dodge_entropy,
                );
                let next_position =
                    vec_add_vec(enemy.position, vec_mul_scalar(direction, enemy.speed));
                let next_position = vec_add_vec(next_position, dodge_direction);
                //  Don't go through walls. Go around instead. (Or try to anyway.)
                if Self::get_collision_wall(&self.walls, enemy.position).is_some() {
                    enemy.position = vec_add_vec(
                        enemy.position,
                        vec_mul_scalar(
                            vec3_cross_product([0.0, 1.0, 0.0], self.player.direction),
                            enemy.speed,
                        ),
                    )
                } else {
                    enemy.position = next_position;
                }
            } else {
                //  BAM. Attack the player.
                self.player.health -= ENEMY_DAMAGE;
            }

            //  BAM. Attacked by player.
            if Self::hit_by_bullet(&mut self.bullets, enemy.position) {
                enemy.health -= BULLET_DAMAGE;
            }
        });
        self.enemies.clone().iter().for_each(|enemy| {
            if enemy.health <= 0.0 {
                self.spawn_explosion(enemy.position);
            }
        });
        self.enemies.retain(|enemy| enemy.health > 0.0);
    }

    #[inline(always)]
    fn tick_bullets(&mut self) {
        self.bullets.retain_mut(|bullet| {
            bullet.position = vec_add_vec(
                bullet.position,
                vec_mul_scalar(bullet.direction, BULLET_SPEED),
            );
            vec_distance(bullet.position, bullet.origin) <= BULLET_MAX_DISTANCE
        });
    }

    #[inline(always)]
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

    #[inline(always)]
    fn tick_poison(&mut self) {
        self.player.health -= PLAYER_POISON_TICK * (get_stage_number(self.ticks) + 1) as f32
    }

    #[inline(always)]
    fn tick_medkit_heal_popup(&mut self) {
        //  Popup go up.
        //  todo!()
        //  Popup go away.
        //  todo!()
    }

    #[inline(always)]
    fn tick_enemy_spawn(&mut self) {
        if (self.ticks % 8) < ENEMY_SPAWN_FREQUENCY_PARAM {
            self.spawn_enemy();
        }
    }
}
