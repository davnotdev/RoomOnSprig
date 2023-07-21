use super::*;

impl GamePlayState {
    //  Check if hit by bullet.
    //  Destroy the bullet if so.
    pub(super) fn hit_by_bullet(bullets: &mut SmallVec<[Bullet; 16]>, agent: Vec3) -> bool {
        let mut was_hit = false;
        bullets.retain(|bullet| {
            if vec_distance(bullet.position, agent) < BULLET_HITBOX_RADIUS {
                was_hit = true;
                false
            } else {
                true
            }
        });
        was_hit
    }

    pub(super) fn get_collision_wall(walls: &SmallVec<[Wall; 64]>, position: Vec3) -> Option<Wall> {
        walls
            .iter()
            .find(|wall| {
                let box_min_x =
                    (wall.position[0] - wall.scale[0]).min(wall.position[0] + wall.scale[0]);
                let box_max_x =
                    (wall.position[0] - wall.scale[0]).max(wall.position[0] + wall.scale[0]);
                let box_min_z =
                    (wall.position[2] - wall.scale[2]).min(wall.position[2] + wall.scale[2]);
                let box_max_z =
                    (wall.position[2] - wall.scale[2]).max(wall.position[2] + wall.scale[2]);

                position[0] >= box_min_x
                    && position[0] <= box_max_x
                    && position[2] >= box_min_z
                    && position[2] <= box_max_z
            })
            .copied()
    }
}
