use super::*;

impl GamePlayState {
    pub(super) fn spawn_walls(&mut self) {
        let settings = MAP_SETTINGS[self.selected_map];

        let north = Wall {
            scale: [1.0, MAP_WALL_Y, settings.bound],
            position: [settings.bound, 0.0, 0.0],
        };
        let south = Wall {
            scale: [1.0, MAP_WALL_Y, settings.bound],
            position: [-settings.bound, 0.0, 0.0],
        };
        let west = Wall {
            scale: [settings.bound, MAP_WALL_Y, 1.0],
            position: [0.0, 0.0, -settings.bound],
        };
        let east = Wall {
            scale: [settings.bound, MAP_WALL_Y, 1.0],
            position: [0.0, 0.0, settings.bound],
        };

        self.walls.push(north);
        self.walls.push(south);
        self.walls.push(west);
        self.walls.push(east);

        let mut i = 0;
        while i < settings.wall_count {
            let scale_x = rand_f32() * settings.wall_max_scale as f32 + 3.0;
            let scale_z = rand_f32() * settings.wall_max_scale as f32 + 3.0;
            let position_x = rand_f32() * (settings.bound - 2.0) * 2.0 - settings.bound;
            let position_z = rand_f32() * (settings.bound - 2.0) * 2.0 - settings.bound;

            let valid_wall = self.walls.iter().any(|other| {
                vec_distance(
                    [other.position[0], 0.0, other.position[2]],
                    [position_x, 0.0, position_z],
                ) < MAP_WALL_TO_WALL_MIN_DISTANCE
            }) || (position_x.abs() as usize) < settings.wall_max_scale + 4
                || (position_z.abs() as usize) < settings.wall_max_scale + 4
                || ((settings.bound - position_x.abs()) as usize) < 18
                || ((settings.bound - position_z.abs()) as usize) < 18;

            if valid_wall {
                continue;
            }

            let wall = Wall {
                scale: [scale_x, MAP_WALL_Y, scale_z],
                position: [position_x, 0.0, position_z],
            };
            self.walls.push(wall);

            i += 1;
        }
    }

    pub(super) fn spawn_enemy(&mut self) {
        let stage = get_stage_number(self.ticks);
        if self.enemies.len() >= ENEMY_CAP_STAGES[stage] {
            return;
        }

        let map_setting = MAP_SETTINGS[self.selected_map];
        let position;
        loop {
            let position_x = rand_f32() * map_setting.bound * 2.0 - map_setting.bound;
            let position_z = rand_f32() * map_setting.bound * 2.0 - map_setting.bound;
            let test_position = [position_x, 0.0, position_z];
            let dist_to_player = vec_distance(self.player.position, test_position);
            if (ENEMY_PLAYER_SPAWN_MIN_RADIUS..=ENEMY_PLAYER_SPAWN_MAX_RADIUS)
                .contains(&dist_to_player)
            {
                position = test_position;
                break;
            }
        }

        let random_speed = rand_f32() * 0.4 + 0.1;
        let random_health = rand_f32() * 100.0 + 40.0;
        //  TODO: Optimize this line.
        let random_color =
            Color::from((rand_f32() * (Color::Orange9 as u8 + 1) as f32).floor() as u8);
        let random_dodge = if rand_f32() >= 0.8 {
            0.0
        } else {
            rand_f32() * ENEMY_MAX_DODGE
        };

        let enemy = Enemy {
            position,
            speed: random_speed,
            health: random_health,
            color: random_color,
            dodge_entropy: random_dodge,
        };
        self.enemies.push(enemy);
    }

    pub(super) fn spawn_medkit(&mut self, position: Option<Vec3>) {
        let position = position.unwrap_or_else(|| {
            let map_setting = MAP_SETTINGS[self.selected_map];

            loop {
                let position_x = rand_f32() * map_setting.bound * 2.0 - map_setting.bound;
                let position_z = rand_f32() * map_setting.bound * 2.0 - map_setting.bound;
                let position = [position_x, 0.0, position_z];
                let dist_to_player = vec_distance(self.player.position, position);
                if self.get_collision_wall(position).is_none()
                    && (MEDKIT_PLAYER_SPAWN_MIN_RADIUS..=MEDKIT_PLAYER_SPAWN_MAX_RADIUS)
                        .contains(&dist_to_player)
                {
                    return position;
                }
            }
        });
        self.medkit = Some(Medkit { position })
    }

    pub(super) fn spawn_bullet(&mut self, origin: Vec3, direction: Vec3) {
        let bullet = Bullet {
            origin,
            direction,
            position: origin,
        };
        self.bullets.push(bullet);
    }

    pub(super) fn spawn_explosion(&mut self, position: Vec3) {
        let explosion = Explosion {
            position,
            size_scalar: 0.2,
        };
        self.explosions.push(explosion);
    }
}
