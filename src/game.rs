use super::*;

struct Bullet {
    origin: Vec3,
    position: Vec3,
    direction: Vec3,
}

const BULLET_DAMAGE: f32 = 40.0;
const BULLET_COOLDOWN_THRESHOLD: f32 = 0.12;
const BULLET_MAX_DISTANCE: f32 = 40.0;
const BULLET_SPEE: f32 = 2.5;

struct Player {
    position: Vec3,
    velocity: Vec3,
    direction: Vec3,
    health: f32,
    bob_tick: usize,
    yaw: f32,
    last_bullet_time: usize,
}

const PLAYER_MAX_HEALTH: f32 = 50.0;
const PLAYER_POISON_TICK: f32 = 0.01;
const PLAYER_MAX_VELOCITY: f32 = 0.65;
const PLAYER_FRICTION_SCALAR: f32 = 0.006;
const PLAYER_WALL_BOUNCE_SCALAR: f32 = 0.8;
const PLAYER_ACCELERATION: f32 = 0.13;

struct Medkit {
    position: Vec3,
}

struct MedkitHealPopup {
    x: f32,
}

struct Enemy {
    speed: f32,
    health: f32,
    color: Color,
    position: Vec3,
    dodge_entropy: f32,
}

const ENEMY_DAMAGE: f32 = 0.2;
const ENEMY_MAX_DODGE: f32 = 0.3;
const ENEMY_REACH: f32 = 1.8;
const ENEMY_SPEED_INCREMENT_SCALAR: f32 = 0.0006;
const ENEMY_PLAYER_SPAWN_MIN_RADIUS: usize = 60;
const ENEMY_PLAYER_SPAWN_MAX_RADIUS: usize = 350;

#[derive(Clone, Copy)]
struct MapSetting {
    bound: f32,
    wall_count: usize,
    wall_max_scale: usize,
}

const MAP_WALL_Y: f32 = 6.0;
const MAP_WALL_TO_WALL_MIN_DISTANCE: f32 = 20.0;
const MAP_SETTINGS: &[MapSetting] = &[
    MapSetting {
        bound: 60.0,
        wall_count: 12,
        wall_max_scale: 4,
    },
    MapSetting {
        bound: 75.0,
        wall_count: 24,
        wall_max_scale: 4,
    },
    MapSetting {
        bound: 85.0,
        wall_count: 30,
        wall_max_scale: 5,
    },
];

pub struct Wall {
    scale: Vec3,
    position: Vec3,
}

pub struct GameState {
    ticks: usize,
    player: Player,
    enemies: SmallVec<[Enemy; 64]>,
    selected_map: usize,
    walls: SmallVec<[Wall; 64]>,
    bullets: SmallVec<[Bullet; 64]>,
}

impl GameState {
    pub fn new() -> Self {
        GameState {
            ticks: 0,
            player: Player {
                position: [10.0, 0.0, 0.0],
                velocity: [0.0, 0.0, 0.0],
                direction: [1.0, 0.0, 0.0],
                health: PLAYER_MAX_HEALTH,
                bob_tick: 0,
                yaw: 0.0,
                last_bullet_time: 0,
            },
            enemies: smallvec![],
            selected_map: 0,
            walls: smallvec![],
            bullets: smallvec![],
        }
    }

    pub fn init(&mut self) {
        self.spawn_walls();
    }

    pub fn update(&mut self, buttons: Buttons) {
        self.ticks += 1;

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
        if buttons.contains(Buttons::J) {
            self.player.yaw += core::f32::consts::PI / 30.0;
        }
        if buttons.contains(Buttons::L) {
            self.player.yaw -= core::f32::consts::PI / 30.0;
        }

        self.tick_player();
    }

    fn player_bob(&mut self) {}

    pub fn render(&self, fb: &mut Framebuffer) {
        fb.clear_color(Color::Gray2);
        fb.clear_depth(core::f32::MAX);

        let camera_position = self.player.position;
        let camera_front = vec_normalize(self.player.direction);

        let projection = ProjectionData {
            fov_rad: core::f32::consts::FRAC_PI_2,
            near: 0.1,
            far: 50.0,
        };

        for enemy in &self.enemies {
            let mv = mat4_identity();
            let mv = mat4_scale(mv, [1.0, 1.0, 1.0]);
            let mv = mat4_rotate(mv, self.ticks as f32 / 10.0, [0.0, 1.0, 0.0]);
            let mv = mat4_translate(mv, enemy.position);

            fb.render_pass(&RenderPass {
                camera_front,
                camera_position,
                triangles: models::cube(),
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
            //  BAM. Attacked by player.
            //  todo!()

            enemy.health > 0.0
        });
    }

    fn tick_bullets(&mut self) {}

    fn tick_medkit(&mut self) {}

    fn tick_explosions(&mut self) {}

    fn tick_poison(&mut self) {}

    fn tick_medkit_heal_popup(&mut self) {}

    pub fn spawn_walls(&mut self) {
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
            let scale_x = rand::random::<f32>() * settings.wall_max_scale as f32 + 3.0;
            let scale_z = rand::random::<f32>() * settings.wall_max_scale as f32 + 3.0;
            let position_x = rand::random::<f32>() * (settings.bound - 2.0) * 2.0 - settings.bound;
            let position_z = rand::random::<f32>() * (settings.bound - 2.0) * 2.0 - settings.bound;

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

    pub fn spawn_enemy(&mut self) {
        //  Do everything else.
        //  todo!()

        let enemy = Enemy {
            speed: 0.5,
            health: 100.0,
            color: Color::Green4,
            position: [0.0, 0.0, 0.0],
            dodge_entropy: 1.0,
        };
        self.enemies.push(enemy);
    }
}
