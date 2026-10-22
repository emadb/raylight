use crate::{map::Map, params::WIDTH, player::Player};

pub enum Side {
    A,
    B,
}

pub struct Hit {
    pub side: Side,
    pub dist: f32,
    /// Position along the wall, [0,1).
    pub u: f32,
    /// Map cell value of the wall.
    pub cell: u8,
}

// DDA - Digital Differential Analyzer
// https://en.wikipedia.org/wiki/Digital_differential_analyzer_(graphics_algorithm)
// https://www.geeksforgeeks.org/computer-graphics/dda-line-generation-algorithm-computer-graphics/
pub fn find_wall(player: &Player, current_x: usize, map: &Map) -> Hit {
    let camera_x = 2.0 * (current_x as f32 + 0.5) / WIDTH as f32 - 1.0;
    let ray_direction_x = player.dir_x() + player.get_plane_x() * camera_x;
    let ray_direction_y = player.dir_y() + player.get_plane_y() * camera_x;

    let mut wall_found = false;
    let mut current_x = player.x.floor() as i32;
    let mut current_y = player.y.floor() as i32;

    let delta_dist_x = if ray_direction_x == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / ray_direction_x).abs()
    };
    let delta_dist_y = if ray_direction_y == 0.0 {
        f32::INFINITY
    } else {
        (1.0 / ray_direction_y).abs()
    };

    let (step_x, mut side_dist_x) = if ray_direction_x < 0.0 {
        (-1, (player.x - current_x as f32) * delta_dist_x)
    } else {
        (1, (current_x as f32 + 1.0 - player.x) * delta_dist_x)
    };

    let (step_y, mut side_dist_y) = if ray_direction_y < 0.0 {
        (-1, (player.y - current_y as f32) * delta_dist_y)
    } else {
        (1, (current_y as f32 + 1.0 - player.y) * delta_dist_y)
    };

    let mut side = Side::A;
    let mut cell = 0;
    while !wall_found {
        if side_dist_x < side_dist_y {
            side_dist_x += delta_dist_x;
            current_x += step_x;
            side = Side::A;
        } else {
            side_dist_y += delta_dist_y;
            current_y += step_y;
            side = Side::B;
        }

        cell = map.get(current_x, current_y);
        if cell != 0 {
            wall_found = true;
        }
    }

    let dist = match side {
        Side::A => side_dist_x - delta_dist_x,
        Side::B => side_dist_y - delta_dist_y,
    };

    // Where the ray hit the wall, as a fraction [0,1) along it.
    let hit = match side {
        Side::A => player.y + dist * ray_direction_y,
        Side::B => player.x + dist * ray_direction_x,
    };
    let u = hit - hit.floor();

    // Mirror so textures aren't flipped on walls seen from the far side.
    let mirrored = match side {
        Side::A => ray_direction_x > 0.0,
        Side::B => ray_direction_y < 0.0,
    };
    let u = if mirrored { 1.0 - u } else { u };

    Hit {
        side,
        dist,
        u,
        cell,
    }
}
