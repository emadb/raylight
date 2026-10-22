use crate::params::{MOVE_SPEED, ROTATION_SPEED, VIEW_ANGLE};
use std::{f32, f64};

struct Vector {
    x: f32,
    y: f32,
}

impl Vector {
    pub fn new(x: f32, y: f32) -> Self {
        Vector { x, y }
    }
}

pub struct Player {
    pub x: f32,
    pub y: f32,
    direction: Vector,
    plane: Vector,
}

impl Player {
    pub fn new(x: u32, y: u32, direction: f64) -> Self {
        let dir_x = direction.sin();
        let dir_y = -direction.cos();

        let k = (VIEW_ANGLE / 2.0).to_radians().tan();

        let plane_x: f32 = k * -dir_y as f32;
        let plane_y: f32 = k * dir_x as f32;

        Self {
            x: x as f32 + 0.5,
            y: y as f32 + 0.5,
            direction: Vector {
                x: dir_x as f32,
                y: dir_y as f32,
            },
            plane: Vector {
                x: plane_x,
                y: plane_y,
            },
        }
    }

    pub fn dir_x(&self) -> f32 {
        self.direction.x
    }
    pub fn dir_y(&self) -> f32 {
        self.direction.y
    }

    pub fn get_plane_x(&self) -> f32 {
        self.plane.x
    }
    pub fn get_plane_y(&self) -> f32 {
        self.plane.y
    }

    pub fn rotate_right(&mut self) {
        let dir_x = self.dir_x() * ROTATION_SPEED.cos() + self.dir_y() * ROTATION_SPEED.sin();
        let dir_y = -self.dir_x() * ROTATION_SPEED.sin() + self.dir_y() * ROTATION_SPEED.cos();
        self.direction = Vector::new(dir_x, dir_y);

        let plane_x = self.plane.x * ROTATION_SPEED.cos() + self.plane.y * ROTATION_SPEED.sin();
        let plane_y = -self.plane.x * ROTATION_SPEED.sin() + self.plane.y * ROTATION_SPEED.cos();
        self.plane.x = plane_x;
        self.plane.y = plane_y;
    }

    pub fn rotate_left(&mut self) {
        let dir_x = self.dir_x() * ROTATION_SPEED.cos() - self.dir_y() * ROTATION_SPEED.sin();
        let dir_y = self.dir_x() * ROTATION_SPEED.sin() + self.dir_y() * ROTATION_SPEED.cos();
        self.direction = Vector::new(dir_x, dir_y);

        let plane_x = self.plane.x * ROTATION_SPEED.cos() - self.plane.y * ROTATION_SPEED.sin();
        let plane_y = self.plane.x * ROTATION_SPEED.sin() + self.plane.y * ROTATION_SPEED.cos();
        self.plane.x = plane_x;
        self.plane.y = plane_y;
    }

    pub fn move_up(&mut self, map: &crate::map::Map) {
        let xx = self.get_next_position_x(1.0);
        let yy = self.get_next_position_y(1.0);
        if map.get(xx.floor() as i32, self.y.floor() as i32) == 0 {
            self.move_x(1.0);
        }
        if map.get(self.x.floor() as i32, yy.floor() as i32) == 0 {
            self.move_y(1.0);
        }
    }

    pub fn move_down(&mut self, map: &crate::map::Map) {
        let xx = self.get_next_position_x(-1.0);
        let yy = self.get_next_position_y(-1.0);
        if map.get(xx.floor() as i32, self.y.floor() as i32) == 0 {
            self.move_x(-1.0);
        }
        if map.get(self.x.floor() as i32, yy.floor() as i32) == 0 {
            self.move_y(-1.0);
        }
    }

    fn move_x(&mut self, dir: f32) {
        self.x += dir * self.dir_x() * MOVE_SPEED;
    }

    fn move_y(&mut self, dir: f32) {
        self.y += dir * self.dir_y() * MOVE_SPEED;
    }

    fn get_next_position_x(&self, dir: f32) -> f32 {
        self.x + dir * self.dir_x() * MOVE_SPEED
    }
    fn get_next_position_y(&self, dir: f32) -> f32 {
        self.y + dir * self.dir_y() * MOVE_SPEED
    }
}
