use super::*;

mod collision;
mod input;
mod render;
mod spawn;
mod tick;

#[derive(Clone, Copy)]
struct Bullet {
    origin: Vec3,
    position: Vec3,
    direction: Vec3,
}

const BULLET_DAMAGE: f32 = 40.0;
const BULLET_HITBOX_RADIUS: f32 = 2.5;
const BULLET_COOLDOWN_THRESHOLD: usize = 12;
const BULLET_MAX_DISTANCE: f32 = 40.0;
const BULLET_SPEED: f32 = 0.5;

#[derive(Clone, Copy)]
struct Explosion {
    position: Vec3,
    size_scalar: f32,
}

const EXPLOSION_MAX_SIZE: f32 = 2.0;
const EXPLOSION_GROWTH_INCREMENT: f32 = 0.05;

#[derive(Clone, Copy)]
struct Player {
    dead: bool,
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
const PLAYER_MAX_VELOCITY: f32 = 0.6;
const PLAYER_FRICTION_SCALAR: f32 = 0.01;
const PLAYER_WALL_BOUNCE_SCALAR: f32 = 0.8;
const PLAYER_ACCELERATION: f32 = 0.1;

#[derive(Clone, Copy)]
struct Medkit {
    position: Vec3,
}

const MEDKIT_HEAL_AMOUNT: f32 = 18.0;
const MEDKIT_PLAYER_SPAWN_MIN_RADIUS: f32 = 40.0;
const MEDKIT_PLAYER_SPAWN_MAX_RADIUS: f32 = 200.0;
const MEDKIT_PICKUP_RANGE: f32 = 5.0;

#[derive(Clone, Copy)]
struct MedkitHealPopup {
    y: f32,
}

const MEDKIT_POPUP_RISE_SPEED: f32 = 0.3;
const MEDKIT_POPUP_MAX_HEIGHT: f32 = 4.0;

#[derive(Clone, Copy)]
struct Enemy {
    speed: f32,
    health: f32,
    color: Color,
    position: Vec3,
    dodge_entropy: f32,
}

const ENEMY_DAMAGE: f32 = 0.2;
const ENEMY_MAX_DODGE: f32 = 0.3;
const ENEMY_REACH: f32 = 3.0;
const ENEMY_SPEED_INCREMENT_SCALAR: f32 = 0.0006;
const ENEMY_PLAYER_SPAWN_MIN_RADIUS: f32 = 60.0;
const ENEMY_PLAYER_SPAWN_MAX_RADIUS: f32 = 350.0;

#[derive(Clone, Copy)]
struct MapSetting {
    bound: f32,
    wall_count: usize,
    wall_max_scale: usize,
}

const MAP_WALL_Y: f32 = 6.0;
const MAP_WALL_TO_WALL_MIN_DISTANCE: f32 = 20.0;
const MAP_SETTINGS: &[MapSetting] = &[MapSetting {
    bound: 85.0,
    wall_count: 30,
    wall_max_scale: 6,
}];

#[derive(Clone, Copy)]
pub struct Wall {
    scale: Vec3,
    position: Vec3,
}

const ENEMY_CAP_STAGES: &[usize] = &[3, 5, 8, 10, 15, 18, 50];
const ENEMY_SPAWN_FREQUENCY_PARAM: usize = 3;
const KILL_SCREEN_STAGE: usize = ENEMY_CAP_STAGES.len() - 1;

fn get_stage_number(ticks: usize) -> usize {
    //  todo!()
    0
}

pub struct GamePlayState {
    ticks: usize,
    player: Player,
    medkit: Option<Medkit>,
    enemies: SmallVec<[Enemy; 64]>,
    selected_map: usize,
    walls: SmallVec<[Wall; 64]>,
    bullets: SmallVec<[Bullet; 16]>,
    explosions: SmallVec<[Explosion; 8]>,
}

impl GamePlayState {
    pub fn new() -> Self {
        GamePlayState {
            ticks: 0,
            player: Player {
                dead: false,
                position: [0.0, 0.0, 0.0],
                velocity: [0.0, 0.0, 0.0],
                direction: [1.0, 0.0, 0.0],
                health: PLAYER_MAX_HEALTH,
                bob_tick: 0,
                yaw: 0.0,
                last_bullet_time: 0,
            },
            medkit: None,
            enemies: smallvec![],
            selected_map: 0,
            walls: smallvec![],
            bullets: smallvec![],
            explosions: smallvec![],
        }
    }

    pub fn init(&mut self) {
        self.spawn_walls();
        self.spawn_medkit(Some([6.0, 0.0, 0.0]));
    }
}

use core::cell::UnsafeCell;
use nanorand::{Rng, WyRand};

static mut RAND: UnsafeCell<Option<WyRand>> = UnsafeCell::new(None);

fn rand_f32() -> f32 {
    let rand = unsafe { &mut *RAND.get() };
    if rand.is_none() {
        *rand = Some(WyRand::new_seed(9999999999));
    }
    let rand = rand.as_mut().unwrap();
    rand.generate::<f32>()
}
